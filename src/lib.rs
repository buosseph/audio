/** Terminology (To avoid future confusion)
 *  Sample  - A single recorded value independent of channel
 *  Frame or Block - A set of samples, one from each channel, to be played simultaneously
 *  Clip  - A set of frames representing an interval of time within or containing the entire read sound
 */

extern crate byteorder;

mod audio;
mod buffer;
pub mod error;

pub use buffer::{
  Sample,
  SampleOrder,
  AudioBuffer
};

mod codecs;
mod containers;

mod wave;
mod aiff;

pub use audio::{
  AudioFormat,
  open,
  load,
  save,
  write
};