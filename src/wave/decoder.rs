use std::io::{Read, Seek};
use audio::{AudioDecoder};
use buffer::*;
use containers::*;
use error::AudioResult;

pub struct Decoder<R> where R: Read + Seek {
  reader: R,
  // container: Container,
  bit_rate: u32,
  sample_rate: u32,
  channels: u32,
  block_size: u32,
  data: Vec<Sample>
}

impl<R> Decoder<R> where R: Read + Seek {
  pub fn new(reader: R) -> Decoder<R> {
    Decoder {
      reader: reader,
      // container: Container
      bit_rate: 0u32,
      sample_rate: 0u32,
      channels: 0u32,
      block_size: 0u32,
      data: Vec::new()
    }//.open_container
  }
}

impl<R> AudioDecoder for Decoder<R> where R: Read + Seek {
  fn bit_rate(&self) -> AudioResult<u32> {
    Ok(self.bit_rate)
  }
  fn sample_rate(&self) -> AudioResult<u32> {
    Ok(self.sample_rate)
  }
  fn channels(&self) -> AudioResult<u32> {
    Ok(self.channels)
  }
  fn sample_order(&self) -> AudioResult<SampleOrder> {
    if self.channels == 1 {
      Ok(SampleOrder::MONO)
    }
    else {
      Ok(SampleOrder::INTERLEAVED)
    }
  }
  /*
  fn open_container(&mut self) -> AudioResult<Vec<u8>> {
    let container = RiffContainer::open(self.r);
    Ok(Vec::new())
  }*/
  //fn read_codec(codec: Codec, data: Vec<u8>) -> AudioResult<Vec<Sample>> {}

  fn decode(mut self) -> AudioResult<AudioBuffer> {
    let mut container = try!(RiffContainer::open(&mut self.reader));
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

/*
#[cfg(test)]
mod tests {
	extern crate test;

	use super::*;

	#[bench]
	fn bench_read_file(b: &mut test::Bencher) {
		b.iter(|| {
			let _ = read_file(&Path::new("tests/wav/Warrior Concerto - no meta.wav"));
		});
	}
}
*/