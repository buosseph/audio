use audio::RawAudio;
use audio::SampleOrder;
use std::io::{File, IoResult};
use std::path::posix::{Path};
use super::chunk;
use super::{FORM, COMM, SSND};

pub fn read_file_data(file_path: &str) -> IoResult<()> {
	let path = Path::new(file_path);
	let mut file = try!(File::open(&path));

	let iff_header =  file.read_be_i32().unwrap();
	if iff_header != FORM {
		panic!("File is not valid AIFF.".to_string())
	}
	let header = chunk::IFFHeader::read_chunk(&mut file).unwrap();


	let comm_chunk_marker = file.read_be_i32().unwrap();
	if comm_chunk_marker != COMM {
		panic!("File is not valid AIFF. Does not contain required common chunk.".to_string())
	}
	let comm = chunk::CommonChunk::read_chunk(&mut file).unwrap();
	let sample_rate: f64 = convert_from_ieee_extended(comm.sampling_rate);


	let ssnd_chunk_marker = file.read_be_i32().unwrap();
	if ssnd_chunk_marker != SSND {
		panic!("File is not valid AIFF. Does not contain required sound data chunk.".to_string())
	}
	let ssnd = chunk::SoundDataChunk::read_chunk(&mut file).unwrap();


	println!(
	"master_iff_chunk:
		(FORM) {}
		File size: {}
		File type: (AIFF) {}
	COMM_chunk:
		Chunk size: {},
		Channels: {},
		Frames: {},
		Bit rate: {},
		Sample rate (extended): {},
	SSND_chunk:
		Chunk size: {},
		Offset: {},
		Block size: {},
	",
		iff_header,
		header.size,
		header.form_type,
		comm.size,
		comm.num_of_channels,
		comm.num_of_frames,
		comm.bit_rate,
		sample_rate,
		ssnd.size,
		ssnd.offset,
		ssnd.block_size
		);

	Ok(())
}

pub fn read_file(file_path: &str) -> IoResult<RawAudio> {
	let path = Path::new(file_path);
	let mut file = try!(File::open(&path));

	let iff_header =  file.read_be_i32().unwrap();
	if iff_header != FORM {
		panic!("File is not valid AIFF.".to_string())
	}
	let header = chunk::IFFHeader::read_chunk(&mut file).unwrap();


	let comm_chunk_marker = file.read_be_i32().unwrap();
	if comm_chunk_marker != COMM {
		panic!("File is not valid AIFF. Does not contain required common chunk.".to_string())
	}
	let comm = chunk::CommonChunk::read_chunk(&mut file).unwrap();
	let sample_rate: uint = convert_from_ieee_extended(comm.sampling_rate) as uint;

	let ssnd_chunk_marker = file.read_be_i32().unwrap();
	if ssnd_chunk_marker != SSND {
		panic!("File is not valid AIFF. Does not contain required sound data chunk.".to_string())
	}
	let ssnd = chunk::SoundDataChunk::read_chunk(&mut file).unwrap();


	println!(
	"master_iff_chunk:
		(FORM) {}
		File size: {}
		File type: (AIFF) {}
	COMM_chunk:
		Chunk size: {},
		Channels: {},
		Frames: {},
		Bit rate: {},
		Sample rate: {},
	SSND_chunk:
		Chunk size: {},
		Offset: {},
		Block size: {},
	",
		iff_header,
		header.size,
		header.form_type,
		comm.size,
		comm.num_of_channels,
		comm.num_of_frames,
		comm.bit_rate,
		sample_rate,
		ssnd.size,
		ssnd.offset,
		ssnd.block_size
		);

	let num_of_frames: uint = comm.num_of_frames as uint;

	match comm.bit_rate {
		16 	=> {
			match comm.num_of_channels {
				2 	=> {
					let mut samples: Vec<f64> = Vec::with_capacity(num_of_frames);
					for i in range(0, num_of_frames) {
						let left_sample = match file.read_be_i16() {
							Ok(sample) => {sample},
							Err(e)	=> {
								panic!(format!(
									"Error reading left sample {} from file: {}", i, e
								))
							}
						};

						let right_sample = match file.read_be_i16() {
							Ok(sample) => {sample},
							Err(e)	=> {
								panic!(format!(
									"Error reading right sample {} from file: {}", i, e
								))
							}
						};

						let float_left: f64 = left_sample as f64 / 32768f64;
						let float_right: f64 = right_sample as f64 / 32768f64;

						samples.push(float_left);
						samples.push(float_right);
					}

					Ok(
						RawAudio{
							bit_rate: 			comm.bit_rate as uint,
							sampling_rate: 		sample_rate,
							num_of_channels: 	comm.num_of_channels as uint,
							order: 				SampleOrder::INTERLEAVED,
							samples: 			samples,
						}
					)
				},

				1 	=> {
					let mut samples: Vec<f64> = Vec::with_capacity(num_of_frames);
					for i in range(0, num_of_frames) {
						match file.read_be_i16() {
							Ok(sample) => {
								let float_sample = sample as f64 / 32768f64;
								samples.push(float_sample);
							},
							Err(e)	=> {
								panic!(format!(
									"Error reading sample {} from file: {}", i, e
								))
							}
						}
					}

					Ok(
						RawAudio {
							bit_rate: 			comm.bit_rate as uint,
							sampling_rate: 		sample_rate,
							num_of_channels: 	comm.num_of_channels as uint,
							order: 				SampleOrder::MONO,
							samples: 			samples,
						}
					)
				},

				_	=> {
					panic!(format!(
						"This file is encoded using an unsupported number of channels. Cannot read {}-channel files.",
						comm.num_of_channels
					))
				}
			}
		},

		_	=> {
			panic!(format!(
				"This file is encoded using an unsupported bitrate. Cannot read {}-bit files.",
				comm.bit_rate
			))
		}
	}
}

fn ieee_u32_to_f64(num: u32) -> f64 {
	((num - 2147483647u32 - 1) as i32) as f64 + 2147483648f64
}

fn convert_from_ieee_extended(bytes: Vec<u8>) -> f64 {
	let mut num: f64;
	let mut exponent: int;
	let mut hi_mant: u32;
	let mut low_mant: u32;

	exponent = ( ((bytes[0] as u16 & 0x7f) << 8) | (bytes[1] & 0xff) as u16 ) as int;
	hi_mant = 	(bytes[2] & 0xff) as u32 	<< 24
			| 	(bytes[3] & 0xff) as u32 	<< 16
			| 	(bytes[4] & 0xff) as u32 	<< 8
			| 	(bytes[5] & 0xff) as u32;
	low_mant = 	(bytes[6] & 0xff) as u32 	<< 24
			| 	(bytes[7] & 0xff) as u32 	<< 16
			| 	(bytes[8] & 0xff) as u32 	<< 8
			| 	(bytes[9] & 0xff) as u32;

	if exponent == 0 && hi_mant == 0 && low_mant == 0 {
		return 0f64;
	}

	if exponent == 0x7fff {
		panic!("Sampling rate is not a number!");
	}
	else {
		exponent -= 16383;
		exponent -= 31;
		num	= ::std::num::FloatMath::ldexp(ieee_u32_to_f64(hi_mant), exponent);		
		exponent -= 32;
		num  += ::std::num::FloatMath::ldexp(ieee_u32_to_f64(low_mant), exponent);
	}

	if bytes[0] & 0x80 > 0 {
		return -num;
	}
	else {
		return num;
	}
}