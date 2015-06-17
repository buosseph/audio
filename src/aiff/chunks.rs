// Unstable

const FORM: i32 = 0x464F524D;
const AIFF: i32 = 0x41494646;
const COMM: i32 = 0x434F4D4D;
const SSND: i32 = 0x53534E44;

use std::fmt;
use std::io::{Read, Seek, SeekFrom};
use buffer::*;
use byteorder::{ByteOrder, ReadBytesExt, BigEndian};
use codecs::{Codec, AudioCodec, LPCM};
use traits::{Container, Chunk};
use error::*;

pub struct AiffContainer {
  compression: CompressionType,
  pub bit_rate: u32,
  pub sample_rate: u32,
  pub channels: u32,
  pub num_frames: u32,
  pub order: SampleOrder,
  pub bytes: Vec<u8>
}

impl Container for AiffContainer {
  /// Reads the bytes provided from the reader.
  /// This is where the reading of chunks occurs.
  fn open<R: Read + Seek>(r: &mut R) -> AudioResult<AiffContainer> {
    let header: &mut[u8] = &mut[0u8; 12];
    try!(r.read(header));
    if BigEndian::read_i32(&header[0..4])  != FORM
    || BigEndian::read_i32(&header[8..12]) != AIFF {
      return Err(AudioError::FormatError("Not valid AIFF".to_string()));
    }
    let file_size = BigEndian::read_i32(&header[4..8]) as usize;
    let mut pos: i64 = 12i64;
    let mut compression: CompressionType = CompressionType::PCM;
    let mut bit_rate    : u32 = 0u32;
    let mut sample_rate : u32 = 0u32;
    let mut num_channels: u32 = 0u32;
    let mut num_frames  : u32 = 0u32;
    let mut bytes: Vec<u8> = Vec::new();
    let mut comm_chunk_read = false;
    let mut ssnd_chunk_read = false;
    while pos < file_size as i64 {
      pos += 4i64;
      match identify(r).ok() {
        Some(AiffChunk::Common) => {
          let chunk = try!(CommonChunk::read(r));
          compression = CompressionType::PCM; // only option in AIFF, not AIFC
          bit_rate = chunk.bit_rate as u32;
          sample_rate = chunk.sample_rate as u32;
          num_channels = chunk.num_channels as u32;
          num_frames = chunk.num_sample_frames;
          comm_chunk_read = true;
          pos += chunk.size as i64;
        },
        Some(AiffChunk::SoundData) => {
          if !comm_chunk_read {
            return Err(AudioError::FormatError(
              "File is not valid AIFF \
              (Common chunk does not occur before SoundData chunk)".to_string()
            ))
          }
          let chunk = try!(SoundDataChunk::read(r));
          bytes = chunk.data.to_vec();
          ssnd_chunk_read = true;
          pos += chunk.size as i64;
        },
        None => {
          let size = try!(r.read_i32::<BigEndian>());
          pos += size as i64;
          let new_pos = r.seek(SeekFrom::Current(pos)).ok().expect("Error while seeking in reader");
          if new_pos > file_size as u64 {
            return Err(AudioError::FormatError("Some chunk trying to read past end of file".to_string()));
          }
        }
      }
    }
    if !comm_chunk_read {
      return Err(AudioError::FormatError(
        "File is not valid AIFF \
        (Missing required Common chunk)".to_string()
      ))
    }
    else if !ssnd_chunk_read {
      return Err(AudioError::FormatError(
        "File is not valid AIFF \
        (Missing required SoundData chunk)".to_string()
      ))
    }
    let sample_order =
      if num_channels == 1u32 {
        SampleOrder::MONO
      } else {
        SampleOrder::INTERLEAVED
      };
    Ok(AiffContainer {
      compression:  compression,
      bit_rate:     bit_rate,
      sample_rate:  sample_rate,
      channels:     num_channels,
      num_frames:   num_frames,
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
    Err(AudioError::UnsupportedError("This feature is not yet complete".to_string()))
  }
}

/// This function reads the four character code for each AIFF chunk
///
/// If an unsupported chunk is found instead, the bytes are consumed,
/// which makes reading the remainder of the file for chunks impossible without
/// skipping the length of the chunk indicated by the next four bytes available
/// in the reader.
fn identify<R: Read + Seek>(r: &mut R) -> AudioResult<AiffChunk> {
  match try!(r.read_i32::<BigEndian>()) {
    COMM => Ok(AiffChunk::Common),
    SSND => Ok(AiffChunk::SoundData),
    err @ _ => Err(AudioError::FormatError(format!("Do not recognize AIFF chunk with identifier 0x{:x}", err)))
  }
}

/// Enumeration of supported WAVE chunks
enum AiffChunk {
  Common,
  SoundData
}

/// Enumeration of supported compression codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompressionType {
  Unknown = 0,
  PCM     = 1
}

impl fmt::Display for CompressionType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(fmt, "{}", self)
  }
}

#[derive(Debug, Clone, Copy)]
struct CommonChunk {
  size: i32,
  num_channels: i16,
  num_sample_frames: u32,
  bit_rate: i16,
  sample_rate: f64
}

impl Chunk for CommonChunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<CommonChunk> {
    let size :i32 = try!(r.read_i32::<BigEndian>());
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    for _ in 0..buffer.capacity() { buffer.push(0u8); }
    try!(r.read(&mut buffer));
    let num_channels      : i16 = BigEndian::read_i16(&buffer[0..2]);
    let num_sample_frames : u32 = BigEndian::read_u32(&buffer[2..6]);
    let bit_rate          : i16 = BigEndian::read_i16(&buffer[6..8]);
    let extended          : &[u8] = &buffer[8..18];
    let sample_rate       : f64 = convert_from_ieee_extended(extended);
    Ok(
      CommonChunk {
        size:               size,
        num_channels:       num_channels,
        num_sample_frames:  num_sample_frames,
        bit_rate:           bit_rate,
        sample_rate:        sample_rate
      }
    )
  }
}

#[allow(dead_code)]
struct SoundDataChunk {
  size: i32,
  offset: u32,
  block_size: u32,
  data: Vec<u8>
}

impl Chunk for SoundDataChunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<SoundDataChunk> {
    let size :i32 = try!(r.read_i32::<BigEndian>());
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    for _ in 0..buffer.capacity() { buffer.push(0u8); }
    try!(r.read(&mut buffer));
    let offset      : u32   = BigEndian::read_u32(&buffer[0..4]);
    let block_size  : u32   = BigEndian::read_u32(&buffer[4..8]);
    if offset > 0 || block_size > 0 {
      return Err(AudioError::UnsupportedError("Can't read block-aligned data".to_string()));
    }
    let data: Vec<u8> = buffer[8..size as usize].to_vec();
    Ok(
      SoundDataChunk {
        size:       size,
        offset:     offset,
        block_size: block_size,
        data:       data
      }
    )
  }
}

fn convert_from_ieee_extended(bytes: &[u8]) -> f64 {
  let mut num: f64;
  let mut exponent: isize;
  let mut hi_mant: u32;
  let mut low_mant: u32;

  exponent = ( ((bytes[0] as u16 & 0x7f) << 8) | (bytes[1] & 0xff) as u16 ) as isize;
  hi_mant =   (bytes[2] as u32 & 0xff)  << 24
      |   (bytes[3] as u32 & 0xff)  << 16
      |   (bytes[4] as u32 & 0xff)  << 8
      |   (bytes[5] as u32 & 0xff);
  low_mant =  (bytes[6] as u32 & 0xff)  << 24
      |   (bytes[7] as u32 & 0xff)  << 16
      |   (bytes[8] as u32 & 0xff)  << 8
      |   (bytes[9] as u32 & 0xff);

  if exponent == 0 && hi_mant == 0 && low_mant == 0 {
    return 0f64;
  }

  if exponent == 0x7fff {
    panic!("Sampling rate is not a number!");
  }
  else {
    exponent -= 16383;
    exponent -= 31;
    num = (hi_mant as f64) * (exponent as f64).exp2();
    exponent -= 32;
    num += (low_mant as f64) * (exponent as f64).exp2();
  }

  if bytes[0] & 0x80 > 0 {
    return -num;
  }
  else {
    return num;
  }
}

/*
fn convert_to_ieee_extended(sample_rate: usize) -> Vec<u8>{
  if sample_rate == 0 {
    let vec: Vec<u8> = vec![0,0,0,0,0,0,0,0,0,0];
    return vec;
  }

  let mut num   : f64 = sample_rate as f64;
  let mut exponent: isize;
  let mut f_mant  : f64;
  let mut fs_mant : f64;
  let mut hi_mant : u32;
  let mut low_mant: u32;


  let sign: isize = match num < 0f64 {
    true => { num *= -1f64; 0x8000 },
    false => { 0x0000 }
  };

  let tuple = Float::frexp(num);  // unstable in 1.0
  f_mant    = tuple.0;
  exponent  = tuple.1;

  if exponent > 16384 || !(f_mant < 1f64) {
    exponent  = (sign|0x7fff) as isize;
    hi_mant   = 0;
    low_mant  = 0;
  }
  else {
    exponent += 16382;
    if exponent < 0 {
      f_mant    = (f_mant as f64) * (exponent as f64).exp2();
      exponent  = 0;
    }

    exponent  |= sign as isize;
    f_mant    = f_mant * (32 as f64).exp2();
    fs_mant   = f_mant.floor();
    hi_mant   = fs_mant as u32;
    f_mant    = (f_mant - fs_mant) * (32 as f64).exp2();
    fs_mant   = f_mant.floor();
    low_mant  = fs_mant as u32;
  }

  let vec: Vec<u8> = vec![
    (exponent >> 8)   as u8,
     exponent         as u8,
    (hi_mant  >> 24)  as u8,
    (hi_mant  >> 16)  as u8,
    (hi_mant  >> 8)   as u8,
     hi_mant          as u8,
    (low_mant >> 24)  as u8,
    (low_mant >> 16)  as u8,
    (low_mant >> 8)   as u8,
     low_mant         as u8
  ];

  return vec;
}
*/

#[cfg(test)]
mod tests {
  use super::convert_from_ieee_extended;
  #[test]
  fn test_convert_from_ieee_extended() {
    let sample_rate = &[0x40, 0x0E, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let result = convert_from_ieee_extended(sample_rate);
    assert_eq!(44100u32, result as u32);
  }
  /*
  #[test]
  fn test_convert_to_ieee_extended() {
    let sample_rate_in_bytes = vec![0x40, 0x0E, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let sample_rate = 44100;
    let result = convert_to_ieee_extended(sample_rate);
    assert_eq!(sample_rate_in_bytes, result);
  }
  */
}
