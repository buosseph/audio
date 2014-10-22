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

// Formats
pub mod wave;
pub mod aiff;

// Processing
pub mod utilities;
pub mod dynamics;

#[test]
fn it_works() {
}
