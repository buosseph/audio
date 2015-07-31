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

mod container;
mod chunks;
pub mod decoder;
pub mod encoder;

pub use aiff::decoder::Decoder as Decoder;
pub use aiff::encoder::Encoder as Encoder;

/// AIFF/AIFC chunk identifiers.
const FORM: &'static [u8; 4] = b"FORM";
const AIFF: &'static [u8; 4] = b"AIFF";
const AIFC: &'static [u8; 4] = b"AIFC";
const FVER: &'static [u8; 4] = b"FVER";
const COMM: &'static [u8; 4] = b"COMM";
const SSND: &'static [u8; 4] = b"SSND";

/// AIFF-C Version 1 timestamp for the FVER chunk.
const AIFC_VERSION1: u32 = 0xA2805140;

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::Read;
  use std::path::{Path, PathBuf};
  use ::audio;
  use ::buffer::AudioBuffer;
  use ::codecs::Codec::*;

  #[test]
  fn i8_aiff_eq() {
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
      let audio = audio::open(path.as_path()).unwrap_or_else(
        |err| {println!("{:?}", err); panic!();}
      );
      let total_samples = audio.samples.len();
      let channels = audio.channels;
      let bit_rate = audio.bit_rate;
      let sample_rate = audio.sample_rate;
      let sample_order = audio.order;

      let write_loc = Path::new("tests/results/tmp_i8.aiff");
      let written = audio::save_as(&write_loc, &audio, LPCM_I8);
      println!("{:?}", written);
      assert!(written.is_ok());
      let verify: AudioBuffer = audio::open(&write_loc).unwrap();
      assert_eq!(total_samples, verify.samples.len());
      assert_eq!(channels, verify.channels);
      assert_eq!(bit_rate, verify.bit_rate);
      assert_eq!(sample_rate, verify.sample_rate);
      assert_eq!(sample_order, verify.order);

      // File sizes are the same
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_loc).unwrap();
      let read_meta = read_file.metadata().unwrap();
      let write_meta = written_file.metadata().unwrap();
      assert_eq!(read_meta.len(), write_meta.len());

      // Bytes are the same
      let mut written_file_bytes = written_file.bytes();
      for byte in read_file.bytes() {
        assert_eq!(
          byte.ok().expect("Error reading byte from read file"),
          written_file_bytes.next().expect("End of file").ok().expect("Error reading byte from written file")
        );
      }
    }
  }

  #[test]
  fn i16_aiff_eq() {
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
      let audio = audio::open(path.as_path()).unwrap_or_else(
        |err| {println!("{:?}", err); panic!();}
      );
      let total_samples = audio.samples.len();
      let channels = audio.channels;
      let bit_rate = audio.bit_rate;
      let sample_rate = audio.sample_rate;
      let sample_order = audio.order;

      let write_loc = Path::new("tests/results/tmp_i16.aiff");
      let written = audio::save_as(&write_loc, &audio, LPCM_I16_BE);
      println!("{:?}", written);
      assert!(written.is_ok());
      let verify: AudioBuffer = audio::open(&write_loc).unwrap();
      assert_eq!(total_samples, verify.samples.len());
      assert_eq!(channels, verify.channels);
      assert_eq!(bit_rate, verify.bit_rate);
      assert_eq!(sample_rate, verify.sample_rate);
      assert_eq!(sample_order, verify.order);

      // File sizes are the same
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_loc).unwrap();
      let read_meta = read_file.metadata().unwrap();
      let write_meta = written_file.metadata().unwrap();
      assert_eq!(read_meta.len(), write_meta.len());

      // Bytes are the same
      let mut written_file_bytes = written_file.bytes();
      for byte in read_file.bytes() {
        assert_eq!(
          byte.ok().expect("Error reading byte from read file"),
          written_file_bytes.next().expect("End of file").ok().expect("Error reading byte from written file")
        );
      }
    }
  }

  #[test]
  fn i24_aiff_eq() {
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
      let audio = audio::open(path.as_path()).unwrap_or_else(
        |err| {println!("{:?}", err); panic!();}
      );
      let total_samples = audio.samples.len();
      let channels = audio.channels;
      let bit_rate = audio.bit_rate;
      let sample_rate = audio.sample_rate;
      let sample_order = audio.order;

      let write_loc = Path::new("tests/results/tmp_i24.aiff");
      let written = audio::save_as(&write_loc, &audio, LPCM_I24_BE);
      println!("{:?}", written);
      assert!(written.is_ok());
      let verify: AudioBuffer = audio::open(&write_loc).unwrap();
      assert_eq!(total_samples, verify.samples.len());
      assert_eq!(channels, verify.channels);
      assert_eq!(bit_rate, verify.bit_rate);
      assert_eq!(sample_rate, verify.sample_rate);
      assert_eq!(sample_order, verify.order);

      // File sizes are the same
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_loc).unwrap();
      let read_meta = read_file.metadata().unwrap();
      let write_meta = written_file.metadata().unwrap();
      assert_eq!(read_meta.len(), write_meta.len());

      // Bytes are the same
      let mut written_file_bytes = written_file.bytes();
      for byte in read_file.bytes() {
        assert_eq!(
          byte.ok().expect("Error reading byte from read file"),
          written_file_bytes.next().expect("End of file").ok().expect("Error reading byte from written file")
        );
      }
    }
  }

  #[test]
  fn i32_aiff_eq() {
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
      let audio = audio::open(path.as_path()).unwrap_or_else(
        |err| {println!("{:?}", err); panic!();}
      );
      let total_samples = audio.samples.len();
      let channels = audio.channels;
      let bit_rate = audio.bit_rate;
      let sample_rate = audio.sample_rate;
      let sample_order = audio.order;

      let write_loc = Path::new("tests/results/tmp_i32.aiff");
      let written = audio::save_as(&write_loc, &audio, LPCM_I32_BE);
      println!("{:?}", written);
      assert!(written.is_ok());
      let verify: AudioBuffer = audio::open(&write_loc).unwrap();
      assert_eq!(total_samples, verify.samples.len());
      assert_eq!(channels, verify.channels);
      assert_eq!(bit_rate, verify.bit_rate);
      assert_eq!(sample_rate, verify.sample_rate);
      assert_eq!(sample_order, verify.order);

      // File sizes are the same
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_loc).unwrap();
      let read_meta = read_file.metadata().unwrap();
      let write_meta = written_file.metadata().unwrap();
      assert_eq!(read_meta.len(), write_meta.len());

      // Bytes are the same
      let mut written_file_bytes = written_file.bytes();
      for byte in read_file.bytes() {
        assert_eq!(
          byte.ok().expect("Error reading byte from read file"),
          written_file_bytes.next().expect("End of file").ok().expect("Error reading byte from written file")
        );
      }
    }
  }

  #[test]
  fn aiff_mod_aifc_eq() {
    let mut path = PathBuf::from("tests");
    path.push("aiff");
    path.push("empty.aiff");
    path.set_file_name("M1F1-int16-AFsp.aif");
    println!("{:?}", path.as_path());
    let aiff =
      match audio::open(path.as_path()) {
        Ok(a) => a,
        Err(e) => panic!(format!("Error: {:?}", e))
      };
    let total_samples = aiff.samples.len();
    let channels      = aiff.channels;
    let bit_rate      = aiff.bit_rate;
    let sample_rate   = aiff.sample_rate;
    let sample_order  = aiff.order;
    // Compare to aifc file with same data
    path.set_file_name("M1F1-int16C-AFsp.aif");
    println!("{:?}", path.as_path());
    let aifc =
      match audio::open(path.as_path()) {
        Ok(a) => a,
        Err(e) => panic!(format!("Error: {:?}", e))
      };
    assert_eq!(aifc.channels, channels);
    assert_eq!(aifc.bit_rate, bit_rate);
    assert_eq!(aifc.sample_rate, sample_rate);
    assert_eq!(aifc.order, sample_order);
    assert_eq!(aifc.samples.len(), total_samples);
    for (aiff_sample, aifc_sample) in aiff.samples.iter().zip(&aifc.samples) {
      assert_eq!(aiff_sample, aifc_sample);
    }
  }

  #[test]
  fn u8_aifc_eq() {
    let mut path = PathBuf::from("tests");
    path.push("aiff");
    path.push("empty.aiff");
    path.set_file_name("mono440-u8-odd-bytes.aiff");
    println!("{:?}", path.as_path());
    let aifc =
      match audio::open(path.as_path()) {
        Ok(a) => a,
        Err(e) => panic!(format!("Error: {:?}", e))
      };
    let total_samples = aifc.samples.len();
    let channels      = aifc.channels;
    let bit_rate      = aifc.bit_rate;
    let sample_rate   = aifc.sample_rate;
    let sample_order  = aifc.order;
    println!("Read file");
    // Write to file.
    let write_path = Path::new("tests/results/tmp_u8.aiff");
    let written = audio::save_as(&write_path, &aifc, LPCM_U8);
    assert!(written.is_ok());
    println!("File written");
    // Read written file and verify read audio is the same.
    let verify: AudioBuffer = audio::open(&write_path).unwrap();
    assert_eq!(channels,      verify.channels);
    assert_eq!(bit_rate,      verify.bit_rate);
    assert_eq!(sample_rate,   verify.sample_rate);
    assert_eq!(sample_order,  verify.order);
    assert_eq!(total_samples, verify.samples.len());
    for (inital_sample, written_sample) in aifc.samples.iter().zip(&verify.samples) {
      assert_eq!(inital_sample, written_sample);
    }
    println!("Read new file");
    // File sizes are the same.
    let read_file     = File::open(path.as_path()).unwrap();
    let written_file  = File::open(&write_path).unwrap();
    let read_meta     = read_file.metadata().unwrap();
    let write_meta    = written_file.metadata().unwrap();
    assert_eq!(read_meta.len(), write_meta.len());
    // Assert every byte is the same between the two files.
    for (inital_byte, written_byte) in read_file.bytes().zip(written_file.bytes()) {
      assert_eq!(inital_byte.ok(), written_byte.ok());
    }
  }

  #[test]
  fn f32_aifc() {
    let mut path = PathBuf::from("tests");
    path.push("aiff");
    path.push("empty.aiff");
    path.set_file_name("M1F1-float32C-AFsp.aif");
    println!("{:?}", path.as_path());
    let aifc =
      match audio::open(path.as_path()) {
        Ok(a) => a,
        Err(e) => panic!(format!("Error: {:?}", e))
      };
    let total_samples = aifc.samples.len();
    let channels      = aifc.channels;
    let bit_rate      = aifc.bit_rate;
    let sample_rate   = aifc.sample_rate;
    let sample_order  = aifc.order;
    println!("Read file");
    // Write to file.
    let write_path = Path::new("tests/results/tmp_f32.aiff");
    let written = audio::save_as(&write_path, &aifc, LPCM_F32_BE);
    assert!(written.is_ok());
    println!("File written");
    // Read written file and verify read audio is the same.
    let verify: AudioBuffer = audio::open(&write_path).unwrap();
    assert_eq!(channels,      verify.channels);
    assert_eq!(bit_rate,      verify.bit_rate);
    assert_eq!(sample_rate,   verify.sample_rate);
    assert_eq!(sample_order,  verify.order);
    assert_eq!(total_samples, verify.samples.len());
    for (inital_sample, written_sample) in aifc.samples.iter().zip(&verify.samples) {
      assert_eq!(inital_sample, written_sample);
    }
    println!("Read new file");
    // File sizes are not the same.
    let read_file     = File::open(path.as_path()).unwrap();
    let written_file  = File::open(&write_path).unwrap();
    // Assert every byte in the SSND chunk is the same between the two files.
    for (inital_byte, written_byte) in read_file.bytes().skip(154).zip(written_file.bytes().skip(72)) {
      assert_eq!(inital_byte.ok(), written_byte.ok());
    }
  }

  #[test]
  fn f64_aifc() {
    let mut path = PathBuf::from("tests");
    path.push("aiff");
    path.push("empty.aiff");
    path.set_file_name("M1F1-float64C-AFsp.aif");
    println!("{:?}", path.as_path());
    let aifc =
      match audio::open(path.as_path()) {
        Ok(a) => a,
        Err(e) => panic!(format!("Error: {:?}", e))
      };
    let total_samples = aifc.samples.len();
    let channels      = aifc.channels;
    let bit_rate      = aifc.bit_rate;
    let sample_rate   = aifc.sample_rate;
    let sample_order  = aifc.order;
    println!("Read file");
    // Write to file.
    let write_path = Path::new("tests/results/tmp_f64.aiff");
    let written = audio::save_as(&write_path, &aifc, LPCM_F64_BE);
    assert!(written.is_ok());
    println!("File written");
    // Read written file and verify read audio is the same.
    let verify: AudioBuffer = audio::open(&write_path).unwrap();
    assert_eq!(channels,      verify.channels);
    assert_eq!(bit_rate,      verify.bit_rate);
    assert_eq!(sample_rate,   verify.sample_rate);
    assert_eq!(sample_order,  verify.order);
    assert_eq!(total_samples, verify.samples.len());
    for (inital_sample, written_sample) in aifc.samples.iter().zip(&verify.samples) {
      assert_eq!(inital_sample, written_sample);
    }
    println!("Read new file");
    // File sizes are not the same.
    let read_file     = File::open(path.as_path()).unwrap();
    let written_file  = File::open(&write_path).unwrap();
    // Assert every byte in the SSND chunk is the same between the two files.
    for (inital_byte, written_byte) in read_file.bytes().skip(154).zip(written_file.bytes().skip(72)) {
      assert_eq!(inital_byte.ok(), written_byte.ok());
    }
  }
}
