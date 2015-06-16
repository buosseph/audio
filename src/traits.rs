//! Traits
//!
//! These are traits shared by multiple audio formats
use std::io::{Read, Seek};
use buffer::{AudioBuffer, Sample};
use codecs::{Codec};
use error::*;

/// This trait is used to open the file container and read metadata.
/// Every `Container` returns the codec data bytes. For example,
/// `WAV` files use a RIFF container; so calling `riff::open` will
/// read the information stored in the container, but it will not 
/// decode the audio, as it may be in any codec such as LPCM, ALaw,
/// or ULaw to name a few.
pub trait Container {
  fn open<R: Read + Seek>(r: &mut R) -> AudioResult<Self>;
  fn read_codec(&mut self) -> AudioResult<Vec<Sample>>;
  fn create(codec: Codec, audio: &AudioBuffer) -> AudioResult<Vec<u8>>;
}

/// The trait used to read 
pub trait Chunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<Self>;
}
