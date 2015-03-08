// For reference, https://github.com/PistonDevelopers/image

#![feature(core)]
#![feature(collections)]
#![feature(int_uint)]
#![feature(io)]
#![feature(path)]
#![feature(std_misc)]
#![feature(test)]

// Structs
pub use audio::RawAudio;
pub use audio::SampleOrder::{
	MONO,
	INTERLEAVED,
	REVERSED,
	PLANAR,
};

// Traits
pub use audio::Dynamics;
pub use audio::Utilities;
pub use audio::OldFilter;

// Functions
pub use audio::{
	load,
	save
};

pub mod audio;


// Formats
pub mod wave;
pub mod aiff;

// Processing
pub mod filter;

// pub mod utilities;
pub mod dynamics;
pub mod old_filter;

pub mod db;