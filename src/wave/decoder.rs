use std::io::{Read, Seek};
use audio::{AudioDecoder};
use buffer::*;
use traits::Container;
use wave::container::WaveContainer;
use error::AudioResult;

pub struct Decoder<R> {
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
  fn decode(mut self) -> AudioResult<AudioBuffer> {
    let mut container = try!(WaveContainer::open(&mut self.reader));
    let bit_rate = container.bit_rate;
    let sample_rate = container.sample_rate;
    let channels = container.channels;
    let order = container.order;
    let data: Vec<Sample> = try!(container.read_codec());
    Ok(
      AudioBuffer {
        bit_rate:     bit_rate,
        sample_rate:  sample_rate,
        channels:     channels,
        order:        order,
        samples:      data
      }
    )
  }
}
