// For reference, https://github.com/PistonDevelopers/image

#![feature(globs)]

pub use audio::RawAudio;
pub use audio::SampleOrder::{
	MONO,
	INTERLEAVED,
	REVERSED,
	PLANAR,
};


pub use audio::Dynamics;
pub use audio::Utilities;
pub use audio::Filter;

pub mod audio;

// Formats
pub mod wave;
pub mod aiff;

// Processing
// pub mod utilities;
pub mod dynamics;
pub mod filter;