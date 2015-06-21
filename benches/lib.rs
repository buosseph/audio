#![feature(test)]
extern crate audio;
extern crate test;

use std::path::Path;
use test::Bencher;

#[bench]
fn read_wave_track(b: &mut Bencher) {
  b.iter(|| {
    audio::open(&Path::new("tests/wav/Warrior Concerto - no meta.wav"));
  });
}