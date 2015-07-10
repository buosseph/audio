//! The Audio Interchange File Format
//!
//! AIFF files use the Interchange File Format (IFF), a generic
//! file container format that uses chunks to store data.

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

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::Read;
  use std::path::{Path, PathBuf};
  use ::audio;
  use ::buffer::AudioBuffer;

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
      let written = audio::save(&write_loc, &audio);
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
}
