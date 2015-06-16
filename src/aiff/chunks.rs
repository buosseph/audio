// Unstable

const FORM: i32 = 0x464F524D;
const AIFF: i32 = 0x41494646;
const COMM: i32 = 0x434F4D4D;
const SSND: i32 = 0x53534E44;

use std::fmt;
use std::io::{Read, Seek};
use buffer::*;
use byteorder::{ByteOrder, ReadBytesExt, BigEndian, LittleEndian};
use codecs::{Codec, AudioCodec, LPCM};
use traits::{Container, Chunk};
use error::*;

pub struct AiffContainer {
  compression: CompressionType,
  pub bit_rate: u32,
  pub sample_rate: u32,
  pub channels: u32,
  pub block_size: u32,
  pub order: SampleOrder,
  pub bytes: Vec<u8>
}

impl Container for AiffContainer {
  /// Reads the bytes provided from the reader.
  /// This is where the reading of chunks ocurrs.
  fn open<R: Read + Seek>(r: &mut R) -> AudioResult<AiffContainer> {
    let header: &mut[u8] = &mut[0u8; 12];
    try!(r.read(header));
    
    if BigEndian::read_i32(&header[0..4])  != FORM
    || BigEndian::read_i32(&header[8..12]) != AIFF {
      return Err(AudioError::FormatError("Not valid AIFF".to_string()));
    }
    let file_size = BigEndian::read_i32(&header[4..8]);

    // Read chunks
    Err(AudioError::UnsupportedError("Not completed".to_string()))
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
  num_channels: i16,
  num_sample_frames: u32,
  bit_rate: i16,
  sample_rate: f64 //Extended
}

impl Chunk for CommonChunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<CommonChunk> {
    let size :i32 = try!(r.read_i32::<BigEndian>());
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    buffer = vec![0u8; size as usize];
    try!(r.read(&mut buffer));
    let num_channels      : i16 = BigEndian::read_i16(&buffer[0..2]);
    let num_sample_frames : u32 = BigEndian::read_u32(&buffer[2..6]);
    let bit_rate          : i16 = BigEndian::read_i16(&buffer[6..8]);
    let extended          : &[u8] = &buffer[8..18];
    //let sample_rate       : f64 = convert_from_ieee_extended(extended);
    Ok(
      CommonChunk {
        num_channels:       num_channels,
        num_sample_frames:  num_sample_frames,
        bit_rate:           bit_rate,
        sample_rate:        0f64
      }
    )
  }
}

struct SoundDataChunk {
  offset: u32,
  block_size: u32,
  data: Vec<u8>
}

impl Chunk for SoundDataChunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<SoundDataChunk> {
    // Chunk size includes offset and block_size => (data_size + 8)
    let size :usize = try!(r.read_i32::<BigEndian>()) as usize;
    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    buffer = vec![0u8; size];
    try!(r.read(&mut buffer));
    let offset      : u32   = BigEndian::read_u32(&buffer[0..4]);
    let block_size  : u32   = BigEndian::read_u32(&buffer[4..8]);
    if offset > 0 || block_size > 0 {
      return Err(AudioError::UnsupportedError("Can't read block-aligned data".to_string()));
    }
    let data: Vec<u8> = buffer[8..size].to_vec();
    Ok(
      SoundDataChunk {
        offset: offset,
        block_size: block_size,
        data: data
      }
    )
  }
}

// Uses unstable functions
/*
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
    num = f64::ldexp(hi_mant as f64, exponent);   
    exponent -= 32;
    num  += f64::ldexp(low_mant as f64, exponent);
  }

  if bytes[0] & 0x80 > 0 {
    return -num;
  }
  else {
    return num;
  }
}

#[cfg(test)]
mod tests {
  use super::convert_from_ieee_extended;
  #[test]
  fn test_convert_from_ieee_extended() {
    let sample_rate = &[0x40, 0x0E, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let result = convert_from_ieee_extended(sample_rate);
    assert_eq!(44100u32, result as u32);
  }
}
*/
