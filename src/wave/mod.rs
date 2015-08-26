//! The Waveform Audio File Format
//!
//! WAVE files use the Resource Interchange File Format (RIFF), a generic
//! file container format that uses chunks to store data. All integers are stored
//! in little-endian format, but identifier bytes are in ASCII, big-endian.
//!
//! References
//! - [McGill University](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html)
//! - [WAVE Spec](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/Docs/riffmci.pdf)
//! - [ksmedia.h](http://www-mmsp.ece.mcgill.ca/documents/audioformats/wave/Docs/ksmedia.h)

mod container;
mod chunks;
pub mod decoder;
pub mod encoder;

pub use wave::decoder::Decoder as Decoder;
pub use wave::encoder::Encoder as Encoder;

/// WAVE chunk identifiers.
const RIFF: &'static [u8; 4] = b"RIFF";
const WAVE: &'static [u8; 4] = b"WAVE";
const FMT:  &'static [u8; 4] = b"fmt ";
const DATA: &'static [u8; 4] = b"data";
const FACT: &'static [u8; 4] = b"fact";

#[cfg(test)]
mod io {
  mod wave {
    use std::fs::File;
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use ::audio;
    use ::codecs::Codec::*;

    #[test]
    fn u8_eq() {
      let mut path = PathBuf::from("tests");
      path.push("wav");
      path.push("empty.wav");
      let files = vec![
        "mono440-u8-44100.wav",
        "mono440-u8-odd-bytes.wav",
        "stereo440-u8-44100.wav"
      ];

      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_u8.wav");
        assert!(audio::save_as(&write_path, &audio, LPCM_U8).is_ok());

        let verify = audio::open(&write_path).unwrap();
        assert_eq!(audio.channels,      verify.channels);
        assert_eq!(audio.sample_rate,   verify.sample_rate);
        assert_eq!(audio.samples.len(), verify.samples.len());
        for (inital_sample, written_sample) in
            audio.samples.iter().zip(&verify.samples) {
          assert_eq!(inital_sample, written_sample);
        }

        // Assert every byte is the same between the two files.
        let read_file = File::open(path.as_path()).unwrap();
        let written_file = File::open(&write_path).unwrap();
        for (inital_byte, written_byte) in
            read_file.bytes().zip(written_file.bytes()) {
          assert_eq!(inital_byte.ok(), written_byte.ok());
        }
      }
    }

    #[test]
    fn i16_eq() {
      let mut path = PathBuf::from("tests");
      path.push("wav");
      path.push("empty.wav");
      let files = vec![
        "i16-pcm-mono.wav",
        "i16-pcm-stereo.wav"
      ];

      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i16.wav");
        assert!(audio::save(&write_path, &audio).is_ok());

        let verify = audio::open(&write_path).unwrap();
        assert_eq!(audio.channels,      verify.channels);
        assert_eq!(audio.sample_rate,   verify.sample_rate);
        assert_eq!(audio.samples.len(), verify.samples.len());
        for (inital_sample, written_sample) in
            audio.samples.iter().zip(&verify.samples) {
          assert_eq!(inital_sample, written_sample);
        }

        // Assert every byte is the same between the two files.
        let read_file = File::open(path.as_path()).unwrap();
        let written_file = File::open(&write_path).unwrap();
        for (inital_byte, written_byte) in
            read_file.bytes().zip(written_file.bytes()) {
          assert_eq!(inital_byte.ok(), written_byte.ok());
        }
      }
    }

    #[test]
    fn i24_eq() {
      let mut path = PathBuf::from("tests");
      path.push("wav");
      path.push("empty.wav");
      let files = vec![
        "mono440-i24-44100.wav",
        "stereo440-i24-44100.wav"
      ];

      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i24.wav");
        assert!(audio::save_as(&write_path, &audio, LPCM_I24_LE).is_ok());

        let verify = audio::open(&write_path).unwrap();
        assert_eq!(audio.channels,      verify.channels);
        assert_eq!(audio.sample_rate,   verify.sample_rate);
        assert_eq!(audio.samples.len(), verify.samples.len());
        for (inital_sample, written_sample) in
            audio.samples.iter().zip(&verify.samples) {
          assert_eq!(inital_sample, written_sample);
        }

        // Assert every byte is the same between the two files.
        let read_file = File::open(path.as_path()).unwrap();
        let written_file = File::open(&write_path).unwrap();
        for (inital_byte, written_byte) in
            read_file.bytes().zip(written_file.bytes()) {
          assert_eq!(inital_byte.ok(), written_byte.ok());
        }
      }
    }

    #[test]
    fn i32_eq() {
      let mut path = PathBuf::from("tests");
      path.push("wav");
      path.push("empty.wav");
      let files = vec![
        "mono440-i32-44100.wav",
        "stereo440-i32-44100.wav"
      ];

      for file in files.iter() {
        path.set_file_name(file);
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i32.wav");
        assert!(audio::save_as(&write_path, &audio, LPCM_I32_LE).is_ok());

        let verify = audio::open(&write_path).unwrap();
        assert_eq!(audio.channels,      verify.channels);
        assert_eq!(audio.sample_rate,   verify.sample_rate);
        assert_eq!(audio.samples.len(), verify.samples.len());
        for (inital_sample, written_sample) in
            audio.samples.iter().zip(&verify.samples) {
          assert_eq!(inital_sample, written_sample);
        }

        // Assert every byte is the same between the two files.
        let read_file = File::open(path.as_path()).unwrap();
        let written_file = File::open(&write_path).unwrap();
        for (inital_byte, written_byte) in
            read_file.bytes().zip(written_file.bytes()) {
          assert_eq!(inital_byte.ok(), written_byte.ok());
        }
      }
    }

    #[test]
    fn f32() {
      let mut path = PathBuf::from("tests");
      path.push("wav");
      path.push("empty.wav");
      path.set_file_name("M1F1-float32-AFsp.wav");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_f32.wav");
      assert!(audio::save_as(&write_path, &audio, LPCM_F32_LE).is_ok());

      let verify = audio::open(&write_path).unwrap();
      assert_eq!(audio.channels,      verify.channels);
      assert_eq!(audio.sample_rate,   verify.sample_rate);
      assert_eq!(audio.samples.len(), verify.samples.len());
      for (inital_sample, written_sample) in
          audio.samples.iter().zip(&verify.samples) {
        assert_eq!(inital_sample, written_sample);
      }

      // Assert every byte, excluding metadata, is the same between the two files.
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_path).unwrap();
      for (inital_byte, written_byte) in
          read_file.bytes().skip(12).take(187990)
          .zip(written_file.bytes().skip(12).take(187990)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }

    #[test]
    fn f64() {
      let mut path = PathBuf::from("tests");
      path.push("wav");
      path.push("empty.wav");
      path.set_file_name("M1F1-float64-AFsp.wav");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_f64.wav");
      assert!(audio::save_as(&write_path, &audio, LPCM_F64_LE).is_ok());

      let verify = audio::open(&write_path).unwrap();
      assert_eq!(audio.channels,      verify.channels);
      assert_eq!(audio.sample_rate,   verify.sample_rate);
      assert_eq!(audio.samples.len(), verify.samples.len());
      for (inital_sample, written_sample) in
          audio.samples.iter().zip(&verify.samples) {
        assert_eq!(inital_sample, written_sample);
      }

      // Assert every byte, excluding metadata, is the same between the two files.
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_path).unwrap();
      for (inital_byte, written_byte) in
          read_file.bytes().skip(12).take(875934)
          .zip(written_file.bytes().skip(12).take(875934)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }

    #[test]
    fn alaw() {
      let mut path = PathBuf::from("tests");
      path.push("wav");
      path.push("empty.wav");
      path.set_file_name("M1F1-Alaw-AFsp.wav");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_alaw.wav");
      assert!(audio::save_as(&write_path, &audio, G711_ALAW).is_ok());

      let verify = audio::open(&write_path).unwrap();
      assert_eq!(audio.channels,      verify.channels);
      assert_eq!(audio.sample_rate,   verify.sample_rate);
      assert_eq!(audio.samples.len(), verify.samples.len());
      for (inital_sample, written_sample) in
          audio.samples.iter().zip(&verify.samples) {
        assert_eq!(inital_sample, written_sample);
      }

      // Assert every byte, excluding metadata, is the same between the two files.
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_path).unwrap();
      for (inital_byte, written_byte) in
          read_file.bytes().skip(50).take(46994)
          .zip(written_file.bytes().skip(50).take(46994)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }

    #[test]
    fn ulaw() {
      let mut path = PathBuf::from("tests");
      path.push("wav");
      path.push("empty.wav");
      path.set_file_name("M1F1-mulaw-AFsp.wav");      
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_ulaw.wav");
      assert!(audio::save_as(&write_path, &audio, G711_ULAW).is_ok());

      let verify = audio::open(&write_path).unwrap();
      assert_eq!(audio.channels,      verify.channels);
      assert_eq!(audio.sample_rate,   verify.sample_rate);
      assert_eq!(audio.samples.len(), verify.samples.len());
      for (inital_sample, written_sample) in
          audio.samples.iter().zip(&verify.samples) {
        assert_eq!(inital_sample, written_sample);
      }

      // Assert every byte, excluding metadata, is the same between the two files.
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_path).unwrap();
      for (inital_byte, written_byte) in
          read_file.bytes().skip(50).take(46994)
          .zip(written_file.bytes().skip(50).take(46994)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }
  }
  mod wavex {
    use std::path::Path;
    use ::audio;

    #[test]
    fn read_wave_extensible_format() {
      let wavex_path = Path::new("tests/wav/M1F1-int16WE-AFsp.wav");
      let wave_path = Path::new("tests/wav/M1F1-int16-AFsp.wav");
      // Compare wavex to wave file with same data.
      let wave = audio::open(&wave_path).unwrap();
      let wavex = audio::open(&wavex_path).unwrap();
      assert_eq!(wave.channels,      wavex.channels);
      assert_eq!(wave.sample_rate,   wavex.sample_rate);
      assert_eq!(wave.samples.len(), wavex.samples.len());
      for (wave_sample, wavex_sample) in
          wave.samples.iter().zip(&wavex.samples) {
        assert_eq!(wave_sample, wavex_sample);
      }
    }
  }
}
