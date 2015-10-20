//! AIFF Chunks
use std::fmt;
use std::io::Write;
use aiff::{COMM, SSND};
use buffer::AudioBuffer;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use codecs::Codec;
use codecs::Codec::*;
use self::CompressionType::*;
use traits::Chunk;
use error::*;

/// AIFC compression type tags and strings.
const NONE: (&'static [u8; 4], &'static [u8]) =
  (b"NONE", b"not compressed");
const RAW : (&'static [u8; 4], &'static [u8]) =
  (b"raw ", b"");
const ULAW: (&'static [u8; 4], &'static [u8]) =
  (b"ulaw", &[0xB5, 0x6C, 0x61, 0x77, 0x20, 0x32, 0x3A, 0x31]); // µLaw 2:1
const ALAW: (&'static [u8; 4], &'static [u8]) =
  (b"alaw", b"ALaw 2:1");
const FL32: (&'static [u8; 4], &'static [u8]) =
  (b"fl32", b"IEEE 32-bit float");
const FL64: (&'static [u8; 4], &'static [u8]) =
  (b"fl64", b"IEEE 64-bit float");

/// Supported compression codes in the AIFC common chunk.
///
/// In traditional AIFF files there is no option for compression. However, AIFC
/// files are often labeled as `.aiff` despite being a different format. AIFC 
/// decoding is not currently supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
  Pcm,
  Raw,
  ALaw,
  MuLaw,
  Float32,
  Float64
}

impl fmt::Display for CompressionType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(fmt, "{}", self)
  }
}

/// The AIFF Common Chunk.
///
/// This chunk provides most of the information required to decode the sampled
/// data. In AIFC files, bit_depth represents the number of samples used in the
/// uncompressed audio data. For example, although uLaw and aLaw codecs compress
/// 16-bit audio to 8-bits, the bit_depth is be set to 16 since the original
/// data uses 16-bits.
#[derive(Debug, Clone, Copy)]
pub struct CommonChunk {
  pub num_channels:     i16,
  pub num_frames:       u32,
  pub bit_depth:        i16,
  pub sample_rate:      f64,
  pub compression_type: CompressionType
}

/// Determines if the `Codec` given requires the audio to be encoded as AIFF-C.
#[inline]
pub fn is_aifc(codec: Codec) -> AudioResult<bool> {
  match codec {
    G711_ALAW   |
    G711_ULAW   |
    LPCM_U8     |
    LPCM_F32_BE |
    LPCM_F64_BE => Ok(true),
    LPCM_I8     |
    LPCM_I16_BE |
    LPCM_I24_BE |
    LPCM_I32_BE => Ok(false),
    c @ _       =>
      return Err(AudioError::Unsupported(
        format!("Aiff does not support {:?} codec", c)
      ))
  }
}

fn get_bit_depth(codec: Codec) -> AudioResult<i16> {
  match codec {
    LPCM_U8      |
    LPCM_I8      => Ok(8),
    G711_ALAW    |
    G711_ULAW    |
    LPCM_I16_BE  => Ok(16),
    LPCM_I24_BE  => Ok(24),
    LPCM_I32_BE  |
    LPCM_F32_BE  => Ok(32),
    LPCM_F64_BE  => Ok(64),
    c @ _ =>
      return Err(AudioError::Unsupported(
        format!("Aiff does not support the {:?} codec", c)
      ))
  }
}

impl CommonChunk {
  #[inline]
  pub fn calculate_size(codec: Codec) -> AudioResult<i32> {
    match codec {
      LPCM_U8      => Ok(24),
      G711_ALAW    |
      G711_ULAW    => Ok(32),
      LPCM_F32_BE  |
      LPCM_F64_BE  => Ok(40),
      LPCM_I8      |
      LPCM_I16_BE  |
      LPCM_I24_BE  |
      LPCM_I32_BE  => Ok(18),
      c @ _       =>
        return Err(AudioError::Unsupported(
          format!("Aiff does not support {:?} codec", c)
        ))
    }
  }
  pub fn write<W: Write>(writer: &mut W, audio: &AudioBuffer, codec: Codec) -> AudioResult<()> {
    try!(writer.write(COMM));
    let chunk_size: i32 = try!(Self::calculate_size(codec));
    try!(writer.write_i32::<BigEndian>(chunk_size));
    try!(writer.write_i16::<BigEndian>(audio.channels as i16));
    try!(writer.write_u32::<BigEndian>(audio.samples.len() as u32 / audio.channels));
    try!(writer.write_i16::<BigEndian>(try!(get_bit_depth(codec))));
    try!(writer.write(&convert_to_ieee_extended(audio.sample_rate as f64)));
    // Write additional information if aifc
    if try!(is_aifc(codec)) {
      // Write compression type identifier
      let compression =
        match codec {
          LPCM_U8 => RAW,
          G711_ALAW => ALAW,
          G711_ULAW => ULAW,
          LPCM_F32_BE => FL32,
          LPCM_F64_BE => FL64,
          fmt @ _   =>
            return Err(AudioError::Unsupported(
              format!("Common chunk does not support {:?}", fmt)
            ))
        };
      try!(writer.write(compression.0));
      if compression.1.len() == 0 {
        try!(writer.write_i16::<BigEndian>(0));
      }
      // It's only here where the chunk size can become odd.
      else {
        try!(writer.write_u8(compression.1.len() as u8));
        try!(writer.write(compression.1));
        // Add trailing byte if string length + 1 is odd.
        if (compression.1.len() + 1) % 2 == 1 {
          try!(writer.write_u8(0));
        }
      }
    }
    Ok(())
  }
}

impl Chunk for CommonChunk {
  fn read(buffer: &[u8]) -> AudioResult<CommonChunk> {
    let compression_type =
      if buffer.len() > 18 {
        match &buffer[18..22] {
          tag if tag == NONE.0  => Pcm,
          tag if tag ==  RAW.0  => Raw,
          tag if tag == FL32.0
              || tag == b"FL32" => Float32,
          tag if tag == FL64.0
              || tag == b"FL64" => Float64,
          tag if tag == ALAW.0  => ALaw,
          tag if tag == ULAW.0  => MuLaw,
          _ => {
            return Err(AudioError::Unsupported(
              "Unknown compression type".to_string()
            ))
          }
        }
      }
      else {
        Pcm
      };
    Ok(
      CommonChunk {
        compression_type: compression_type,
        num_channels:     BigEndian::read_i16(&buffer[0..2]),
        num_frames:       BigEndian::read_u32(&buffer[2..6]),
        bit_depth:        BigEndian::read_i16(&buffer[6..8]),
        sample_rate:      convert_from_ieee_extended(&buffer[8..18])
      }
    )
  }
}

pub struct SoundDataChunk;
impl SoundDataChunk {
  pub fn write<W: Write>(writer: &mut W, encoded_data: &[u8]) -> AudioResult<()> {
    try!(writer.write(SSND));
    try!(writer.write_i32::<BigEndian>((encoded_data.len() + 8) as i32));
    try!(writer.write_u32::<BigEndian>(0u32));   // offset. For now, always 0
    try!(writer.write_u32::<BigEndian>(0u32));   // block_size. For now, always 0
    try!(writer.write_all(encoded_data));
    // Add trailing byte if data size is odd, all chunks must be of even size.
    if encoded_data.len() % 2 != 0 {
      try!(writer.write_u8(0));
    }
    Ok(())
  }
} 

/// Breaks number into a normalized fraction and a base-2 exponent, satisfying:
/// > - `self = x * 2^exp`
/// > - `0.5 <= abs(x) < 1.0`
fn frexp(num: f64) -> (f64, isize) {
  use std::mem;

  if num == 0f64 || num.is_nan() || num.is_infinite(){
    (num, 0isize)
  }
  else {
    let neg = num < 0f64;
    let bits: u64 = unsafe {mem::transmute_copy(&num)};
    let mut exponent: isize = ((bits >> 52) & (0x7ff)) as isize;
    let mut mantissa: u64 = bits & 0xfffffffffffff;
    if exponent == 0 {
      exponent += 1;
    }
    else {
      mantissa = mantissa | (1u64 << 52);
    }
    exponent -= 1075;
    let mut real_mantissa = mantissa as f64;
    // Normalize
    while real_mantissa > 1f64 {
      mantissa >>= 1;
      real_mantissa /= 2f64;
      exponent += 1;
    }
    if neg {
      real_mantissa *= -1f64;
    }
    (real_mantissa, exponent)
  }
}

/// Converts the 10-byte extended floating-point format to a `f64` value.
pub fn convert_from_ieee_extended(bytes: &[u8]) -> f64 {
  let mut num       : f64;
  let mut exponent  : isize;
  let     hi_mant   : u32;
  let     low_mant  : u32;

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


/// Converts a `f64` value to a 10-byte extended floating-point format.
pub fn convert_to_ieee_extended(sample_rate: f64) -> Vec<u8>{
  if sample_rate == 0f64 {
    let vec: Vec<u8> = vec![0,0,0,0,0,0,0,0,0,0];
    return vec;
  }
  let mut num       : f64 = sample_rate;
  let mut exponent  : isize;
  let mut f_mant    : f64;
  let mut fs_mant   : f64;
  let     hi_mant   : u32;
  let     low_mant  : u32;
  let sign: isize = match num < 0f64 {
    true  => { num *= -1f64; 0x8000 },
    false => { 0x0000 }
  };
  let tuple = frexp(num);
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

    exponent |= sign as isize;
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
mod sample_rate_conversion {
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
