use std::io::{Read, Seek};
use buffer::AudioBuffer;
use error::AudioResult;
use traits::{AudioDecoder, Container};
use wave::container::WaveContainer;

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
    let container = try!(WaveContainer::open(self.reader));
    Ok(AudioBuffer::from_samples(
      container.sample_rate,
      container.channels,
      container.samples
    ))
  }
}
