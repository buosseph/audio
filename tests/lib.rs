extern crate audio;

#[test]
fn load() {
  use std::path::Path;
  use audio::AudioBuffer;
  use audio::error::AudioResult;

  let mut path = Path::new(".gitignore");
  let mut result: AudioResult<AudioBuffer> = audio::open(path);
  assert!(result.is_err());

  path = Path::new("Cargo.toml");
  result = audio::open(path);
  assert!(result.is_err());
}

#[test]
fn save() {
  use std::path::Path;
  use audio::{AudioBuffer, SampleOrder};
  use audio::error::AudioResult;

  // Important cases
  // - Directory (path) doesn't exist
  let path = Path::new("tests/results/empty.wav");
  let samples: Vec<f64> = Vec::with_capacity(0);
  let audio
    = AudioBuffer {
      bit_rate: 16u32,
      sample_rate: 44100u32,
      channels: 2u32,
      order: SampleOrder::INTERLEAVED,
      samples: samples,
    };
  let result: AudioResult<()> = audio::save(path, &audio);
  assert!(result.is_ok());
}
