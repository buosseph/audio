#![feature(test)]
extern crate audio;
extern crate test;

use std::path::Path;
use test::Bencher;

/*
  A set of bench results for quick reference
  test read_aiff_track  ... bench: 213,222,314 ns/iter (+/- 5,191,823)
  test read_wave_track  ... bench: 190,206,245 ns/iter (+/- 6,010,546)
  test write_aiff_track ... bench: 558,127,499 ns/iter (+/- 59,900,198)
  test write_wave_track ... bench: 575,245,107 ns/iter (+/- 60,170,170)
*/
 

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
