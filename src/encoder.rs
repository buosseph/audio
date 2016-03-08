use codecs::Codec;
use codecs::Codec::*;
use buffer::AudioBuffer;
use error::*;
use sample::*;


#[derive(Debug)]
pub struct AudioEncoder<'a> {
  pub codec:       Codec,
  pub bit_depth:   u32,
  pub sample_rate: u32,
  pub channels:    u32,
  pub samples:     &'a [Sample]
}

impl<'a> AudioEncoder<'a> {
  pub fn new() -> Self {
    AudioEncoder {
      codec: Codec::LPCM_I16_LE,
      bit_depth: 0,
      sample_rate: 0,
      channels: 0,
      samples: &[]
    }
  }

  pub fn from_buffer(audio: &'a AudioBuffer,
                     codec: Codec)
  -> Self {
    AudioEncoder {
      codec:       codec,
      bit_depth:   codec.bit_depth() as u32,
      sample_rate: audio.sample_rate,
      channels:    audio.channels,
      samples:     &audio.samples
    }
  }
}

impl<'a> AudioEncoder<'a> {
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
        ::codecs::lpcm::write(self.samples, self.codec)
      },

      G711_ALAW |
      G711_ULAW => {
        ::codecs::g711::write(self.samples, self.codec)
      }
    }
  }
}
