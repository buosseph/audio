// For reference, https://github.com/PistonDevelopers/image

#![feature(globs)]

// Audio structs
pub use audio::RawAudio as RawAudio;
pub use audio:: {
	MONO,
	INTERLEAVED,
	REVERSED,
	PLANAR
};

// Traits
pub use audio::Dynamics;
pub use audio::Utilities;


pub mod audio;

// Codecs
pub mod wave;

// Processing
pub mod utilities;
pub mod dynamics;

#[test]
fn it_works() {
}
