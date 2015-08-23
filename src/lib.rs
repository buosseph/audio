/** Terminology:
 *  Sample  - A single recorded value independent of channel.
 *  Frame - A set of samples, one from each channel, to be played simultaneously. This is also called a block in some formats.
 *  Clip  - A set of frames representing an interval of time within or containing the entire read sound.
 */

extern crate byteorder;

mod audio;
pub use audio::{
  AudioFormat,
  open,
  load,
  save,
  save_as,
  write,
  write_as
};

mod buffer;
pub use buffer::{
  AudioBuffer,
  FromSample,
  Sample,
  SampleOrder,
  ToSample
};

mod codecs;
pub use codecs::Codec as Codec;

pub mod error;

mod traits;

mod wave;
mod aiff;


