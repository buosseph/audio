extern crate audio;

use std::path::PathBuf;
use audio::FromSample;
use audio::Sample;

#[test]
fn wave() {
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
    "M1F1-mulaw-AFsp.wav",
    // wave extensible formats
    "M1F1-uint8WE-AFsp.wav",
    "M1F1-int16WE-AFsp.wav",
    "M1F1-int24WE-AFsp.wav",
    "M1F1-int32WE-AFsp.wav",
    "M1F1-float32WE-AFsp.wav",
    "M1F1-float64WE-AFsp.wav",
    "M1F1-AlawWE-AFsp.wav",
    "M1F1-mulawWE-AFsp.wav",
  ];
  for file in files.iter() {
    path_buf.set_file_name(file);
    if let Ok(audio) = audio::open(path_buf.as_path()) {
      samples.push(audio.samples[100]);
    }
  }
  let sample_8_bit = i8::from_sample(samples[0]);
  for s in samples.iter().skip(1) {
    assert_eq!(sample_8_bit, i8::from_sample(*s));
  }
}

#[test]
fn aiff() {
  let mut samples: Vec<Sample> = Vec::new();
  let mut path_buf = PathBuf::from("tests");
  path_buf.push("aiff");
  path_buf.push("empty.aiff");
  let files = vec![
    "M1F1-int8-AFsp.aif",
    "M1F1-int16-AFsp.aif",
    "M1F1-int24-AFsp.aif",
    "M1F1-int32-AFsp.aif",
    // aifc formats
    "M1F1-int8C-AFsp.aif",
    "M1F1-int16C-AFsp.aif",
    "M1F1-int24C-AFsp.aif",
    "M1F1-int32C-AFsp.aif",
    "M1F1-float32C-AFsp.aif",
    "M1F1-float64C-AFsp.aif",
    "M1F1-AlawC-AFsp.aif",
    "M1F1-mulawC-AFsp.aif"
  ];
  for file in files.iter() {
    path_buf.set_file_name(file);
    if let Ok(audio) = audio::open(path_buf.as_path()) {
      samples.push(audio.samples[100]);
    }
  }
  let sample_8_bit = i8::from_sample(samples[0]);
  for s in samples.iter().skip(1) {
    assert_eq!(sample_8_bit, i8::from_sample(*s));
  }
}
