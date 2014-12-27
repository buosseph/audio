use audio::RawAudio;
use audio::SampleOrder;

use std::io::{File, IoResult};
use std::path::posix::Path;

use super::chunk;
use super::{RIFF, FMT, DATA};

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
									panic!(format!(
										"Error reading left sample {} from file: {}", i, e
									))
								}
							};

							let right_sample = match file.read_le_i16() {
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
								bit_rate: fmt.bit_rate as uint,
								sampling_rate: fmt.sampling_rate as uint,
								num_of_channels: fmt.num_of_channels as uint,
								order: SampleOrder::INTERLEAVED,
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
									panic!(format!(
										"Error reading sample {} from file: {}", i, e
									))
								}
							}
						}

						Ok(
							RawAudio {
								bit_rate: fmt.bit_rate as uint,
								sampling_rate: fmt.sampling_rate as uint,
								num_of_channels: fmt.num_of_channels as uint,
								order: SampleOrder::MONO,
								samples: samples,
							}
						)
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
	}
	else {
		panic!("This file is not encoded using PCM.".to_string())
	}
}

#[cfg(test)]
mod tests {
	fn it_works() {
		
	}
}