//! Traits
//!
//! These are traits shared by multiple audio formats
use std::io::{Read, Seek};
use buffer::{AudioBuffer, Sample};
use codecs::{Codec};
use error::*;

/// Trait which all decoders must implement
pub trait AudioDecoder {
  fn decode(self) -> AudioResult<AudioBuffer>;
}

/// Trait which all encoders must implement
pub trait AudioEncoder {
  fn encode(&mut self, audio: &AudioBuffer) -> AudioResult<()>;
}

/// Every `Container` returns the codec data bytes. For example,
/// `WAV` files use a RIFF container; so calling `riff::open` will
/// read the information stored in the container, but it will not 
/// decode the audio, as it may be in any codec such as LPCM, ALaw,
/// or ULaw to name a few.
pub trait Container {
  /// Decodes metadata provided by the container format. Audio
  /// bytes are not decoded.
  fn open<R: Read + Seek>(r: &mut R) -> AudioResult<Self>;
  /// Decodes the audio bytes within the container to
  /// be used in an `AudioBuffer`.
  fn read_codec(&mut self) -> AudioResult<Vec<Sample>>;
  /// Writes the provided audio into a valid WaveContainer
  /// give the codec specified is supported by the format.
  fn create(codec: Codec, audio: &AudioBuffer) -> AudioResult<Vec<u8>>;
}

/// A `Chunk` contains data relevant to the audio file, such
/// as track metadata or decoding details.
pub trait Chunk {
  /// Decodes chunk from byte slice.
  fn read(buffer: &[u8]) -> AudioResult<Self>;
}
