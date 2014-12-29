// For reference, https://github.com/PistonDevelopers/image

#![feature(globs)]

// Audio structs
pub use audio::RawAudio as RawAudio;
pub use audio::SampleOrder as SampleOrder;

// Traits
pub use audio::Dynamics as Dynamics;
pub use audio::Utilities as Utilities;
pub use audio::Filter as Filter;

pub mod audio;

// Formats
pub mod wave;
pub mod aiff;

// Processing (Need to be refactored)
// pub mod utilities;
pub mod dynamics;
pub mod filter;

#[test]
fn it_works() {
}
