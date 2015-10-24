//! AIFF Chunks
use std::fmt;
use std::io::{Read, Seek, SeekFrom, Write};
use aiff;
use aiff::is_aifc;
use aiff::read_codec;
use buffer::AudioBuffer;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use codecs::Codec;
use codecs::Codec::*;
use sample::*;
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
/// uncompressed audio data. For example, although uLaw and aLaw codecs
/// compress 16-bit audio to 8-bits, the bit_depth is be set to 16 since the
/// original data uses 16-bits.
#[derive(Debug, Clone, Copy)]
pub struct CommonChunk {
  pub num_channels:     i16,
  pub num_frames:       u32,
  pub bit_depth:        i16,
  pub sample_rate:      f64,
  pub compression_type: CompressionType
}


impl CommonChunk {
  /// Returns the size of the chunk based on the codec to be used to encode
  /// the relevant audio samples.
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

  /// Returns the bit depth corresponding to the given codec used in this
  /// format.
  #[inline]
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

  /// Writes the Common Chunk to the given writer using information from the
  /// provided `AudioBuffer` and `Codec` to be used for sample encoding.
  ///
  /// This function writes the entire Common Chunk, including the chunk
  /// identifier and the chunk size.
  pub fn write<W: Write>(writer: &mut W, audio: &AudioBuffer, codec: Codec) -> AudioResult<()> {
    let chunk_size: i32 = try!(Self::calculate_size(codec));
    let is_aifc: bool   = try!(is_aifc(codec));

    try!(writer.write(aiff::COMM));
    try!(writer.write_i32::<BigEndian>(chunk_size));
    try!(writer.write_i16::<BigEndian>(audio.channels as i16));
    try!(writer.write_u32::<BigEndian>(audio.samples.len() as u32 / audio.channels));
    try!(writer.write_i16::<BigEndian>(try!(Self::get_bit_depth(codec))));
    try!(writer.write(&convert_to_ieee_extended(audio.sample_rate as f64)));

    // Write additional AIFF-C information
    if is_aifc {
      // Write compression type identifier
      let compression =
        match codec {
          LPCM_U8     => RAW,
          G711_ALAW   => ALAW,
          G711_ULAW   => ULAW,
          LPCM_F32_BE => FL32,
          LPCM_F64_BE => FL64,
          fmt @ _     =>
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

  /// Reads the data within the Common Chunk from the given reader.
  ///
  /// This function assumes the Common Chunk has already been found and its
  /// header read using `find_chunk(reader, COMM)` and should use the
  /// chunk_size returned by that function.
  pub fn read_chunk_data<R: Read + Seek>(reader: &mut R, chunk_size: usize) -> AudioResult<Self> {
    let mut chunk_buffer = vec![0u8; chunk_size];
    try!(reader.read(&mut chunk_buffer));

    let compression_type =
      if chunk_buffer.len() > 18 {
        match &chunk_buffer[18..22] {
          tag if tag == NONE.0  => CompressionType::Pcm,
          tag if tag == RAW.0  => CompressionType::Raw,
          tag if tag == FL32.0
              || tag == b"FL32" => CompressionType::Float32,
          tag if tag == FL64.0
              || tag == b"FL64" => CompressionType::Float64,
          tag if tag == ALAW.0  => CompressionType::ALaw,
          tag if tag == ULAW.0  => CompressionType::MuLaw,
          _ => {
            return Err(AudioError::Unsupported(
              "Unknown compression type".to_string()
            ))
          }
        }
      }
      else {
        CompressionType::Pcm
      };
    Ok(
      CommonChunk {
        compression_type: compression_type,
        num_channels:     BigEndian::read_i16(&chunk_buffer[0..2]),
        num_frames:       BigEndian::read_u32(&chunk_buffer[2..6]),
        bit_depth:        BigEndian::read_i16(&chunk_buffer[6..8]),
        sample_rate:      convert_from_ieee_extended(&chunk_buffer[8..18])
      }
    )
  }
}

/// The AIFF Sound Data Chunk.
///
/// This chunk contains the encoded sound data, which cannot be read without
/// the information from the Common Chunk. This chunk also contains offset
/// and block size values within 8 bytes prior to the actual sound data.
/// Because there is little additional information other than the encoded
/// audio samples, reading this chunks results in the return of the actual
/// audio samples rather than a struct representing this chunk.
pub struct SoundDataChunk;

impl SoundDataChunk {
  /// Writes the Sound Data Chunk to the given writer using the provided
  /// encoded data.
  ///
  /// This function writes the entire Sound Data Chunk, including the chunk
  /// identifier and the chunk size. The offset and block size values are
  /// always written as zero. A pad byte is added if the encoded data is not
  /// of even size.
  pub fn write<W: Write>(writer: &mut W, encoded_data: &[u8]) -> AudioResult<()> {
    try!(writer.write(aiff::SSND));
    try!(writer.write_i32::<BigEndian>((encoded_data.len() + 8) as i32));
    try!(writer.write_u64::<BigEndian>(0)); // offset and block_size. Both 0
    try!(writer.write_all(encoded_data));

    // IFF chunks must be of even size
    if encoded_data.len() % 2 != 0 {
      try!(writer.write_u8(0));
    }
    Ok(())
  }

  /// Reads the data within the SSND chunk from the given reader.
  ///
  /// This function assumes the SSND chunk has already been found and its header
  /// read using `find_chunk(reader, SSND)` and should use the chunk_size returned
  /// by that function. The audio data within the chunk is decoded using the give
  /// codec, which can be determined using
  /// `determine_codec(compression_type, bit_depth)`.
  ///
  /// The current implementaiton skips the first 8 bytes of the data inside the
  /// SSND chunk, which contains the sample offset and block_size, as that
  /// information is not used in decoding.
  pub fn read_chunk_data<R: Read + Seek>(reader: &mut R, chunk_size: usize, codec: Codec) -> AudioResult<Vec<Sample>> {
    // Skip the offset and block_size bytes, we don't use them.
    try!(reader.seek(SeekFrom::Current(8)));

    let mut data_buffer = vec![0u8; chunk_size - 8];
    try!(reader.read(&mut data_buffer));
    read_codec(&data_buffer, codec)
  }
}

// ----------------------------------------------------------
// IEEE-Extended Functions
// ----------------------------------------------------------

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
