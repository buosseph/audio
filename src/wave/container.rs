use std::io::{Read, Seek, SeekFrom};
use buffer::*;
use byteorder::{ByteOrder, ReadBytesExt, BigEndian, LittleEndian};
use codecs::{Codec, AudioCodec, LPCM};
use traits::{Container, Chunk};
use wave::chunks::{
  CompressionType,
  WaveChunk,
  FormatChunk,
  DataChunk
};
use error::*;

/// WAVE chunk FourCCs for identification
const RIFF: u32 = 0x52494646;
const WAVE: u32 = 0x57415645;
const FMT:  u32 = 0x666D7420;
const DATA: u32 = 0x64617461;
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
  fn open<R: Read + Seek>(r: &mut R) -> AudioResult<WaveContainer> {
    let header: &mut[u8] = &mut[0u8; 12];
    try!(r.read(header));
    if BigEndian::read_u32(&header[0..4])  != RIFF
    || BigEndian::read_u32(&header[8..12]) != WAVE {
      return Err(AudioError::FormatError("Not valid WAVE".to_string()));
    }
    let file_size = LittleEndian::read_u32(&header[4..8]) as usize;
    let mut pos: i64 = 12i64;
    let mut compression: CompressionType = CompressionType::PCM;
    let mut bit_rate    : u32 = 0u32;
    let mut sample_rate : u32 = 0u32;
    let mut num_channels: u32 = 0u32;
    let mut block_size  : u32 = 0u32;
    let mut bytes: Vec<u8> = Vec::new();
    let mut fmt_chunk_read = false;
    let mut data_chunk_read = false;
    while pos < file_size as i64 {
      pos += 4i64;
      match identify(r).ok() {
        Some(WaveChunk::Format) => {
          let chunk = try!(FormatChunk::read(r));
          compression = chunk.compression_type;
          bit_rate = chunk.bit_rate as u32;
          sample_rate = chunk.sample_rate;
          num_channels = chunk.num_of_channels as u32;
          block_size = chunk.block_size as u32;
          fmt_chunk_read = true;
          pos += chunk.size as i64;
        },
        Some(WaveChunk::Data) => {
          if !fmt_chunk_read {
            return Err(AudioError::FormatError("File is not valid WAVE (Format chunk does not occur before Data chunk)".to_string()))
          }
          // Rearrange all samples so that it's in big endian
          // NOTE: Consider passing byteorder to codec to avoid reversing
          let mut chunk = try!(DataChunk::read(r));
          let sample_size = bit_rate as usize / 8;
          if sample_size != 1 {
            for sample_bytes in chunk.bytes.chunks_mut(sample_size) {
              sample_bytes.reverse();
            }
          }
          bytes = chunk.bytes.to_vec();
          data_chunk_read = true;
          pos += chunk.size as i64;
        },
        None => {
          let size = try!(r.read_u32::<LittleEndian>());
          pos += size as i64;
          let new_pos = r.seek(SeekFrom::Current(pos)).ok().expect("Error while seeking in reader");
          if new_pos > file_size as u64 {
            return Err(AudioError::FormatError("Some chunk trying to read past end of file".to_string()));
          }
        }
      }
    }
    if !fmt_chunk_read {
      return Err(AudioError::FormatError("File is not valid WAVE (Missing required Format chunk)".to_string()))
    }
    else if !data_chunk_read {
      return Err(AudioError::FormatError("File is not valid WAVE (Missing required Data chunk)".to_string()))
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

  fn read_codec(&mut self) -> AudioResult<Vec<Sample>> {
    let codec = match self.compression {
      CompressionType::PCM => Codec::LPCM,
      _ => return Err(AudioError::UnsupportedError("This file uses an unsupported codec".to_string()))
    };
    match codec {
      Codec::LPCM => LPCM::read(&mut self.bytes, &self.bit_rate, &self.channels)
    }
  }

  /// Writes the provided audio into a valid WaveContainer
  /// give the codec specified is supported by the format.
  ///
  /// Currently the creation of a WaveContainer only
  /// supports the writing of 16-bit audio using LPCM
  #[allow(unused_assignments)]
  fn create(codec: Codec, audio: &AudioBuffer) -> AudioResult<Vec<u8>> {
    /*
     *  TODO: Dealing with bit rates not supported by format.
     *
     *  Audio can be manipulated to be of any bit rate, but codecs don't
     *  support that. It's preferred that the fields in AudioBuffer
     *  reflect the samples stored, so values need to be checked when
     *  encoding. For bit rate, just round up to nearest multiple of 8
     *  that's less than the highest supported bit rate and use that
     *  value for the encoding bit rate.
     *
     *  For now, assume provided bit_rate is a multiple of 8
     */
    match audio.order {
      SampleOrder::MONO    => {},
      SampleOrder::INTERLEAVED => {},
      _     => return Err(AudioError::UnsupportedError("Multi-channel audio must be interleaved in RIFF containers".to_string()))
    }
    let num_of_channels : u16   = audio.channels as u16;
    let sample_rate     : u32   = audio.sample_rate as u32;
    let bit_rate        : u16   = audio.bit_rate as u16;
    let mut data: Vec<u8> = match codec {
      Codec::LPCM => try!(LPCM::create(audio)),
    };
    // Rearrange all samples to little endian
    let sample_size = bit_rate as usize / 8;
    if sample_size != 1 {
      for sample_bytes in data.chunks_mut(sample_size) {
        sample_bytes.reverse();
      }
    }
    let data_rate       : u32   = (audio.sample_rate * audio.channels * audio.bit_rate / 8) as u32;
    let block_size      : usize = (audio.channels * audio.bit_rate) as usize / 8;
    let fmt_chunk_size  : u32   = 16;
    let num_of_frames   : usize = audio.samples.len() / audio.channels as usize;
    let data_size       : u32   = (num_of_frames * block_size) as u32;
    let total_bytes     : usize = 44 + data_size as usize; // Always write 44 byte header
    let file_size       : u32   =  (total_bytes - 8) as u32;
      // = 4 + (8 + fmt_chunk size) + (8 + (data_chunk size * block_size)) (NOTE: 8 bytes are purposely missing for riff_header and file_size)
      // = 4 + (WAVE chunks) = total_bytes - 8 (exclude first 8 bytes)
    let mut buffer      : Vec<u8>   = Vec::with_capacity(total_bytes);
    for _ in 0..buffer.capacity() { buffer.push(0u8); }
    debug_assert_eq!(buffer.capacity(), buffer.len());
    BigEndian::write_u32(&mut buffer[0..4], RIFF);
    LittleEndian::write_u32(&mut buffer[4..8], file_size);
    BigEndian::write_u32(&mut buffer[8..12], WAVE);
    BigEndian::write_u32(&mut buffer[12..16], FMT);
    LittleEndian::write_u32(&mut buffer[16..20], fmt_chunk_size);
    LittleEndian::write_u16(&mut buffer[20..22], 1u16); // Always encode as PCM
    LittleEndian::write_u16(&mut buffer[22..24], num_of_channels);
    LittleEndian::write_u32(&mut buffer[24..28], sample_rate);
    LittleEndian::write_u32(&mut buffer[28..32], data_rate);
    LittleEndian::write_u16(&mut buffer[32..34], block_size as u16);
    LittleEndian::write_u16(&mut buffer[34..36], bit_rate);
    BigEndian::write_u32(&mut buffer[36..40], DATA);
    LittleEndian::write_u32(&mut buffer[40..44], data_size);
    let mut i = 44; // Because we're only writing WAVs with 44 byte headers
    for byte in data.iter() {
      buffer[i] = *byte;
      i += 1;
    }
    Ok(buffer)
  }
}

/// This function reads the four byte identifier for each RIFF chunk
///
/// If an unsupported chunk is found instead, the identifier bytes are lost
/// and makes reading the remainder of the file for chunks impossible without
/// skipping the length of the chunk indicated by the next four bytes available
/// in the reader.
fn identify<R: Read + Seek>(r: &mut R) -> AudioResult<WaveChunk> {
  match try!(r.read_u32::<BigEndian>()) {
    FMT  => Ok(WaveChunk::Format),
    DATA => Ok(WaveChunk::Data),
    //FACT => Ok(WaveChunk::Fact),
    err @ _ => Err(AudioError::FormatError(format!("Do not recognize RIFF chunk with identifier 0x{:x}", err)))
  }
}