//! Traits
//!
//! These are traits shared by multiple audio formats
use std::io::{Read, Seek, Write};
use buffer::AudioBuffer;
use codecs::Codec;
use error::*;

// Decodes audio formats to create `AudioBuffer`s.
pub trait AudioDecoder {
  fn decode(self) -> AudioResult<AudioBuffer>;
}

/// Encodes `AudioBuffer`s to an audio format.
pub trait AudioEncoder {
  fn encode(&mut self, audio: &AudioBuffer) -> AudioResult<()>;
  fn encode_as(&mut self,
               audio: &AudioBuffer,
               codec: Codec) -> AudioResult<()>;
}

// Container and Chunk traits must be refactored or removed, they will cause errors
// in future release

/// A `Container` is the higher level representation of the audio format.
pub trait Container {
  /// Decodes metadata provided by the container format. Audio bytes are not
  /// decoded.
  fn open<R: Read + Seek>(reader: &mut R) -> AudioResult<Self>;
  /// Writes the `AudioBuffer` to the provided writer following the container
  /// format and using the given `Codec` and `SampleFormat`.
  fn create<W: Write>(writer: &mut W,
                      audio: &AudioBuffer,
                      codec: Codec) -> AudioResult<()>;
}

/// A `Chunk` contains data relevant to the audio format, such as track metadata
/// or decoding details.
pub trait Chunk {
  /// Decodes chunk from byte slice.
  fn read(buffer: &[u8]) -> AudioResult<Self>;
}
