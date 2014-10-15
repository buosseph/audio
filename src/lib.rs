// For reference, https://github.com/PistonDevelopers/image

#![feature(globs)]
pub use audio::RawAudio as RawAudio;
pub use audio:: {
	MONO,
	INTERLEAVED,
	REVERSED,
	PLANAR
};

pub mod audio;

// Codecs
pub mod wave;

// Processing
//pub mod dynamics;

#[test]
fn it_works() {
}
