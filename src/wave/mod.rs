//pub mod chunk;
//pub mod decoder;
//pub mod encoder;

// Hex constants are stored, read, and written as little endian
const RIFF: u32 = 0x46464952;
const WAVE: u32 = 0x45564157;
const FMT:	u32 = 0x20746D66;
const DATA: u32 = 0x61746164;

use std::io::{Read, Seek};
use audio::{AudioDecoder};
use buffer::*;
use error::{AudioResult, AudioError};

pub struct Decoder<R> {
  r: R,
  bit_rate: u8,
  sample_rate: u32,
  channels: u32,
  data: Vec<Sample>
}

impl<R: Read + Seek> Decoder<R> {
  pub fn new(reader: R) -> Decoder<R> {
    Decoder {
      r: reader,
      bit_rate: 0u8,
      sample_rate: 0u32,
      channels: 0u32,
      data: Vec::new()
    }
  }
}

impl<R: Read + Seek> AudioDecoder for Decoder<R> {
  fn bit_rate(&self) -> AudioResult<u8> {
    Ok(self.bit_rate)
  }
  fn sample_rate(&self) -> AudioResult<u32> {
    Ok(self.sample_rate)
  }
  fn channels(&self) -> AudioResult<u32> {
    Ok(self.channels)
  }
  fn sample_order(&self) -> AudioResult<SampleOrder> {
    Ok(SampleOrder::INTERLEAVED)
  }
  /*
  fn read_format(&self) -> AudioResult<Vec<u8>> {
    //container::riff::read(self.r);
  }
  */
  //fn read_codec(codec: Codec, data: Vec<u8>) -> AudioResult<Vec<Sample>> {}

  fn decode(self) -> AudioResult<AudioBuffer> {
    //let bytes: AudioResult<Vec<u8>> = try!(self.read_format());
    //let data: Vec<Sample> = try!(read_codec(LPCM, bytes));
    Ok(
      AudioBuffer {
        bit_rate:     try!(self.bit_rate()),
        sample_rate:  try!(self.sample_rate()),
        channels:     try!(self.channels()),
        order:        try!(self.sample_order()),
        samples:      Vec::with_capacity(1)
      }
    )
  }
}

/*
#[cfg(test)]
mod tests {
	#[test]
	fn test_read_write_eq() {
		use super::*;

		let folder: String = String::from_str("tests/wav/");
		let files = vec![
			"i16-pcm-mono.wav",
			"i16-pcm-stereo.wav",
			"Warrior Concerto - no meta.wav",
		];

		for file in files.iter() {
			let mut path: String = folder.clone();
			path.push_str(*file);

			let audio = decoder::read_file(&Path::new(path.as_slice())).unwrap();
			let total_samples = audio.samples.len();
			let channels = audio.channels;
			let bit_rate = audio.bit_rate;
			let sample_rate = audio.sample_rate;
			let sample_order = audio.order;

			let written = encoder::write_file(&audio, &Path::new("tmp.wav")).unwrap();
			assert!(written);

			let verify = decoder::read_file(&Path::new("tmp.wav")).unwrap();

			// Assert written file is same length as read file!
			assert_eq!(total_samples, verify.samples.len());
			assert_eq!(channels, verify.channels);
			assert_eq!(bit_rate, verify.bit_rate);
			assert_eq!(sample_rate, verify.sample_rate);
			assert_eq!(sample_order, verify.order);
		}
	}

	#[test]
	fn test_read_lengths() {
		use super::*;

		let folder: String = String::from_str("tests/wav/");
		let stereo_files = vec![
			"stereo440-i32-44100.wav",
			"stereo440-i24-44100.wav",
			"stereo440-i16-44100.wav",
			"stereo440-u8-44100.wav",
		];
		let mono_files = vec![
			"mono440-i32-44100.wav",
			"mono440-i24-44100.wav",
			"mono440-i16-44100.wav",
			"mono440-u8-44100.wav",
		];

		for file in stereo_files.iter() {
			let mut path: String = folder.clone();
			path.push_str(*file);

			let audio = decoder::read_file(&Path::new(path.as_slice())).unwrap();
			let total_samples = audio.samples.len();
			let channels = audio.channels;
			let sample_rate = audio.sample_rate;

			assert_eq!(total_samples, 88200);
			assert_eq!(channels, 2);
			assert_eq!(sample_rate, 44100);
		}

		for file in mono_files.iter() {
			let mut path: String = folder.clone();
			path.push_str(*file); 

			let audio = decoder::read_file(&Path::new(path.as_slice())).unwrap();
			let total_samples = audio.samples.len();
			let channels = audio.channels;
			let sample_rate = audio.sample_rate;

			assert_eq!(total_samples, 44100);
			assert_eq!(channels, 1);
			assert_eq!(sample_rate, 44100);
		}
	}

	#[test]
	fn test_read_i32() {
		use super::*;

		let folder: String = String::from_str("tests/wav/");
		let files = vec![
			"stereo440-i32-44100.wav",
			"mono440-i32-44100.wav",
		];

		for file in files.iter() {
			let mut path: String = folder.clone();
			path.push_str(*file);

			let audio = decoder::read_file(&Path::new(path.as_slice())).unwrap();
			let channels = audio.channels;
			let bit_rate = audio.bit_rate;
			let sample_rate = audio.sample_rate;

			if file.eq(&"stereo440-i32-44100.wav".to_string()) {
				assert_eq!(channels, 2);
			}
			else {
				assert_eq!(channels, 1);
			}
			assert_eq!(bit_rate, 32);
			assert_eq!(sample_rate, 44100);
		}
	}
}
*/