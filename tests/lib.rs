//#![feature(test)]
extern crate audio;
//extern crate test;

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
fn test_u8_read_write_eq() {
  use std::fs::File;
  use std::io::Read;
  use std::path::{Path, PathBuf};
  use audio::AudioBuffer;

  let mut path = PathBuf::from("tests");
  path.push("wav");
  path.push("empty.wav");
  let files = vec![
    "mono440-u8-44100.wav",
    "stereo440-u8-44100.wav",
  ];

  for file in files.iter() {
    path.set_file_name(file);
    println!("{:?}", path.as_path());
    let audio = audio::open(path.as_path()).unwrap(); //.ok().expect("Couldn't open file");
    let total_samples = audio.samples.len();
    let channels = audio.channels;
    let bit_rate = audio.bit_rate;
    let sample_rate = audio.sample_rate;
    let sample_order = audio.order;

    let write_loc = Path::new("tmp.wav");
    let written = audio::save(&write_loc, &audio);
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
fn test_i16_read_write_eq() {
  use std::fs::File;
  use std::io::Read;
  use std::path::{Path, PathBuf};
  use audio::AudioBuffer;

  let mut path = PathBuf::from("tests");
  path.push("wav");
  path.push("empty.wav");
  let files = vec![
    "i16-pcm-mono.wav",
    "i16-pcm-stereo.wav",
    "Warrior Concerto - no meta.wav",
  ];

  for file in files.iter() {
    path.set_file_name(file);
    println!("{:?}", path.as_path());
    let audio = audio::open(path.as_path()).unwrap(); //.ok().expect("Couldn't open file");
    let total_samples = audio.samples.len();
    let channels = audio.channels;
    let bit_rate = audio.bit_rate;
    let sample_rate = audio.sample_rate;
    let sample_order = audio.order;

    let write_loc = Path::new("tmp.wav");
    let written = audio::save(&write_loc, &audio);
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

/*
#[bench]
fn bench_read_wav(b: &mut test::Bencher) {
  b.iter(|| {
    use std::path::Path;
    audio::open(&Path::new("tests/wav/Warrior Concerto - no meta.wav"));
  });
}
*/
