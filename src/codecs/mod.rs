use std::fmt;
use buffer::*;
use error::*;

mod lpcm;
mod g711;

/// All supported audio codecs.
///
/// Any codec where endianess and type influence coding is represented with a
/// separate variant.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Codec {
  /// Unsigned 8-bit linear PCM
  LPCM_U8,
  /// Signed 8-bit linear PCM
  LPCM_I8,
  /// Signed 16-bit linear PCM in little endian format
  LPCM_I16_LE,
  /// Signed 16-bit linear PCM in big endian format
  LPCM_I16_BE,
  /// Signed 24-bit linear PCM in little endian format
  LPCM_I24_LE,
  /// Signed 24-bit linear PCM in big endian format
  LPCM_I24_BE,
  /// Signed 32-bit linear PCM in little endian format
  LPCM_I32_LE,
  /// Signed 32-bit linear PCM in big endian format
  LPCM_I32_BE,
  /// 32-bit floating-point linear PCM in little endian format
  LPCM_F32_LE,
  /// 32-bit floating-point linear PCM in big endian format
  LPCM_F32_BE,
  /// 64-bit floating-point linear PCM in little endian format
  LPCM_F64_LE,
  /// 64-bit floating-point linear PCM in big endian format
  LPCM_F64_BE,
  /// G.711 8-bit A-law
  G711_ALAW,
  /// G.711 8-bit µ-law
  G711_ULAW
}

impl fmt::Display for Codec {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(fmt, "{}", self)
  }
}

/// Decodes bytes using the specified `Codec`.
pub fn decode(bytes: &[u8], codec: Codec) -> AudioResult<Vec<Sample>> {
  use Codec::*;
  match codec {
    LPCM_U8     |
    LPCM_I8     |
    LPCM_I16_LE |
    LPCM_I16_BE |
    LPCM_I24_LE |
    LPCM_I24_BE |
    LPCM_I32_LE |
    LPCM_I32_BE |
    LPCM_F32_LE |
    LPCM_F32_BE |
    LPCM_F64_LE |
    LPCM_F64_BE => {
      lpcm::read(bytes, codec)
    },
    G711_ALAW |
    G711_ULAW => {
      g711::read(bytes, codec)
    }
  }
}

/// Encodes `Sample`s the specified `Codec`.
pub fn encode(audio: &AudioBuffer, codec: Codec) -> AudioResult<Vec<u8>> {
  use Codec::*;
  match codec {
    LPCM_U8     |
    LPCM_I8     |
    LPCM_I16_LE |
    LPCM_I16_BE |
    LPCM_I24_LE |
    LPCM_I24_BE |
    LPCM_I32_LE |
    LPCM_I32_BE |
    LPCM_F32_LE |
    LPCM_F32_BE |
    LPCM_F64_LE |
    LPCM_F64_BE => {
      lpcm::create(audio, codec)
    },
    G711_ALAW |
    G711_ULAW => {
      g711::create(audio, codec)
    }
  }
}
