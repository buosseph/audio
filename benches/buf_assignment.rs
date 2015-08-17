// This benchmark is for testing implementation approaches,
// not the library itself.

#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn range(b: &mut Bencher) {
  let mut v = vec![0u8; 44100 * 300];
  b.iter(|| {
    for i in 0..v.len() {
        v[i] = (28 * i) as u8;
    }
  });
}

#[bench]
fn enumerate(b: &mut Bencher) {
  let mut v = vec![0u8; 44100 * 300];
  b.iter(|| {
    for (i, sample) in v.iter_mut().enumerate() {
        *sample = (28 * i) as u8;
    }
  });
}
