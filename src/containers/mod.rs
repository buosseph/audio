use std::io::{Read, Seek};
use error::*;

pub mod riff;

pub use containers::riff::RiffContainer as RiffContainer;

/// This trait is used to open the file container and read metadata.
/// Every `Container` returns the codec data bytes. For example,
/// `WAV` files use a RIFF container; so calling `riff::open` will
/// read the information stored in the container, but it will not 
/// decode the audio, as it may be in any codec such as LPCM, ALaw,
/// or ULaw to name a few.
pub trait Container<'r, R> where R: Read + Seek {
  fn open(r: &'r mut R) -> AudioResult<Self>;
  //fn read_chunk<C>(r: &mut R) -> AudioResult<C> where C: Chunk;
}

/// The trait used to read 
pub trait Chunk {
  fn read<R>(r: &mut R) -> AudioResult<Self> where R: Read + Seek;
}