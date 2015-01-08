// For reference, https://github.com/PistonDevelopers/image

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
pub use audio::Filter;

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
// pub mod utilities;
pub mod dynamics;
pub mod filter;

pub mod db;