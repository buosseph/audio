use audio::RawAudio;
use audio::SampleOrder;
// use audio::{AudioDecoder};
use std::io::{File, IoResult};
use std::path::posix::Path;
use super::chunk;
use super::chunk::CompressionCode;
use super::{RIFF, FMT, DATA};

// Needs to be in a trait since it's going to be used by AIFF
fn le_u8_array_to_i16(array: &[u8; 2]) -> i16{
	(array[1] as i16) << 8 | array[0] as i16
}
#[test]
fn test_le_u8_array_to_i16() {
	let array: [u8; 4] = [0x24, 0x17, 0x1e, 0xf3];
	let case1: &[u8; 2] = &[array[0], array[1]];
	let case2: &[u8; 2] = &[array[2], array[3]];
	assert_eq!(5924i16, le_u8_array_to_i16(case1));
	assert_eq!(-3298i16, le_u8_array_to_i16(case2));
}


pub fn read_file_meta(file_path: &str) -> IoResult<()>{
	let path = Path::new(file_path);
	let mut file = try!(File::open(&path));

	let riff_header = try!(file.read_le_u32());
	if riff_header != RIFF {
		panic!("File is not valid WAVE.".to_string())
	}
	let header = chunk::RIFFHeader::read_chunk(&mut file).unwrap();

	let format_chunk_marker = try!(file.read_le_u32());
	if format_chunk_marker != FMT {
		panic!("File is not valid WAVE. Does not contain required format chunk.".to_string())
	}
	let fmt = chunk::FormatChunk::read_chunk(&mut file).unwrap();

	let data_chunk_marker = try!(file.read_le_u32());
	if data_chunk_marker != DATA {
		panic!("Files is not valid WAVE. Does not contain required data chunk.".to_string())
	}
	let data_size = file.read_le_u32().unwrap();

	println!(
	"master_riff_chunk:
		(RIFF) {}
		File size: {}
		File type: (WAVE) {}
	fmt_chunk:
		Chunk size: {},
		Format: {} (1 = PCM, 3 = IEEE float, ...),
		Channels: {},
		Sample rate: {},
		Data rate: {},
		Block size: {},
		Bit rate: {}
	data_chunk:
		Data size: {} bytes
	",
		riff_header,
		header.size,
		header.format,
		fmt.size,
		fmt.compression_code,
		fmt.num_of_channels,
		fmt.sampling_rate,
		fmt.data_rate,
		fmt.block_size,
		fmt.bit_rate,
		data_size,
		);

	Ok(())
}

pub fn read_file(file_path: &str) -> IoResult<RawAudio> {
	// Assume 44 byte header for now (if fmt chunk is longer than )

	let path = Path::new(file_path);
	let mut file = try!(File::open(&path));

	let riff_header = try!(file.read_le_u32());
	if riff_header != RIFF {
		panic!("File is not valid WAVE.".to_string())
	}
	let header = chunk::RIFFHeader::read_chunk(&mut file).unwrap();

	let format_chunk_marker = try!(file.read_le_u32());
	if format_chunk_marker != FMT {
		panic!("File is not valid WAVE. Does not contain required format chunk.".to_string())
	}
	let fmt = chunk::FormatChunk::read_chunk(&mut file).unwrap();

	let data_chunk_marker = try!(file.read_le_u32());
	if data_chunk_marker != DATA {
		panic!("Files is not valid WAVE. Does not contain required data chunk.".to_string())
	}
	let data = chunk::DataChunk::read_chunk(&mut file).unwrap();

	// 	println!(
	// "master_riff_chunk:
	// 	(RIFF) {}
	// 	File size: {}
	// 	File type: (WAVE) {}
	// fmt_chunk:
	// 	Chunk size: {},
	// 	Format: {} (1 = PCM, 3 = IEEE float, ...),
	// 	Channels: {},
	// 	Sample rate: {},
	// 	Data rate: {},
	// 	Block size: {},
	// 	Bit rate: {}
	// data_chunk:
	// 	Data size: {} bytes
	// ",
	// 	riff_header,
	// 	header.size,
	// 	header.format,
	// 	fmt.size,
	// 	fmt.compression_code,
	// 	fmt.num_of_channels,
	// 	fmt.sampling_rate,
	// 	fmt.data_rate,
	// 	fmt.block_size,
	// 	fmt.bit_rate,
	// 	data_size,
	// 	);


	// Reading:
	// - Check if PCM
	// - Check bitrate
	// - Check channels and block size

	let num_of_frames: uint = data.size as uint / fmt.block_size as uint ;
	let mut samples: Vec<f64> = Vec::with_capacity(num_of_frames * fmt.num_of_channels as uint);

	match fmt.compression_code {
		CompressionCode::PCM => {
			match fmt.bit_rate {
				// Uses signed ints (8-bit uses uints)
				16 => {
					match (fmt.num_of_channels, fmt.block_size) {
						(2, 4) => {
							let mut frame: &[u8];
							for i in range(0, num_of_frames) {
								frame = data.data.slice(i * fmt.block_size as uint, i * fmt.block_size as uint + fmt.block_size as uint);

								let left_sample	: i16 	= le_u8_array_to_i16(&[frame[0], frame[1]]);
								let right_sample: i16 	= le_u8_array_to_i16(&[frame[2], frame[3]]);
							
								let float_left	: f64 	= left_sample as f64 / 32768f64;
								let float_right	: f64 	= right_sample as f64 / 32768f64;

								samples.push(float_left);
								samples.push(float_right);
							}
						},

						(1, 2) => {
							let mut frame: &[u8];
							for i in range(0, num_of_frames) {
								frame = data.data.slice(i * fmt.block_size as uint, i * fmt.block_size as uint + fmt.block_size as uint);
								let sample : i16 = le_u8_array_to_i16(&[frame[0], frame[1]]);
								let float_sample : f64 = sample as f64 / 32768f64;
								samples.push(float_sample);
							}
						},

						(_, _) => {
							panic!(format!(
								"This file is encoded using an unsupported number of channels. Cannot read {}-channel files.",
								fmt.num_of_channels
							))
						}
					}
				},

				_ => {
					panic!(format!(
						"This file is encoded using an unsupported bitrate. Cannot read {}-bit files.",
						fmt.bit_rate
					))
				}
			}
		},
		_ => {
			panic!("This file is not encoded using PCM.".to_string())
		}
	}

	Ok(
		RawAudio {
			bit_rate: fmt.bit_rate as uint,
			sample_rate: fmt.sampling_rate as uint,
			channels: fmt.num_of_channels as uint,
			order: SampleOrder::MONO,
			samples: samples,
		}
	)
}

#[cfg(test)]
mod tests {
	extern crate test;

	use super::*;

	#[bench]
	fn bench_read_file(b: &mut test::Bencher) {
		b.iter(|| {
			let _ = read_file("BrassAttack4.wav");
		});
	}
}