use std::iter::IntoIterator;

use codecs::Codec;
use codecs::Codec::*;
use error::*;
use sample::*;


// TODO: Refactor to use reference of samples (requires lifetime)
#[derive(Debug)]
pub struct AudioEncoder {
  pub codec:       Codec,
  pub bit_depth:   u32,
  pub sample_rate: u32,
  pub channels:    u32,
  pub samples:     Vec<Sample>
}

impl AudioEncoder {
  pub fn new() -> Self {
    AudioEncoder {
      codec: Codec::LPCM_I16_LE,
      bit_depth: 0,
      sample_rate: 0,
      channels: 0,
      samples: Vec::new()
    }
  }

  // pub fn from_buffer(audio: &AudioBuffer, codec: Codec) -> Self {
  //   AudioEncoder {
  //     codec: codec,
  //     bit_depth: codec.bit_depth(),
  //     sample_rate; audio.sample_rate,
  //     channels: audio.channels,
  //     samples: &audio.samples
  //   }
  // }
}

impl IntoIterator for AudioEncoder {
  type Item = Sample;
  type IntoIter = ::std::vec::IntoIter<Sample>;

  fn into_iter(self) -> Self::IntoIter {
    self.samples.into_iter()
  }
}

impl AudioEncoder {
  pub fn encode(&mut self) -> AudioResult<Vec<u8>> {
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
        ::codecs::lpcm::write(self.samples.as_slice(), self.codec)
      },

      G711_ALAW |
      G711_ULAW => {
        ::codecs::g711::write(self.samples.as_slice(), self.codec)
      }
    }
  }
}
