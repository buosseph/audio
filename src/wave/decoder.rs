use std::io::{Read, Seek};
use buffer::AudioBuffer;
use traits::{AudioDecoder, Container};
use wave::container::WaveContainer;
use error::AudioResult;

/// Decodes audio in wave format from the
/// provided reader.
pub struct Decoder<'r, R: 'r> where R: Read + Seek {
  reader: &'r mut R,
}

impl<'r, R> Decoder<'r, R> where R: Read + Seek {
  /// Create a new wave format `Decoder` using
  /// the provided reader.
  #[inline]
  pub fn new(reader: &'r mut R) -> Decoder<R> {
    Decoder {
      reader: reader
    }
  }
}

impl<'r, R> AudioDecoder for Decoder<'r, R> where R: Read + Seek {
  /// Creates an `AudioBuffer` from the included reader via
  /// a `WaveContainer`.
  #[inline]
  fn decode(mut self) -> AudioResult<AudioBuffer> {
    let mut container = try!(WaveContainer::open(self.reader));
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
