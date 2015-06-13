use std::io::{Read, Seek};
use audio::{AudioDecoder};
use buffer::*;
use codecs::*;
use containers::*;
use error::{AudioResult, AudioError};

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
    Ok(SampleOrder::INTERLEAVED)
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

pub fn read_file(path: &Path) -> AudioResult<RawAudio> {
	let mut file = try!(File::open(path));

	let riff_header = try!(file.read_le_u32());
	if riff_header != RIFF {
		return Err(AudioError::FormatError(
			"File is not valid WAVE".to_string()
		))
	}
	try!(chunk::RIFFHeader::read_chunk(&mut file));

	let format_chunk_marker = try!(file.read_le_u32());
	if format_chunk_marker != FMT {
		return Err(AudioError::FormatError(
			"File is not valid WAVE. Does not contain required format chunk.".to_string()
		))
	}
	let fmt = chunk::FormatChunk::read_chunk(&mut file).unwrap();

	let data_chunk_marker = try!(file.read_le_u32());
	if data_chunk_marker != DATA {
		return Err(AudioError::FormatError(
			"File is not valid WAVE. Does not contain required data chunk.".to_string()
		))
	}
	let data = chunk::DataChunk::read_chunk(&mut file).unwrap();


	let num_of_frames: uint = data.size as uint / fmt.block_size as uint ;
	let mut samples: Vec<f64> = Vec::with_capacity(num_of_frames * fmt.num_of_channels as uint);
	let mut sample_order: SampleOrder;

	let mut frame: &[u8];
	match fmt.compression_code {
		PCM => {
			match fmt.bit_rate {
				8 => {
					match (fmt.num_of_channels, fmt.block_size) {
						(2, 2) => {
							sample_order = INTERLEAVED;
							for i in range(0, num_of_frames) {
								frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];

								let left_sample	: u8 	= frame[0];
								let right_sample: u8 	= frame[1];
							
								let float_left	: f64 	= (left_sample as f64 - 128f64) / 128f64;
								let float_right	: f64 	= (right_sample as f64 - 128f64) / 128f64;

								samples.push(float_left);
								samples.push(float_right);
							}
						},

						(1, 1) => {
							sample_order = MONO;
							for i in range(0, num_of_frames) {
								frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];
								let sample : u8 		= frame[0];
								let float_sample : f64 	= (sample as f64 - 128f64) / 128f64;
								samples.push(float_sample);
							}
						},

						(_, _) => {
							return Err(AudioError::UnsupportedError(
								format!(
								"Cannot read {}-channel .wav files.",
								fmt.num_of_channels)
							))
						}
					}
				},

				16 => {
					match (fmt.num_of_channels, fmt.block_size) {
						(2, 4) => {
							sample_order = INTERLEAVED;
							for i in range(0, num_of_frames) {
								frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];

								let left_sample	: i16 	= (frame[1] as i16) << 8 | frame[0] as i16;
								let right_sample: i16 	= (frame[3] as i16) << 8 | frame[2] as i16;
							
								let float_left	: f64 	= left_sample as f64 / 32768f64;
								let float_right	: f64 	= right_sample as f64 / 32768f64;

								samples.push(float_left);
								samples.push(float_right);
							}
						},

						(1, 2) => {
							sample_order = MONO;
							for i in range(0, num_of_frames) {
								frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];
								let sample : i16 		= (frame[1] as i16) << 8 | frame[0] as i16;
								let float_sample : f64 	= sample as f64 / 32768f64;
								samples.push(float_sample);
							}
						},

						(_, _) => {
							return Err(AudioError::UnsupportedError(
								format!(
								"Cannot read {}-channel .wav files.",
								fmt.num_of_channels)
							))
						}
					}
				},

				24 => {
					match (fmt.num_of_channels, fmt.block_size) {
						(2, 6) => {
							sample_order = INTERLEAVED;
							for i in range(0, num_of_frames) {
								frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];

								let left_sample	: i32 	= (frame[2] as i32) << 16 | (frame[1] as i32) << 8 | frame[0] as i32;
								let right_sample: i32 	= (frame[5] as i32) << 16 | (frame[4] as i32) << 8 | frame[3] as i32;
							
								let float_left	: f64 	= left_sample as f64 / 8388608f64;
								let float_right	: f64 	= right_sample as f64 / 8388608f64;

								samples.push(float_left);
								samples.push(float_right);
							}
						},

						(1, 3) => {
							sample_order = MONO;
							for i in range(0, num_of_frames) {
								frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];
								let sample : i32 		= (frame[2] as i32) << 16 | (frame[1] as i32) << 8 | frame[0] as i32;
								let float_sample : f64 	= sample as f64 / 8388608f64;
								samples.push(float_sample);
							}
						},

						(_, _) => {
							return Err(AudioError::UnsupportedError(
								format!(
								"Cannot read {}-channel .wav files.",
								fmt.num_of_channels)
							))
						}
					}
				},

				32 => {
					match (fmt.num_of_channels, fmt.block_size) {
						(2, 8) => {
							sample_order = INTERLEAVED;
							for i in range(0, num_of_frames) {
								frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];

								let left_sample	: i32 	= (frame[3] as i32) << 24 | (frame[2] as i32) << 16 | (frame[1] as i32) << 8 | frame[0] as i32;
								let right_sample: i32 	= (frame[7] as i32) << 24 | (frame[6] as i32) << 16 | (frame[5] as i32) << 8 | frame[4] as i32;
							
								let float_left	: f64 	= left_sample as f64 / 2147483648f64;
								let float_right	: f64 	= right_sample as f64 / 2147483648f64;

								samples.push(float_left);
								samples.push(float_right);
							}
						},

						(1, 4) => {
							sample_order = MONO;
							for i in range(0, num_of_frames) {
								frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];
								let sample : i32 		= (frame[3] as i32) << 24 | (frame[2] as i32) << 16 | (frame[1] as i32) << 8 | frame[0] as i32;
								let float_sample : f64 	= sample as f64 / 2147483648f64;
								samples.push(float_sample);
							}
						},

						(_, _) => {
							return Err(AudioError::UnsupportedError(
								format!(
								"Cannot read {}-channel .wav files.",
								fmt.num_of_channels)
							))
						}
					}
				},

				_ => {
					return Err(AudioError::UnsupportedError(
						format!(
						"Cannot read {}-bit .wav files.",
						fmt.bit_rate)
					))
				}
			}
		},
		_ => {
			return Err(AudioError::UnsupportedError(
				"Can only read PCM encoded .wav files.".to_string()
			))
		}
	}

	Ok(
		RawAudio {
			bit_rate: 		fmt.bit_rate as uint,
			sample_rate: 	fmt.sampling_rate as uint,
			channels: 		fmt.num_of_channels as uint,
			order: 			sample_order,
			samples: 		samples,
		}
	)
}
*/

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