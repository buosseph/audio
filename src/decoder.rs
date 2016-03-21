use std::cmp::min;

use codecs::Codec;
use codecs::Codec::*;
use sample::*;

#[derive(Debug)]
pub struct AudioDecoder {
  pub codec:       Option<Codec>,
  pub bit_depth:   u32,
  pub sample_rate: u32,
  pub channels:    u32,
  pub data:        Vec<u8>,

  index: usize
}

impl AudioDecoder {
  pub fn new() -> Self {
    AudioDecoder {
      index:       0,
      codec:       None,
      bit_depth:   0,
      sample_rate: 0,
      channels:    0,
      data:        Vec::new()
    }
  }
}

impl Iterator for AudioDecoder {
  type Item = Sample;

  fn next(&mut self) -> Option<Sample> {
    // Has reached end of iterator
    if self.index >= self.data.len() {
      return None;
    }

    if let Some(codec) = self.codec {
      let remaining_bytes = self.data.len() - self.index;
      let sample_size     = (codec.bit_depth() / 8) as usize;
      let slice_size      = min(sample_size, remaining_bytes);

      // Not enough bytes in next slice to decode sample
      if slice_size != sample_size {
        return None;
      }

      let slice   = &self.data[self.index .. self.index + slice_size];
      self.index += slice_size;

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
          ::codecs::lpcm::read_sample(slice, codec).ok()
        },

        G711_ALAW |
        G711_ULAW => {
          ::codecs::g711::read_sample(slice, codec).ok()
        }
      }
    }

    // Codec hasn't been set
    else {
      None
    }
  }
}

impl ExactSizeIterator for AudioDecoder {
  fn len(&self) -> usize {
    self.data.len()
  }
}

// TODO: Implement DoubleEndedIterator trait?
// TODO: Add tests
