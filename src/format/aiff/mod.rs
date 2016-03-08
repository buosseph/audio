//! The Audio Interchange File Format
//!
//! AIFF files use the Interchange File Format (IFF), a generic file container
//! format that uses chunks to store data. All bytes are stored in big-endian
//! format.
//!
//! References
//! - [McGill University](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/AIFF/AIFF.html)
//! - [AIFF Spec](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/AIFF/Docs/AIFF-1.3.pdf)
//! - [AIFF/AIFFC Spec from Apple](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/AIFF/Docs/MacOS_Sound-extract.pdf)

mod chunks;
mod read;
mod write;

pub use self::read::read   as read;
pub use self::write::write as write;

/// Aiff/Aifc chunk identifiers.
const FORM: &'static [u8; 4] = b"FORM";
const AIFF: &'static [u8; 4] = b"AIFF";
const AIFC: &'static [u8; 4] = b"AIFC";
const FVER: &'static [u8; 4] = b"FVER";
const COMM: &'static [u8; 4] = b"COMM";
const SSND: &'static [u8; 4] = b"SSND";

/// Aiff-C Version 1 timestamp for the FVER chunk.
#[allow(dead_code)]
const AIFC_VERSION_1: u32 = 0xA2805140;


#[cfg(test)]
mod io {
  mod aiff {
    use std::fs::File;
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use ::audio;
    use ::codecs::Codec::*;

    #[test]
    fn i8_eq() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      let files = vec![
        "mono440-i8-44100.aiff",
        "stereo440-i8-44100.aiff"
      ];

      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i8.aiff");
        assert!(audio::save_as(&write_path, &audio, LPCM_I8).is_ok());

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
      path.push("aiff");
      path.push("empty.aiff");
      let files = vec![
        "mono440-i16-44100.aiff",
        "stereo440-i16-44100.aiff"
      ];
      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i16.aiff");
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
      path.push("aiff");
      path.push("empty.aiff");
      let files = vec![
        "mono440-i24-44100.aiff",
        "stereo440-i24-44100.aiff"
      ];
      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i24.aiff");
        assert!(audio::save_as(&write_path, &audio, LPCM_I24_BE).is_ok());

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
      path.push("aiff");
      path.push("empty.aiff");
      let files = vec![
        "mono440-i32-44100.aiff",
        "stereo440-i32-44100.aiff"
      ];
      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i32.aiff");
        assert!(audio::save_as(&write_path, &audio, LPCM_I32_BE).is_ok());

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
  }

  mod aifc {
    use std::fs::File;
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use ::audio;
    use ::codecs::Codec::*;

    #[test]
    fn read_aifc_format() {
      let aiff_path = Path::new("tests/aiff/M1F1-int16-AFsp.aif");
      let aifc_path = Path::new("tests/aiff/M1F1-int16C-AFsp.aif");
      // Compare aifc to aiff file with same data.
      let aiff = audio::open(&aiff_path).unwrap();
      let aifc = audio::open(&aifc_path).unwrap();
      assert_eq!(aiff.channels,      aifc.channels);
      assert_eq!(aiff.sample_rate,   aifc.sample_rate);
      assert_eq!(aiff.samples.len(), aifc.samples.len());
      for (aiff_sample, aifc_sample) in
          aiff.samples.iter().zip(&aifc.samples) {
        assert_eq!(aiff_sample, aifc_sample);
      }
    }

    #[test]
    fn u8_eq() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      path.set_file_name("mono440-u8-odd-bytes.aiff");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_u8.aiff");
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

    #[test]
    fn f32() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      path.set_file_name("M1F1-float32C-AFsp.aif");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_f32.aiff");
      assert!(audio::save_as(&write_path, &audio, LPCM_F32_BE).is_ok());

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
          read_file.bytes().skip(154)
          .zip(written_file.bytes().skip(72)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }

    #[test]
    fn f64() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      path.set_file_name("M1F1-float64C-AFsp.aif");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_f64.aiff");
      assert!(audio::save_as(&write_path, &audio, LPCM_F64_BE).is_ok());

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
          read_file.bytes().skip(154)
          .zip(written_file.bytes().skip(72)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }

    #[test]
    fn ulaw() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      path.set_file_name("M1F1-mulawC-AFsp.aif");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_ulaw.aiff");
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
          read_file.bytes().skip(146)
          .zip(written_file.bytes().skip(64)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }

    #[test]
    fn alaw() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      path.set_file_name("M1F1-AlawC-AFsp.aif");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_alaw.aiff");
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
          read_file.bytes().skip(146)
          .zip(written_file.bytes().skip(64)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }
  }
}
