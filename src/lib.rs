/** Terminology (To avoid future confusion)
 *  Sample  - A single recorded value independent of channel
 *  Frame - A set of samples, one from each channel, to be played simultaneously
 *  Clip  - A set of frames representing an interval of time within or containing the entire read sound
 */

mod audio;
mod buffer;
pub mod error;

//pub mod wave;

pub use buffer::{
  Sample,
  SampleOrder,
  AudioBuffer
};

pub use audio::{
  load,
  save
};