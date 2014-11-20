use audio::{RawAudio, MONO, INTERLEAVED};

use std::io::{File, IoResult};
use std::path::posix::{Path};

use super::chunk;
use super::{RIFF, FMT, DATA};

// TODO: Replace fails with IoErrors

pub fn read_file_meta(file_path: &str) {
	let path = Path::new(file_path);
	let mut file = match File::open(&path) {
		Ok(f)	=> f,
		Err(e)		=> fail!("File error: {}", e),
	};


	let riff_header = file.read_le_u32().unwrap();
	if riff_header != RIFF {
		fail!("File is not valid WAVE.");
	}
	let header = chunk::RIFFHeader::read_chunk(&mut file).unwrap();


	let format_chunk_marker = file.read_le_u32().unwrap();
	if format_chunk_marker != FMT {
		fail!("File is not valid WAVE. Does not contain required format chunk.");
	}
	let fmt = chunk::FormatChunk::read_chunk(&mut file).unwrap();


	let data_chunk_marker = file.read_le_u32().unwrap();
	if data_chunk_marker != DATA {
		fail!("Files is not valid WAVE. Does not contain required data chunk.");
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

}

#[allow(unreachable_code)]
pub fn read_file(file_path: &str) -> IoResult<RawAudio> {
	// Assume 44 byte header for now (if fmt chunk is longer than )

	let path = Path::new(file_path);
	let mut file = match File::open(&path) {
		Ok(f)	=> f,
		Err(e)	=> fail!("\nError opening file at path: {}\n\n{}", file_path, e),
	};


	let riff_header = try!(file.read_le_u32());
	if riff_header != RIFF {
		fail!("File is not valid WAVE.");
	}
	let header = chunk::RIFFHeader::read_chunk(&mut file).unwrap();


	let format_chunk_marker = try!(file.read_le_u32());
	if format_chunk_marker != FMT {
		fail!("File is not valid WAVE. Does not contain required format chunk.");
	}
	let fmt = chunk::FormatChunk::read_chunk(&mut file).unwrap();


	let data_chunk_marker = try!(file.read_le_u32());
	if data_chunk_marker != DATA {
		fail!("Files is not valid WAVE. Does not contain required data chunk.");
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


	// Reading:
	// - Check if PCM
	// - Check bitrate
	// - Check channels and block size

	let number_of_samples: uint = data_size as uint / fmt.block_size as uint ;
		// = data_size / block_size = data_size * 8 / (num_of_channels * bit_rate) 
	println!("{}", number_of_samples);
	if fmt.compression_code as uint == 1 {
		match fmt.bit_rate as uint {
			// Uses signed ints (8-bit uses uints)
			16 => {
				match (fmt.num_of_channels as uint, fmt.block_size as uint) {

					// Stereo
					(2, 4) => {
						let mut samples: Vec<f64> = Vec::with_capacity(number_of_samples);
						for i in range(0, number_of_samples) {
							let left_sample = match file.read_le_i16() {
								Ok(sample) => {sample},
								Err(e)	=> {
									fail!("Error reading left sample {} from file: {}", i, e);
								}
							};

							let right_sample = match file.read_le_i16() {
								Ok(sample) => {sample},
								Err(e)	=> {
									fail!("Error reading right sample {} from file: {}", i, e);
								}
							};

							let float_left: f64 = left_sample as f64 / 32768f64;
							let float_right: f64 = right_sample as f64 / 32768f64;

							samples.push(float_left);
							samples.push(float_right);
						}

						Ok(
							RawAudio{
								bit_rate: fmt.bit_rate as uint,
								sampling_rate: fmt.sampling_rate as uint,
								num_of_channels: fmt.num_of_channels as uint,
								order: INTERLEAVED,
								samples: samples,
							}
						)
					},

					// Mono
					(1, 2) => {
						let mut samples: Vec<f64> = Vec::with_capacity(number_of_samples);
						for i in range(0, number_of_samples) {
							match file.read_le_i16() {
								Ok(sample) => {
									let float_sample = sample as f64 / 32768f64;
									samples.push(float_sample);
								},
								Err(e)	=> {
									fail!("Error reading sample {} from file: {}", i, e);
								}
							}
						}

						Ok(
							RawAudio {
								bit_rate: fmt.bit_rate as uint,
								sampling_rate: fmt.sampling_rate as uint,
								num_of_channels: fmt.num_of_channels as uint,
								order: MONO,
								samples: samples,
							}
						)
					},

					(_, _) => {
						fail!("This file is encoded using an unsupported number of channels.");
					}
				}

			},

			_ => {
				fail!("This file is encoded using an unsupported bitrate. Cannot read {}-bit files.", fmt.bit_rate);
			}
		}
	}
	else {
		fail!("This file is not encoded using PCM.");
	}

}