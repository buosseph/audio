use std::fmt;
use buffer::*;
use error::AudioResult;

pub mod lpcm;
pub mod g711;
pub use codecs::lpcm::LPCM as LPCM;

/// All supported audio codecs. Any codec where endianess and type influence
/// coding is represented with a separate variant.
#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Codec {
  LPCM_U8,
  LPCM_I8,
  LPCM_ALAW,
  LPCM_ULAW,
  LPCM_I16_LE,
  LPCM_I16_BE,
  LPCM_I24_LE,
  LPCM_I24_BE,
  LPCM_I32_LE,
  LPCM_I32_BE,
  LPCM_F32_LE,
  LPCM_F32_BE,
  LPCM_F64_LE,
  LPCM_F64_BE
}

impl fmt::Display for Codec {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(fmt, "{}", self)
  }
}

/// All functions required by all codecs.
pub trait AudioCodec {
  fn read(bytes: &[u8], codec: Codec) -> AudioResult<Vec<Sample>>;
  fn create(audio: &AudioBuffer, codec: Codec) -> AudioResult<Vec<u8>>;
}
