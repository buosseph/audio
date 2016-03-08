use sample::Sample;

/// A container for audio samples and important attributes.
#[derive(Clone, Debug)]
pub struct AudioBuffer {
  /// Number of samples per second
  pub sample_rate: u32,
  /// Number of channels
  pub channels: u32,
  /// Decoded audio samples
  pub samples: Vec<Sample>
}

impl AudioBuffer {
  /// Creates a new, empty `AudioBuffer`.
  pub fn new(sample_rate: u32, channels: u32) -> Self {
    AudioBuffer {
      sample_rate: sample_rate,
      channels: channels,
      samples: vec![0f32; 0]
    }
  }

  /// Creates a new `AudioBuffer` using the given `Sample`s.
  pub fn from_samples(sample_rate: u32, channels: u32, samples: Vec<Sample>) -> Self {
    AudioBuffer {
      sample_rate: sample_rate,
      channels: channels,
      samples: samples
    }
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
  fn new() {
    let audio = AudioBuffer::new(192000, 6);
    assert_eq!(192000, audio.sample_rate);
    assert_eq!(6, audio.channels);
    assert_eq!(0, audio.samples.len());
  }

  #[test]
  fn from_samples() {
    let samples = vec![0f32, 1f32, 0f32, -1f32];
    let audio = AudioBuffer::from_samples(48000, 1, samples);
    assert_eq!(48000, audio.sample_rate);
    assert_eq!(1, audio.channels);
    assert_eq!(4, audio.samples.len());
    assert!(( 0f32 - audio.samples[0]).abs() < 1e-4);
    assert!(( 1f32 - audio.samples[1]).abs() < 1e-4);
    assert!(( 0f32 - audio.samples[2]).abs() < 1e-4);
    assert!((-1f32 - audio.samples[3]).abs() < 1e-4);
  }

  #[test]
  fn duration() {
    let mut audio =
      AudioBuffer::from_samples(44100, 1, vec![0f32; 48000]);
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
