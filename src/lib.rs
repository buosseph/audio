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
pub use buffer::AudioBuffer;

mod codecs;
pub use codecs::Codec as Codec;

mod decoder;
mod encoder;

mod error;
pub use error::{
  AudioResult,
  AudioError
};

pub mod format;

mod sample;
pub use sample::{
  FromSample,
  Sample,
  SampleOrder,
  ToSample
};

mod traits;

mod wave;
