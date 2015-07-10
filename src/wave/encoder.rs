use std::io::Write;
use buffer::AudioBuffer;
use codecs::Codec;
use traits::{AudioEncoder, Container};
use wave::container::WaveContainer;
use error::AudioResult;

/// Encodes audio to wave format to the
/// provided writer.
pub struct Encoder<'w, W: 'w> {
  writer: &'w mut W,
}

impl<'w, W> Encoder<'w, W> where W: Write {
  /// Create a new wave format `Encoder` using
  /// the provided writer.
  #[inline]
  pub fn new(writer: &'w mut W) -> Encoder<'w, W> {
    Encoder {
      writer: writer
    }
  }
}

impl<'w, W> AudioEncoder for Encoder<'w, W> where W: Write {
  /// Creates and writes a `WaveContainer` to the included writer.
  #[inline]
  fn encode(&mut self, audio: &AudioBuffer) -> AudioResult<()> {
    let buffer: Vec<u8> = try!(WaveContainer::create(Codec::LPCM, audio));
    try!(self.writer.write_all(&buffer));
    Ok(())
  }
}
