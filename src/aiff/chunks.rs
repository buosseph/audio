//! AIFF Chunks
//!
//! This module is unstable unitl
//! `f64::frexp` is stablized.
use std::fmt;
use byteorder::{ByteOrder, ReadBytesExt, BigEndian};
use traits::Chunk;
use error::*;

/// Enumeration of supported AIFF chunks.
pub enum AiffChunk {
  Common,
  SoundData
}

/// Enumeration of supported compression codes in the AIFC common chunk.
///
/// In traditional AIFF files there is no option for compression.
/// However, AIFC files are often labeled as `.aiff` despite being a
/// different format. AIFC decoding is not currently supported.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
  Unknown = 0,
  PCM     = 1
}

impl fmt::Display for CompressionType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(fmt, "{}", self)
  }
}

/// The AIFF Common Chunk.
///
/// This chunk provides most of the information
/// required to decode the sampled data.
#[derive(Debug, Clone, Copy)]
pub struct CommonChunk {
  pub num_channels: i16,
  pub num_frames:   u32,
  pub bit_rate:     i16,
  pub sample_rate:  f64
}

impl Chunk for CommonChunk {
  fn read(buffer: &[u8]) -> AudioResult<CommonChunk> {
    Ok(
      CommonChunk {
        num_channels: BigEndian::read_i16(&buffer[0..2]),
        num_frames:   BigEndian::read_u32(&buffer[2..6]),
        bit_rate:     BigEndian::read_i16(&buffer[6..8]),
        sample_rate:  convert_from_ieee_extended(&buffer[8..18])
      }
    )
  }
}

/// Converts the 10-byte extended floating-point format
/// to a `f64` value.
pub fn convert_from_ieee_extended(bytes: &[u8]) -> f64 {
  let mut num: f64;
  let mut exponent: isize;
  let mut hi_mant: u32;
  let mut low_mant: u32;

  exponent = ( ((bytes[0] as u16 & 0x7f) << 8) | (bytes[1] & 0xff) as u16 ) as isize;
  hi_mant =   (bytes[2] as u32 & 0xff)  << 24
            | (bytes[3] as u32 & 0xff)  << 16
            | (bytes[4] as u32 & 0xff)  << 8
            | (bytes[5] as u32 & 0xff);
  low_mant =  (bytes[6] as u32 & 0xff)  << 24
            | (bytes[7] as u32 & 0xff)  << 16
            | (bytes[8] as u32 & 0xff)  << 8
            | (bytes[9] as u32 & 0xff);

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


/// Converts a `f64` value to a 10-byte extended
/// floating-point format.
pub fn convert_to_ieee_extended(sample_rate: f64) -> Vec<u8>{
  if sample_rate == 0f64 {
    let vec: Vec<u8> = vec![0,0,0,0,0,0,0,0,0,0];
    return vec;
  }

  let mut num   : f64 = sample_rate;
  let mut exponent: isize;
  let mut f_mant  : f64;
  let mut fs_mant : f64;
  let mut hi_mant : u32;
  let mut low_mant: u32;


  let sign: isize = match num < 0f64 {
    true => { num *= -1f64; 0x8000 },
    false => { 0x0000 }
  };

  let tuple = num.frexp();
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

#[cfg(test)]
mod tests {
  use super::convert_from_ieee_extended;
  use super::convert_to_ieee_extended;

  #[test]
  fn extended_to_sample_rate() {
    let sample_rate = &[0x40, 0x0E, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let result = convert_from_ieee_extended(sample_rate);
    assert_eq!(44100u32, result as u32);
  }

  #[test]
  fn sample_rate_to_extended() {
    let sample_rate_in_bytes = vec![0x40, 0x0E, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let sample_rate = 44100f64;
    let result = convert_to_ieee_extended(sample_rate);
    assert_eq!(sample_rate_in_bytes, result);
  }
}
