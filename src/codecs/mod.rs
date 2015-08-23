use std::fmt;
use buffer::*;
use error::*;

mod lpcm;
mod g711;

/// All supported audio codecs. Any codec where endianess and type influence
/// coding is represented with a separate variant.
#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Codec {
  LPCM_U8,
  LPCM_I8,
  LPCM_I16_LE,
  LPCM_I16_BE,
  LPCM_I24_LE,
  LPCM_I24_BE,
  LPCM_I32_LE,
  LPCM_I32_BE,
  LPCM_F32_LE,
  LPCM_F32_BE,
  LPCM_F64_LE,
  LPCM_F64_BE,
  G711_ALAW,
  G711_ULAW
}

impl fmt::Display for Codec {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(fmt, "{}", self)
  }
}

/// Decodes bytes using `codec` returning `Sample`s.
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

/// Encodes `Sample`s using `codec` returning bytes.
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

/// All functions required by a codec.
pub trait AudioCodec {
  fn read(bytes: &[u8], codec: Codec) -> AudioResult<Vec<Sample>>;
  fn create(audio: &AudioBuffer, codec: Codec) -> AudioResult<Vec<u8>>;
}
