// To compile with library use `rustc -L target main.rs` as the library is in target/
#![feature(globs)]
extern crate audio;

// Need to import traits
// use audio::Dynamics;
// use audio::Utilities;

#[allow(dead_code)]
const VERSION: &'static str = "rustc 0.13.0-nightly (f168c12c5 2014-10-25 20:57:10 +0000)";

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
	println!("{}\n", VERSION);

	// let mut audio = audio::RawAudio {
	// 	bit_rate: 24,
	// 	sampling_rate: 44100,
	// 	num_of_channels: 2,
	// 	order: audio::INTERLEAVED,
	// 	samples: vec![0.1f64, 0.2f64, 0.2f64, 0.1f64],
	// };

	// audio.print_meta_data();
	// audio.print_samples();


	// audio.amplify(2f64);
	// audio.invert();

	// audio.print_samples();

	// let wav_audio1 = audio::wave::read_file("sine-440.wav");
	// let wav_audio2 = audio::wave::read_file("test.wav");

	let wav_audio = audio::wave::decoder::read_file("BrassAttack4.wav").unwrap();
	let written = audio::wave::encoder::write_file(wav_audio, "written.wav").unwrap();
	if written {println!("File written");}

	//wav_audio.print_samples();
	//let aiff_audio = audio::aiff::read_file_data("sine-440.aiff");


	/*let mut raw_audio = wave::read_file("../BrassAttack4.wav");

	// Simplest LP Filter
	//let mut processed_audio = raw_audio.samples.clone();

	//raw_audio.amplify(7.94328f32);
	raw_audio.delay();

	wave::write_file(raw_audio, "../processed.wav");*/

}