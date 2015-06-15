extern crate audio;

#[test]
fn test_load() {
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
fn test_save() {
  use std::path::Path;
  use audio::{AudioBuffer, SampleOrder};
  use audio::error::AudioResult;

  // Important cases
  // - Directory (path) doesn't exist
  let path = Path::new("results/empty.wav");
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

#[test]
fn test_read_write_eq() {
  use std::path::{Path, PathBuf};
  use audio::AudioBuffer;

  let mut path = PathBuf::from("tests/wav");
  let files = vec![
    "i16-pcm-mono.wav",
    "i16-pcm-stereo.wav",
    "Warrior Concerto - no meta.wav",
  ];

  for file in files.iter() {
    path.set_file_name(file);

    let audio = audio::open(path.as_path()).unwrap();
    let total_samples = audio.samples.len();
    let channels = audio.channels;
    let bit_rate = audio.bit_rate;
    let sample_rate = audio.sample_rate;
    let sample_order = audio.order;

    let written = audio::save(&Path::new("tmp.wav"), &audio);
    assert!(written.is_ok());
    let verify: AudioBuffer = audio::open(&Path::new("tmp.wav")).unwrap();

    // Assert written file is same length as read file!
    assert_eq!(total_samples, verify.samples.len());
    assert_eq!(channels, verify.channels);
    assert_eq!(bit_rate, verify.bit_rate);
    assert_eq!(sample_rate, verify.sample_rate);
    assert_eq!(sample_order, verify.order);
  }
}