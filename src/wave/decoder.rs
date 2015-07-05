use std::io::{Read, Seek};
use audio::AudioDecoder;
use buffer::*;
use traits::Container;
use wave::container::WaveContainer;
use error::AudioResult;

pub struct Decoder<R> where R: Read + Seek {
  reader: R,
}

impl<R> Decoder<R> where R: Read + Seek {
  #[inline]
  pub fn new(reader: R) -> Decoder<R> {
    Decoder {
      reader: reader
    }
  }
}

impl<R> AudioDecoder for Decoder<R> where R: Read + Seek {
  #[inline]
  fn decode(mut self) -> AudioResult<AudioBuffer> {
    let mut container = try!(WaveContainer::open(&mut self.reader));
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
