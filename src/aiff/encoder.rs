use std::io::Write;
use buffer::AudioBuffer;
use codecs::{Codec, SampleFormat};
use traits::{AudioEncoder, Container};
use aiff::container::AiffContainer;
use error::AudioResult;

/// Encodes audio to aiff format to the provided writer.
pub struct Encoder<'w, W: 'w> {
  writer: &'w mut W,
}

impl<'w, W> Encoder<'w, W> where W: Write {
  /// Create a new aiff format `Encoder` using the provided writer.
  #[inline]
  pub fn new(writer: &'w mut W) -> Encoder<'w, W> {
    Encoder {
      writer: writer
    }
  }
}
 
impl<'w, W> AudioEncoder for Encoder<'w, W> where W: Write {
  /// Creates and writes a `AiffContainer` to the included writer. The audio
  /// is encoded to standard 16-bit, uncompressed LPCM audio.
  #[inline]
  fn encode(&mut self, audio: &AudioBuffer) -> AudioResult<()> {
    AiffContainer::create(&mut self.writer, audio,
                          SampleFormat::Signed16, Codec::LPCM)
  }
  /// Creates and writes a `AiffContainer` using the provided `SampleFormat`
  /// to the included writer. This is how audio can be encoded to different
  /// bit rates supported by the format.
  #[inline]
  fn encode_as(&mut self, audio: &AudioBuffer,
               sample_format: SampleFormat) -> AudioResult<()> {
    AiffContainer::create(&mut self.writer, audio,
                          sample_format, Codec::LPCM)
  }
}
