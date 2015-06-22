#![feature(test)]
extern crate audio;
extern crate test;

use std::path::Path;
use test::Bencher;

#[bench]
fn read_wave_track(b: &mut Bencher) {
  b.iter(|| {
    audio::open(&Path::new("tests/wav/Warrior Concerto - no meta.wav"))
      .ok().expect("Couldn't read file");
  });
}

#[bench]
fn read_aiff_track(b: &mut Bencher) {
  b.iter(|| {
    audio::open(&Path::new("tests/aiff/Warrior Concerto - no meta.aiff"))
      .ok().expect("Couldn't read file");
  });
}

/*
 *  The benchmarks below are just to have some comparision
 *  with strictly reading from a File or BufReader.
 *
 *  One set of bench results for a quick reference
 *
 *  test buf_reader      ... bench: 102,721,113 ns/iter (+/- 23,721,150)
 *  test file            ... bench:  73,460,776 ns/iter (+/- 36,662,689)
 *  test read_aiff_track ... bench: 279,125,245 ns/iter (+/- 35,967,603)
 *  test read_wave_track ... bench: 326,092,886 ns/iter (+/- 69,465,665)
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