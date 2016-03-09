extern crate audio;

use std::path::Path;
use audio::{
  AudioBuffer,
  AudioResult
};

#[test]
fn open() {
  let mut result: AudioResult<AudioBuffer>;

  let err_cases =
    vec![
      // Path::extension cases
      //  - No file name
      //  - No embedded `.`
      //  - File name begins with `.` and has no other `.`s within
      "tests/wav/",
      "tests/wav",
      ".gitignore",
      // Unsupported audio or file format
      "Cargo.toml",
      // Does not exist
      "does/not/exist.wav"
    ];
  for path in err_cases.iter() {
    result = audio::open(Path::new(path));
    assert!(result.is_err());
  }

  // Valid path
  result = audio::open(Path::new("tests/wav/M1F1-int16-AFsp.wav"));
  assert!(result.is_ok());
}

#[test]
fn save() {
  let samples: Vec<f32> = Vec::with_capacity(0);
  let audio = AudioBuffer::from_samples(44100, 2, samples);
  let mut result: AudioResult<()>;

  let err_cases =
    vec![
      // Path::extension cases
      //  - No file name
      //  - No embedded `.`
      //  - File name begins with `.` and has no other `.`s within
      "tests/results/",
      "tests/results/tmp",
      "tests/results/.wav",
      // Unsupported audio or file format
      "tests/results/tmp.mp2"
    ];
  for path in err_cases.iter() {
    result = audio::save(Path::new(path), &audio);
    assert!(result.is_err());
  }

  // Valid paths
  result = audio::save(Path::new("tests/results/tmp.wav"), &audio);
  assert!(result.is_ok());
  result = audio::save(Path::new("tests/results/tmp.aiff"), &audio);
  assert!(result.is_ok());
}

#[test]
fn save_as() {
  use audio::Codec::{LPCM_F32_BE, LPCM_F32_LE};

  let samples: Vec<f32> = Vec::with_capacity(0);
  let audio = AudioBuffer::from_samples(44100, 2, samples);
  let mut result: AudioResult<()>;

  let err_cases =
    vec![
      // Path::extension cases
      //  - No file name
      //  - No embedded `.`
      //  - File name begins with `.` and has no other `.`s within
      "tests/results/",
      "tests/results/tmp",
      "tests/results/.wav",
      // Unsupported audio format
      "tests/results/tmp.mp2"
    ];
  for path in err_cases.iter() {
    result = audio::save_as(Path::new(path), &audio, LPCM_F32_LE);
    assert!(result.is_err());
  }

  // Valid paths
  result = audio::save_as(Path::new("tests/results/tmp.wav"), &audio, LPCM_F32_LE);
  assert!(result.is_ok());
  result = audio::save_as(Path::new("tests/results/tmp.aiff"), &audio, LPCM_F32_BE);
  assert!(result.is_ok());
}
