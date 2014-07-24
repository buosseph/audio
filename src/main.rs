#![feature(struct_variant)]
#![feature(globs)]


pub mod wave;

#[allow(dead_code)]
static version: &'static str = "rustc 0.11.0-pre-nightly (380657557cb3793d39dfc0d2321fc946cb3496f5 2014-07-02 00:21:36 +0000)";


#[allow(dead_code)]
fn main() {
	println!("{}\n", version);

	let mono_file = wave::read_file("../wav/test-pcm-mono.wav");
	println!("{}", mono_file);

	let stereo_file = wave::read_file("../wav/test-pcm-stereo.wav");
	println!("{}", stereo_file);

}