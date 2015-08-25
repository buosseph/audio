extern crate audio;

#[cfg(test)]
mod api {
  use super::audio;
  use std::path::Path;
  use audio::{AudioBuffer, AudioResult};

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
}

#[cfg(test)]
mod transcoding {
  use super::audio;
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
}

