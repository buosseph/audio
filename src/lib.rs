// For reference, https://github.com/PistonDevelopers/image

#![feature(globs)]

// Audio structs
pub use audio::RawAudio as RawAudio;

// Traits
pub use audio::Dynamics as Dynamics;
// pub use audio::Utilities as Utilities;
pub use audio::Filter as Filter;

pub mod audio;

// Formats
pub mod wave;
pub mod aiff;

// Processing (Need to be refactored)
// pub mod utilities;
pub mod dynamics;

#[test]
fn it_works() {
}
