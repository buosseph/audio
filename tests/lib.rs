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
  let samples: Vec<f32> = Vec::with_capacity(0);
  let audio =
    AudioBuffer {
      bit_rate:    16u32,
      sample_rate: 44100u32,
      channels:    2u32,
      order:       SampleOrder::INTERLEAVED,
      samples:     samples,
    };
  let result: AudioResult<()> = audio::save(path, &audio);
  assert!(result.is_ok());
}

#[test]
#[should_panic]
fn transcoding() {
  use std::path::{Path, PathBuf};
  use audio::AudioBuffer;
  use audio::buffer::FromSample;
  use audio::Sample;

  // let mut buffers: Vec<AudioBuffer> = Vec::new();
  let mut samples: Vec<Sample> = Vec::new();
  let mut path_buf = PathBuf::from("tests");
  path_buf.push("wav");
  path_buf.push("empty.wav");
  let files = vec![
    "M1F1-uint8-AFsp.wav",
    "M1F1-int16-AFsp.wav",
    "M1F1-int24-AFsp.wav",
    "M1F1-int32-AFsp.wav",
    "M1F1-float32-AFsp.wav",
    "M1F1-float64-AFsp.wav",
    "M1F1-Alaw-AFsp.wav",
    "M1F1-mulaw-AFsp.wav"
  ];
  for file in files.iter() {
    path_buf.set_file_name(file);
    if let Ok(audio) = audio::open(path_buf.as_path()) {
      samples.push(audio.samples[100]);
    }
  }
  for s in samples.iter() {
    println!("{:?}", u8::from_sample(*s));
    // assert!((0.016448975f32 - *s).abs() < 1e6f32;
    // assert_eq!(0.016448975f32, *s);
  }
  panic!();
}