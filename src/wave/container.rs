use std::io::{Read, Seek, SeekFrom, Write};
use buffer::*;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt, LittleEndian};
use codecs::{Endian, Codec, AudioCodec, LPCM, SampleFormat};
use traits::{Container, Chunk};
use wave::chunks::*;
use error::*;

/// WAVE chunk identifiers
const RIFF: &'static [u8; 4] = b"RIFF";
const WAVE: &'static [u8; 4] = b"WAVE";
const FMT:  &'static [u8; 4] = b"fmt ";
const DATA: &'static [u8; 4] = b"data";

/// Struct containing all necessary information
/// for encoding and decoding bytes to an `AudioBuffer`
pub struct WaveContainer {
  compression:      CompressionType,
  sample_format:    SampleFormat,
  pub bit_rate:     u32,
  pub sample_rate:  u32,
  pub channels:     u32,
  pub block_size:   u32,
  pub order:        SampleOrder,
  pub bytes:        Vec<u8>
}

impl Container for WaveContainer {
  /// Reads the bytes provided from the reader.
  /// This is where the reading of chunks ocurrs.
  fn open<R: Read + Seek>(reader: &mut R) -> AudioResult<WaveContainer> {
    let header: &mut[u8] = &mut[0u8; 12];
    try!(reader.read(header));
    if &header[0..4]  != RIFF
    || &header[8..12] != WAVE {
      return Err(AudioError::FormatError(
        "Not valid WAVE".to_string()
      ));
    }
    let     file_size : usize = LittleEndian::read_u32(&header[4..8]) as usize;
    let mut pos       : i64   = 12i64;
    let mut container =
      WaveContainer {
        compression:  CompressionType::PCM,
        sample_format: SampleFormat::Signed16,
        bit_rate:     0u32,
        sample_rate:  0u32,
        channels:     0u32,
        block_size:   0u32,
        order:        SampleOrder::MONO,
        bytes:        Vec::new()
      };
    let mut fmt_chunk_read  = false;
    let mut data_chunk_read = false;
    let mut chunk_size;
    let mut chunk_buffer;
    while pos < file_size as i64 {
      pos += 4i64;
      match identify(reader).ok() {
        Some(WaveChunk::Format) => {
          chunk_size    = try!(reader.read_u32::<LittleEndian>());
          chunk_buffer  = vec![0u8; chunk_size as usize];
          try!(reader.read(&mut chunk_buffer));
          let chunk     = try!(FormatChunk::read(&chunk_buffer));
          container.compression     = chunk.compression_type;
          container.bit_rate        = chunk.bit_rate      as u32;
          container.sample_rate     = chunk.sample_rate;
          container.channels        = chunk.num_channels  as u32;
          container.block_size      = chunk.block_size    as u32;
          fmt_chunk_read            = true;
          pos                      += chunk_size          as i64;
        },
        Some(WaveChunk::Data) => {
          if !fmt_chunk_read {
            return Err(AudioError::FormatError(
              "File is not valid WAVE \
              (Format chunk does not occur before Data chunk)".to_string()
            ))
          }
          chunk_size      = try!(reader.read_u32::<LittleEndian>());
          container.bytes = vec![0u8; chunk_size as usize];
          try!(reader.read(&mut container.bytes));
          data_chunk_read = true;
          pos            += chunk_size as i64;
        },
        None => {
          chunk_size  = try!(reader.read_u32::<LittleEndian>());
          pos        += chunk_size as i64;
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
    container.order =
      if container.channels == 1u32 {
        SampleOrder::MONO
      } else {
        SampleOrder::INTERLEAVED
      };
    container.sample_format =
      match container.bit_rate {
        8  => SampleFormat::Unsigned8,
        16 => SampleFormat::Signed16,
        24 => SampleFormat::Signed24,
        32 => SampleFormat::Signed32,
        _ =>
          return Err(AudioError::FormatError(
            "Audio encoded with invalid sample format".to_string()
          ))
      };
    Ok(container)
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
      Codec::LPCM => LPCM::read(&mut self.bytes, Endian::LittleEndian, &self.bit_rate, &self.channels)
    }
  }

  fn create(codec: Codec, audio: &AudioBuffer) -> AudioResult<Vec<u8>> {
    // Allow use of SampleFormat
    // Must check for SampleFormat unsupported by wave
    match audio.order {
      SampleOrder::MONO         => {},
      SampleOrder::INTERLEAVED  => {},
      _     => 
        return Err(AudioError::UnsupportedError(
          "Multi-channel audio must be interleaved in RIFF containers".to_string()
        ))
    }
    let header_size     : u32     = 44; // Num bytes before audio samples. Always write 44 bytes
    let fmt_chunk_size  : u32     = 16;
    let total_bytes     : u32     = 12
                                  + (8 + fmt_chunk_size)
                                  + (8 + (audio.samples.len() as u32 * audio.bit_rate / 8));
    let data            : Vec<u8> =
      match codec {
        Codec::LPCM => try!(LPCM::create(audio, Endian::LittleEndian)),
      };
    debug_assert_eq!(total_bytes, header_size + audio.samples.len() as u32 * audio.bit_rate / 8);
    let mut buffer      : Vec<u8> = Vec::with_capacity(total_bytes as usize);
    try!(buffer.write(RIFF));
    try!(buffer.write_u32::<LittleEndian>(total_bytes - 8));
    try!(buffer.write(WAVE));
    try!(buffer.write(FMT));
    try!(buffer.write_u32::<LittleEndian>(fmt_chunk_size));
    try!(buffer.write_u16::<LittleEndian>(1u16)); // Always LPCM
    try!(buffer.write_u16::<LittleEndian>(audio.channels as u16));
    try!(buffer.write_u32::<LittleEndian>(audio.sample_rate as u32));
    try!(buffer.write_u32::<LittleEndian>(audio.sample_rate * audio.channels * audio.bit_rate / 8u32));
    try!(buffer.write_u16::<LittleEndian>((audio.channels * audio.bit_rate / 8u32) as u16));
    try!(buffer.write_u16::<LittleEndian>(audio.bit_rate as u16));
    try!(buffer.write(DATA));
    try!(buffer.write_u32::<LittleEndian>((audio.samples.len() * ((audio.bit_rate) as usize / 8)) as u32));
    try!(buffer.write(&data));
    debug_assert_eq!(total_bytes as usize, buffer.len());
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
    err @ _ => 
      Err(AudioError::FormatError(
        format!("Do not recognize RIFF chunk with identifier {:?}", err)
      ))
  }
}