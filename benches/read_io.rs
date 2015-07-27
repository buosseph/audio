// // This benchmark is for testing implementation approaches,
// // not the library itself. For that reason, it's not linked
// // to benches/lib.rs and is not tested using `cargo bench`
// //
// // Use `rustc -C opt-level=3 read_io.rs --test -o bench && ./bench --bench`

// #![feature(test)]
// extern crate test;
// use std::fs::File;
// use std::io::{BufReader, Read};
// use std::path::Path;
// use test::Bencher;

// #[bench]
// fn use_vec(b: &mut Bencher) {
//   let path = &Path::new("../tests/wav/Warrior Concerto - no meta.wav");
//   b.iter(|| {
//     let mut file = File::open(&path).ok().expect("Couldn't open file");
//     let mut buffer = vec![0u8; 38762540];
//     file.read_to_end(&mut buffer);
//   });
// }

// #[bench]
// fn use_buf_reader(b: &mut Bencher) {
//   let path = &Path::new("../tests/wav/Warrior Concerto - no meta.wav");
//   b.iter(|| {
//     let file = File::open(&path).ok().expect("Couldn't open file");
//     let reader = BufReader::with_capacity(38762540, file);
//   });
// }

// #[allow(unused_variables)]
// #[test]
// fn length() {
//   let path = &Path::new("../tests/wav/Warrior Concerto - no meta.wav");
//   // Buffer using vector
//   let mut file = File::open(&path).ok().expect("Couldn't open file");
//   let mut buffer = vec![0u8; 38762540];
//   file.read(&mut buffer);
//   // Use BufReader
//   let mut reader = BufReader::new(File::open(&path).ok().expect("Couldn't open file")); // Just buffer entire file for this test
//   let mut r_bytes = reader.bytes();
//   let mut b_bytes = buffer.bytes();
//   for i in 0..38762540 {
//     let buf_val = b_bytes.next().unwrap().unwrap();
//     let reader_val = r_bytes.next().unwrap().unwrap();
//     if !buf_val == reader_val {
//       panic!(format!("Error at byte {:?}:\tbuf_val: {:?} != reader_val: {:?}", i, buf_val, reader_val));
//     }
//   }
// }
