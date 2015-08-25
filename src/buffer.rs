use error::AudioResult;
use sample::{Sample, SampleOrder};

/// A container for audio samples and important attributes.
#[derive(Clone, Debug)]
pub struct AudioBuffer {
  /// Number of quantization levels
  pub bit_depth:   u32,
  /// Number of samples per second
  pub sample_rate: u32,
  /// Number of channels
  pub channels:    u32,
  /// Organization of samples
  pub order:       SampleOrder,
  /// Decoded audio samples
  pub samples:     Vec<Sample>
}

impl AudioBuffer {
  /// Constructor
  pub fn new(bit_depth: u32, sample_rate: u32, channels: u32, order: SampleOrder, samples: Vec<Sample>) -> Self {
    AudioBuffer {
      bit_depth: bit_depth,
      sample_rate: sample_rate,
      channels: channels,
      order: order,
      samples: samples
    }
  }

  /// Create an `AudioBuffer` from a set of bytes using a `Codec`
  ///
  /// Bytes are interpreted using a `Codec` and are passed to a new `AudioBuffer`
  /// set with the specified sample rate and number of channels.
  pub fn from_raw(sample_rate: u32, channels: u32, bytes: &[u8], codec: ::codecs::Codec) -> AudioResult<Self> {
    Ok(AudioBuffer {
      bit_depth: codec.bit_depth() as u32,
      sample_rate: sample_rate,
      channels: channels,
      order: SampleOrder::Interleaved,
      samples: try!(::codecs::decode(bytes, codec))
    })
  }

  /// The duration of the audio in milliseconds.
  pub fn duration(&self) -> usize {
    self.samples.len() / self.channels as usize * 1000
                       / self.sample_rate as usize
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn from_raw() {
    use ::codecs::Codec::LPCM_I16_BE;
    let bytes = vec![0x7f, 0xff, 0x80, 0x00, 0x80, 0x00, 0x7f, 0xff];
    let audio = AudioBuffer::from_raw(44100, 2, &bytes, LPCM_I16_BE).unwrap();
    assert_eq!(44100, audio.sample_rate);
    assert_eq!(2, audio.channels);
    assert_eq!(16, audio.bit_depth);
    assert!(( 1f32 - audio.samples[0]).abs() < 1e-4);
    assert!((-1f32 - audio.samples[1]).abs() < 1e-4);
    assert!((-1f32 - audio.samples[2]).abs() < 1e-4);
    assert!(( 1f32 - audio.samples[3]).abs() < 1e-4);
  }

  #[test]
  fn duration() {
    use sample::SampleOrder;

    let mut audio =
      AudioBuffer::new(16, 44100, 1, SampleOrder::Mono, vec![0f32; 48000]);
    assert_eq!(1088, audio.duration());

    audio.samples = vec![0f32; 44200];
    assert_eq!(1002, audio.duration());

    audio.channels = 2;
    assert_eq!(501, audio.duration());

    audio.samples = vec![0f32; 44100];
    assert_eq!(500, audio.duration());

    audio.channels = 5;
    audio.samples = vec![0f32; 48000];
    assert_eq!(217, audio.duration());
  }
}
