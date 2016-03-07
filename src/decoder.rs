use std::iter::IntoIterator;

use codecs::Codec;
use codecs::Codec::*;
use error::*;
use sample::*;

#[derive(Debug)]
pub struct AudioDecoder {
  pub codec:       Codec,
  pub bit_depth:   u32,
  pub sample_rate: u32,
  pub channels:    u32,
  pub num_frames:  u32,
  pub data:        Vec<u8>
}

impl AudioDecoder {
  pub fn new() -> Self {
    AudioDecoder {
      codec: Codec::LPCM_I16_LE,
      bit_depth: 0,
      sample_rate: 0,
      channels: 0,
      num_frames: 0,
      data: Vec::new()
    }
  }
}

impl IntoIterator for AudioDecoder {
  type Item = u8;
  type IntoIter = ::std::vec::IntoIter<u8>;

  fn into_iter(self) -> Self::IntoIter {
    self.data.into_iter()
  }
}

impl AudioDecoder {
  pub fn decode(&mut self) -> AudioResult<Vec<Sample>> {
    match self.codec {
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
        ::codecs::lpcm::read(self.data.as_slice(), self.codec)
      },

      G711_ALAW |
      G711_ULAW => {
        ::codecs::g711::read(self.data.as_slice(), self.codec)
      }
    }
  }
}
