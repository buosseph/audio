#![feature(test)]
extern crate audio;
extern crate byteorder;
extern crate test;

use std::path::Path;
use test::Bencher;


// Last saved results from previous design
// test read_aiff_track  ... bench: 167,915,163 ns/iter (+/- 4,564,825)
// test read_wave_track  ... bench: 181,276,731 ns/iter (+/- 4,720,503)
// test write_aiff_track ... bench: 462,689,052 ns/iter (+/- 88,574,306)
// test write_wave_track ... bench: 394,253,940 ns/iter (+/- 63,157,675)

// Recent results
// test open_aiff_track ... bench: 159,762,970 ns/iter (+/- 10,027,318)
// test open_wave_track ... bench: 154,542,292 ns/iter (+/- 12,215,616)
// test save_aiff_track ... bench: 641,674,169 ns/iter (+/- 423,290,736)
// test save_wave_track ... bench: 487,117,003 ns/iter (+/- 332,105,354)

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
