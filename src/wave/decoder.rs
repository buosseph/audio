use std::io::{Read, Seek};
use audio::{AudioDecoder};
use buffer::*;
use containers::*;
use error::AudioResult;

pub struct Decoder<R> where R: Read + Seek {
  reader: R,
  data: Vec<Sample>
}

impl<R> Decoder<R> where R: Read + Seek {
  pub fn new(reader: R) -> Decoder<R> {
    Decoder {
      reader: reader,
      data: Vec::new()
    }
  }
}

impl<R> AudioDecoder for Decoder<R> where R: Read + Seek {
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
  /*
  fn open_container(&mut self) -> AudioResult<Vec<u8>> {
    let container = RiffContainer::open(self.r);
    Ok(Vec::new())
  }*/
  //fn read_codec(codec: Codec, data: Vec<u8>) -> AudioResult<Vec<Sample>> {}
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