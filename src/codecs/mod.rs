use std::fmt;
use buffer::*;
use error::AudioResult;

pub mod lpcm;
pub use codecs::lpcm::LPCM as LPCM;

pub enum Endian {
  LittleEndian,
  BigEndian
}

/// How a sample is stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleFormat {
  Unsigned8,
  Signed8,
  Signed16,
  Signed24,
  Signed32
}

/// All supported audio codecs.
pub enum Codec {
  LPCM
}

impl fmt::Display for Codec {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(fmt, "{}", self)
  }
}

/// All functions required by all codecs
pub trait AudioCodec {
  fn read(bytes: &[u8], sample_format: SampleFormat, endian: Endian) -> AudioResult<Vec<Sample>>;
  fn create(audio: &AudioBuffer, sample_format: SampleFormat, endian: Endian) -> AudioResult<Vec<u8>>;
}
