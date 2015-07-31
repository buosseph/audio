//! WAVE Chunks
use std::fmt;
use std::io::Write;
use buffer::AudioBuffer;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use codecs::Codec;
use codecs::Codec::*;
use error::*;
use self::CompressionType::*;
use self::FormatChunkVariant::*;
use traits::Chunk;
use wave::FMT;

/// Format tag for the wave extensible format. Unlike chunk identifiers,
/// this is read as little endian data since it is within the chunk.
const WAVE_FORMAT_EXTENSIBLE_TAG: u16 = 0xFFFE;

/// Supported WAVE chunks
///
/// Some chunks may only contain one item with a size specified by the chunk
/// size, such as the fact and data chunks. These chunks don't have a struct
/// implemented because they are trivial to read.
pub enum WaveChunk {
  Format,
  Fact,
  Data
}

/// Supported compression codes in the WAVE format chunk. These also correspond
/// to wave format tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
  Unknown   = 0x0000,
  Pcm       = 0x0001
  // IEEEFloat = 0x0003,
  // ALaw      = 0x0006,
  // MuLaw     = 0x0007
}

impl fmt::Display for CompressionType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(fmt, "{}", self)
  }
}

/// The WAVE Format Chunk.
///
/// This chunk provides most of the information required to decode the sampled
/// data.
///
/// The format chunk can be of three different sizes: 16, 18, and 40 bytes. If
/// the data is encoded as LPCM, then the chunk will be 16 bytes long. If the
/// data is encoded using any other codec, then the chunk will be 18 bytes long.
/// Non-LPCM data also requires the presence of a fact chunk within the file.
///
/// Wave files also have an extensible format which provided additional data
/// to eliminate ambiguities in the standard format. The `WAVE_EXTENSIBLE_FORMAT`
/// requires the chunk to be 40 bytes long, and moves the compression type
/// information later in the chunk. Extensible format data is included 
#[derive(Debug, Clone, Copy)]
pub struct FormatChunk {
  pub compression_type: CompressionType,
  pub num_channels:     u16,
  pub sample_rate:      u32,
  pub data_rate:        u32,
  pub block_size:       u16,
  pub bit_rate:         u16,
  // pub cb_size:                Some(u16),
  // pub valid_bits_per_sample:  Some(u16),
  // pub speaker_position_mask:  Some(u32),  // Requires `extern crate bitflags`
  // pub guid:                   Some([u8; 16])
}

/// The variants of the format chunk with their respective chunk sizes.
pub enum FormatChunkVariant {
  WaveFormatPcm         = 16,
  WaveFormatNonPcm      = 18,
  WaveFormatExtensible  = 40
}

// /// Speaker positions supported by wave extensible format.
// bitflags! {
//   speaker_positions SpeakerPosition: u32 {
//     const SPEAKER_FRONT_LEFT            = 0x1,
//     const SPEAKER_FRONT_RIGHT           = 0x2,
//     const SPEAKER_FRONT_CENTER          = 0x4,
//     const SPEAKER_LOW_FREQUENCY         = 0x8,
//     const SPEAKER_BACK_LEFT             = 0x10,
//     const SPEAKER_BACK_RIGHT            = 0x20,
//     const SPEAKER_FRONT_LEFT_OF_CENTER  = 0x40,
//     const SPEAKER_FRONT_RIGHT_OF_CENTER = 0x80,
//     const SPEAKER_BACK_CENTER           = 0x100,
//     const SPEAKER_SIDE_LEFT             = 0x200,
//     const SPEAKER_SIDE_RIGHT            = 0x400,
//     const SPEAKER_TOP_CENTER            = 0x800,
//     const SPEAKER_TOP_FRONT_LEFT        = 0x1000,
//     const SPEAKER_TOP_FRONT_CENTER      = 0x2000,
//     const SPEAKER_TOP_FRONT_RIGHT       = 0x4000,
//     const SPEAKER_TOP_BACK_LEFT         = 0x8000,
//     const SPEAKER_TOP_BACK_CENTER       = 0x10000,
//     const SPEAKER_TOP_BACK_RIGHT        = 0x20000,
//     const SPEAKER_RESERVED              = 0x7FFC0000,
//     const SPEAKER_ALL                   = 0x80000000,  // Any possible speaker configuration
//     // DVD Speaker Positions mapping
//     const SPEAKER_GROUND_FRONT_LEFT   = SPEAKER_FRONT_LEFT,
//     const SPEAKER_GROUND_FRONT_CENTER = SPEAKER_FRONT_CENTER,
//     const SPEAKER_GROUND_FRONT_RIGHT  = SPEAKER_FRONT_RIGHT,
//     const SPEAKER_GROUND_REAR_LEFT    = SPEAKER_BACK_LEFT,
//     const SPEAKER_GROUND_REAR_RIGHT   = SPEAKER_BACK_RIGHT,
//     const SPEAKER_TOP_MIDDLE          = SPEAKER_TOP_CENTER,
//     const SPEAKER_SUPER_WOOFER        = SPEAKER_LOW_FREQUENCY,
//     // Predefined configurations
//     // DirectSound Speaker Configurations
//     const SPEAKER_MONO      = SPEAKER_FRONT_CENTER,
//     const SPEAKER_STEREO    = SPEAKER_FRONT_LEFT.bits
//                             | SPEAKER_FRONT_RIGHT.bits,
//     const SPEAKER_QUAD      = SPEAKER_FRONT_LEFT.bits
//                             | SPEAKER_FRONT_RIGHT.bits
//                             | SPEAKER_BACK_LEFT.bits
//                             | SPEAKER_BACK_RIGHT.bits,
//     const SPEAKER_SURROUND  = SPEAKER_FRONT_LEFT.bits
//                             | SPEAKER_FRONT_RIGHT.bits
//                             | SPEAKER_FRONT_CENTER.bits
//                             | SPEAKER_BACK_CENTER.bits,
//     const SPEAKER_5_1       = SPEAKER_FRONT_LEFT.bits
//                             | SPEAKER_FRONT_RIGHT.bits
//                             | SPEAKER_FRONT_CENTER.bits
//                             | SPEAKER_LOW_FREQUENCY.bits
//                             | SPEAKER_BACK_LEFT.bits
//                             | SPEAKER_BACK_RIGHT.bits,
//     const SPEAKER_7_1       = SPEAKER_FRONT_LEFT.bits
//                             | SPEAKER_FRONT_RIGHT.bits
//                             | SPEAKER_FRONT_CENTER.bits
//                             | SPEAKER_LOW_FREQUENCY.bits
//                             | SPEAKER_BACK_LEFT.bits
//                             | SPEAKER_BACK_RIGHT.bits
//                             | SPEAKER_FRONT_LEFT_OF_CENTER.bits
//                             | SPEAKER_FRONT_RIGHT_OF_CENTER.bits
//   }
// }

/// Determines if codec is supported by container.
pub fn is_supported(codec: Codec) -> AudioResult<bool> {
  match codec {
    LPCM_U8     |
    LPCM_I16_LE |
    LPCM_I24_LE |
    LPCM_I32_LE => Ok(true),
    c @ _       =>
      return Err(AudioError::UnsupportedError(
        format!("Wave does not support {:?} codec", c)
      ))
  }
}

/// Determines if the given audio requires the use of the wave extensible format.
pub fn is_extensible(audio: &AudioBuffer, codec: Codec) -> AudioResult<bool> {
  match (audio.channels, codec) {
    // The simple version:
    // (ch, _) if ch >= 3  => true,
    // if bit_rate % 8 != 0 => true,
    // speaker_positions != 0 => true
    //
    // If audio has more than two channels, it must use the extensible format,
    // but the codec must still be checked if it is supported by the container.
    (ch, LPCM_U8)     if ch >= 3  => Ok(true),
    (ch, LPCM_I16_LE) if ch >= 3  => Ok(true),
    (ch, LPCM_I24_LE) if ch >= 3  => Ok(true),
    (ch, LPCM_I32_LE) if ch >= 3  => Ok(true),
    // 1 or 2 channel LPCM can use the standard format.
    (ch, LPCM_U8)     if ch < 2   => Ok(false),
    (ch, LPCM_I16_LE) if ch < 2   => Ok(false),
    (ch, LPCM_I24_LE) if ch < 2   => Ok(false),
    (ch, LPCM_I32_LE) if ch < 2   => Ok(false),
    (_, c @ _) =>
      return Err(AudioError::UnsupportedError(
        format!("Wave does not support {:?} codec", c)
      ))
  }
}

impl FormatChunk {
  #[inline]
  pub fn calculate_size(codec: Codec) -> AudioResult<u32> {
    match codec {
      LPCM_U8     |
      LPCM_I16_LE |
      LPCM_I24_LE |
      LPCM_I32_LE => Ok(WaveFormatPcm as u32),
      // LPCM_ALAW   |
      // LPCM_ULAW   |
      // LPCM_F32_LE |
      // LPCM_F64_LE => Ok(WaveFormatNonPcm as u32),
      c @ _       =>
        return Err(AudioError::UnsupportedError(
          format!("Wave does not support {:?} codec", c)
        ))
    }
  }
  // Cases:
  // is WAVE_FORMAT_EXTENSIBLE if:
  //  - LPCM data is more than 16-bits per sample
  //  - Data has more than two channels
  //  - Actual number of bits per sample is not equal to container size???
  //  - A mapping of channels to speakers is provided
  // else WAVE_FORMAT_PCM if:
  //  - Data is LPCM (16 or 8 bit, mono or stereo)
  // else WAVE_FORMAT_NON_PCM
  pub fn write<W: Write>(writer: &mut W, audio: &AudioBuffer) -> AudioResult<()> {
    try!(writer.write(FMT));
    // WaveFormatPcm
    try!(writer.write_u32::<LittleEndian>(WaveFormatPcm as u32));
    try!(writer.write_u16::<LittleEndian>(Pcm as u16));
    try!(writer.write_u16::<LittleEndian>(audio.channels as u16));
    try!(writer.write_u32::<LittleEndian>(audio.sample_rate as u32));
    try!(writer.write_u32::<LittleEndian>(
      audio.sample_rate * audio.channels * audio.bit_rate / 8u32));
    try!(writer.write_u16::<LittleEndian>(
      (audio.channels * audio.bit_rate / 8u32) as u16));
    try!(writer.write_u16::<LittleEndian>(audio.bit_rate as u16));
    Ok(())
  }
}

impl Chunk for FormatChunk {
  #[inline]
  fn read(buffer: &[u8]) -> AudioResult<FormatChunk> {
    let mut format_tag: u16 = LittleEndian::read_u16(&buffer[0..2]);
    if format_tag == WAVE_FORMAT_EXTENSIBLE_TAG {
      format_tag = LittleEndian::read_u16(&buffer[24..26])
    }
    let compression_type : CompressionType = 
      match format_tag {
        0x0001 => Pcm,
        // 0x0003 => Float,
        // 0x0006 => ALaw,
        // 0x0007 => MuLaw,
        _ => Unknown,
      };
    Ok(
      FormatChunk {
        compression_type: compression_type,
        num_channels:     LittleEndian::read_u16(&buffer[2..4]),
        sample_rate:      LittleEndian::read_u32(&buffer[4..8]),
        data_rate:        LittleEndian::read_u32(&buffer[8..12]),
        block_size:       LittleEndian::read_u16(&buffer[12..14]),
        bit_rate:         LittleEndian::read_u16(&buffer[14..16]),
      }
    )
  }
}
