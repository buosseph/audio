#![feature(test)]
extern crate audio;
extern crate byteorder;
extern crate test;

use std::path::Path;
use test::Bencher;

/*
  A set of bench results for quick reference
  test read_aiff_track                 ... bench: 167,915,163 ns/iter (+/- 4,564,825)
  test read_wave_track                 ... bench: 181,276,731 ns/iter (+/- 4,720,503)
  test write_aiff_track                ... bench: 462,689,052 ns/iter (+/- 88,574,306)
  test write_wave_track                ... bench: 394,253,940 ns/iter (+/- 63,157,675)
*/

#[bench]
fn open_wave_track(b: &mut Bencher) {
  b.iter(|| {
    audio::open(&Path::new("tests/wav/Warrior Concerto - no meta.wav"))
      .ok().expect("Couldn't read file");
  });
}

#[bench]
fn open_aiff_track(b: &mut Bencher) {
  b.iter(|| {
    audio::open(&Path::new("tests/aiff/Warrior Concerto - no meta.aiff"))
      .ok().expect("Couldn't read file");
  });
}

#[bench]
fn save_wave_track(b: &mut Bencher) {
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
fn save_aiff_track(b: &mut Bencher) {
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
