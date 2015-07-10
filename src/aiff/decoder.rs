use std::io::{Read, Seek};
use buffer::AudioBuffer;
use traits::{AudioDecoder, Container};
use aiff::container::AiffContainer;
use error::AudioResult;

/// Decodes audio in aiff format from the
/// provided reader.
pub struct Decoder<R> where R: Read + Seek {
  reader: R,
}

impl<R> Decoder<R> where R: Read + Seek {
  /// Create a new aiff format `Decoder` using
  /// the provided reader.
  #[inline]
  pub fn new(reader: R) -> Decoder<R> {
    Decoder {
      reader: reader
    }
  }
}

impl<R> AudioDecoder for Decoder<R> where R: Read + Seek {
  /// Creates an `AudioBuffer` from the included reader via
  /// a `AiffContainer`.
  #[inline]
  fn decode(mut self) -> AudioResult<AudioBuffer> {
    let mut container = try!(AiffContainer::open(&mut self.reader));
    Ok(
      AudioBuffer {
        bit_rate:     container.bit_rate,
        sample_rate:  container.sample_rate,
        channels:     container.channels,
        order:        container.order,
        samples:      try!(container.read_codec())
      }
    )
  }
}
