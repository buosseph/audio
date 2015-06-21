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