use std::io::{Read, Seek, SeekFrom};
use buffer::*;
use byteorder::{ByteOrder, ReadBytesExt, LittleEndian};
use codecs::{Endian, Codec, AudioCodec, LPCM};
use traits::{Container, Chunk};
use wave::chunks::{
  CompressionType,
  WaveChunk,
  FormatChunk
};
use error::*;

/// WAVE chunk FourCCs for identification
const RIFF: &'static [u8; 4] = b"RIFF";
const WAVE: &'static [u8; 4] = b"WAVE";
const FMT:  &'static [u8; 4] = b"fmt ";
const DATA: &'static [u8; 4] = b"data";
//const FACT: u32 = 0x66616374;

/// Struct containing all necessary information
/// for encoding and decoding bytes to an `AudioBuffer`
pub struct WaveContainer {
  compression: CompressionType,
  pub bit_rate: u32,
  pub sample_rate: u32,
  pub channels: u32,
  pub block_size: u32,
  pub order: SampleOrder,
  pub bytes: Vec<u8>
}

impl Container for WaveContainer {
  /// Reads the bytes provided from the reader.
  /// This is where the reading of chunks ocurrs.
  fn open<R: Read + Seek>(reader: &mut R) -> AudioResult<WaveContainer> {
    let header: &mut[u8] = &mut[0u8; 12];
    try!(reader.read(header));
    if &header[0..4]  != RIFF
    || &header[8..12] != WAVE {
      return Err(AudioError::FormatError("Not valid WAVE".to_string()));
    }
    let     file_size   : usize   = LittleEndian::read_u32(&header[4..8]) as usize;
    let mut pos         : i64     = 12i64;
    let mut compression           = CompressionType::PCM;
    let mut bit_rate    : u32     = 0u32;
    let mut sample_rate : u32     = 0u32;
    let mut num_channels: u32     = 0u32;
    let mut block_size  : u32     = 0u32;
    let mut bytes       : Vec<u8> = Vec::new();
    let mut fmt_chunk_read        = false;
    let mut data_chunk_read       = false;
    let mut chunk_size;
    let mut chunk_buffer;
    while pos < file_size as i64 {
      pos += 4i64;
      match identify(reader).ok() {
        Some(WaveChunk::Format) => {
          chunk_size      = try!(reader.read_u32::<LittleEndian>());
          chunk_buffer    = vec![0u8; chunk_size as usize];
          try!(reader.read(&mut chunk_buffer));
          let chunk       = try!(FormatChunk::read(&chunk_buffer));
          compression     = chunk.compression_type;
          bit_rate        = chunk.bit_rate        as u32;
          sample_rate     = chunk.sample_rate;
          num_channels    = chunk.num_of_channels as u32;
          block_size      = chunk.block_size      as u32;
          fmt_chunk_read  = true;
          pos            += chunk_size            as i64;
        },
        Some(WaveChunk::Data) => {
          if !fmt_chunk_read {
            return Err(AudioError::FormatError(
              "File is not valid WAVE \
              (Format chunk does not occur before Data chunk)".to_string()
            ))
          }
          chunk_size      = try!(reader.read_u32::<LittleEndian>());
          bytes           = vec![0u8; chunk_size as usize];
          try!(reader.read(&mut bytes));
          data_chunk_read = true;
          pos            += chunk_size as i64;
        },
        None => {
          let size = try!(reader.read_u32::<LittleEndian>());
          pos += size as i64;
          let new_pos = reader.seek(SeekFrom::Current(pos))
            .ok().expect("Error while seeking in reader");
          if new_pos > file_size as u64 {
            return Err(AudioError::FormatError(
              "Some chunk trying to read past end of file".to_string()
            ));
          }
        }
      }
    }
    if !fmt_chunk_read {
      return Err(AudioError::FormatError(
        "File is not valid WAVE (Missing required Format chunk)".to_string()
      ))
    }
    else if !data_chunk_read {
      return Err(AudioError::FormatError(
        "File is not valid WAVE (Missing required Data chunk)".to_string()
      ))
    }
    let sample_order =
      if num_channels == 1u32 {
        SampleOrder::MONO
      } else {
        SampleOrder::INTERLEAVED
      };
    Ok(WaveContainer {
      compression:  compression,
      bit_rate:     bit_rate,
      sample_rate:  sample_rate,
      channels:     num_channels,
      block_size:   block_size,
      order:        sample_order,
      bytes:        bytes
    })
  }

  #[inline]
  fn read_codec(&mut self) -> AudioResult<Vec<Sample>> {
    let codec = match self.compression {
      CompressionType::PCM => Codec::LPCM,
      _ =>
        return Err(AudioError::UnsupportedError(
          "This file uses an unsupported codec".to_string()
        ))
    };
    match codec {
      // Change endian to LittleEndian
      Codec::LPCM => LPCM::read(&mut self.bytes, Endian::LittleEndian, &self.bit_rate, &self.channels)
    }
  }

  /// Writes the provided audio into a valid WaveContainer
  /// give the codec specified is supported by the format.
  #[allow(unused_assignments)]
  fn create(codec: Codec, audio: &AudioBuffer) -> AudioResult<Vec<u8>> {
    match audio.order {
      SampleOrder::MONO    => {},
      SampleOrder::INTERLEAVED => {},
      _     => return Err(AudioError::UnsupportedError("Multi-channel audio must be interleaved in RIFF containers".to_string()))
    }
    let header_size     : usize = 44; // Not really the header, but all data before audio samples
    let num_of_channels : u16   = audio.channels as u16;
    let sample_rate     : u32   = audio.sample_rate as u32;
    let bit_rate        : u16   = audio.bit_rate as u16;
    let data: Vec<u8> = match codec {
      Codec::LPCM => try!(LPCM::create(audio, Endian::LittleEndian)),
    };
    let data_rate       : u32   = (audio.sample_rate * audio.channels * audio.bit_rate / 8) as u32;
    let block_size      : usize = (audio.channels * audio.bit_rate) as usize / 8;
    let fmt_chunk_size  : u32   = 16;
    let num_of_frames   : usize = audio.samples.len() / audio.channels as usize;
    let data_size       : u32   = (num_of_frames * block_size) as u32;
    let total_bytes     : usize = header_size + data_size as usize; // Always write 44 byte header
    let file_size       : u32   =  (total_bytes - 8) as u32;
      // = 4 + (8 + fmt_chunk size) + (8 + (data_chunk size * block_size)) (NOTE: 8 bytes are purposely missing for riff_header and file_size)
      // = 4 + (WAVE chunks) = total_bytes - 8 (exclude first 8 bytes)
    let mut buffer      : Vec<u8>   = Vec::with_capacity(total_bytes);
    for byte in RIFF.iter() { buffer.push(*byte) }
    for i in 0..4 { buffer.push(( file_size         .swap_bytes() >> 8 * (3 - i)) as u8) }
    for byte in WAVE.iter() { buffer.push(*byte) }
    for byte in  FMT.iter() { buffer.push(*byte) }
    for i in 0..4 { buffer.push(( fmt_chunk_size    .swap_bytes() >> 8 * (3 - i)) as u8) }
    for i in 0..2 { buffer.push(( 1u16              .swap_bytes() >> 8 * (1 - i)) as u8) } // Always encode as PCM
    for i in 0..2 { buffer.push(( num_of_channels   .swap_bytes() >> 8 * (1 - i)) as u8) }
    for i in 0..4 { buffer.push(( sample_rate       .swap_bytes() >> 8 * (3 - i)) as u8) }
    for i in 0..4 { buffer.push(( data_rate         .swap_bytes() >> 8 * (3 - i)) as u8) }
    for i in 0..2 { buffer.push(((block_size as u16).swap_bytes() >> 8 * (1 - i)) as u8) }
    for i in 0..2 { buffer.push(( bit_rate          .swap_bytes() >> 8 * (1 - i)) as u8) }
    for byte in DATA.iter() { buffer.push(*byte) }
    for i in 0..4 { buffer.push(( data_size         .swap_bytes() >> 8 * (3 - i)) as u8) }
    for byte in data.iter() { buffer.push(*byte) }
    debug_assert_eq!(total_bytes, buffer.len());
    Ok(buffer)
  }
}

/// This function reads the four byte identifier for each RIFF chunk
///
/// If an unsupported chunk is found instead, the identifier bytes are lost
/// and makes reading the remainder of the file for chunks impossible without
/// skipping the length of the chunk indicated by the next four bytes available
/// in the reader.
#[inline]
fn identify<R: Read + Seek>(reader: &mut R) -> AudioResult<WaveChunk> {
  let mut buffer = [0u8; 4];
  try!(reader.read(&mut buffer));
  match &buffer {
    FMT  => Ok(WaveChunk::Format),
    DATA => Ok(WaveChunk::Data),
    //FACT => Ok(WaveChunk::Fact),
    err @ _ => 
      Err(AudioError::FormatError(
        format!("Do not recognize RIFF chunk with identifier {:?}", err)
      ))
  }
}