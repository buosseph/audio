#![feature(test)]
extern crate audio;
extern crate test;

use std::path::Path;
use test::Bencher;

#[bench]
fn empty(b: &mut Bencher) {
  b.iter(|| 1);
}

#[bench]
fn read_wave_track(b: &mut Bencher) {
  b.iter(|| {
    audio::open(&Path::new("tests/wav/Warrior Concerto - no meta.wav"))
      .ok().expect("Couldn't read file");
  });
}

#[bench]
fn write_wave_track(b: &mut Bencher) {
  let audio = audio::open(
    &Path::new("tests/wav/Warrior Concerto - no meta.wav")
  ).ok().expect("Couldn't read file");
  b.iter(|| {
    audio::save(
      &Path::new("tests/results/tmp_i16.wav"),
      &audio
    ).ok().expect("Couldn't write file");
  });
}

#[bench]
fn read_aiff_track(b: &mut Bencher) {
  b.iter(|| {
    audio::open(&Path::new("tests/aiff/Warrior Concerto - no meta.aiff"))
      .ok().expect("Couldn't read file");
  });
}

#[bench]
fn write_aiff_track(b: &mut Bencher) {
  let audio = audio::open(
    &Path::new("tests/aiff/Warrior Concerto - no meta.aiff")
  ).ok().expect("Couldn't read file");
  b.iter(|| {
    audio::save(
      &Path::new("tests/results/tmp_i16.aiff"),
      &audio
    ).ok().expect("Couldn't write file");
  });
}


/*
 *  The benchmarks below are just to have some comparision
 *  with strictly reading from a File or BufReader.
 *
 *  One set of bench results for a quick reference
 *
 *  test buf_reader       ... bench: 104,821,811 ns/iter (+/- 27,349,621)
 *  test empty            ... bench:           0 ns/iter (+/- 0)
 *  test file             ... bench:  64,956,282 ns/iter (+/- 25,325,489)
 *  test read_aiff_track  ... bench: 327,196,191 ns/iter (+/- 83,703,581)
 *  test read_wave_track  ... bench: 356,682,279 ns/iter (+/- 55,742,915)
 *  test write_aiff_track ... bench: 843,659,323 ns/iter (+/- 519,412,469)
 *  test write_wave_track ... bench: 709,889,187 ns/iter (+/- 157,425,662)
 *
 */

#[bench]
fn file(b: &mut Bencher) {
  use std::fs::File;
  use std::io::Read;

  b.iter(|| {
    let mut fs = File::open(&Path::new("tests/aiff/Warrior Concerto - no meta.aiff"))
      .ok().expect("Couldn't read file");
    let meta = fs.metadata().ok().expect("Couldn't read metadata");
    let size = meta.len() as usize;
    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    for _ in 0..buffer.capacity() { buffer.push(0u8); }
    fs.read(&mut buffer).ok().expect("Error reading file to buffer");
  });
}

#[bench]
fn buf_reader(b: &mut Bencher) {
  use std::fs::File;
  use std::io::{BufReader, Read};

  b.iter(|| {
    let fs = File::open(&Path::new("tests/aiff/Warrior Concerto - no meta.aiff"))
      .ok().expect("Couldn't read file");
    let meta = fs.metadata().ok().expect("Couldn't read metadata");
    let size = meta.len() as usize;
    let mut reader = BufReader::with_capacity(size, fs);
    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    for _ in 0..buffer.capacity() { buffer.push(0u8); }
    reader.read(&mut buffer).ok().expect("Error reading file to buffer");
  });
}