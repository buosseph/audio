//use audio::AudioDecoder;

#[allow(unused_imports)]
use audio::{
	RawAudio,
	MONO,
	INTERLEAVED,
	REVERSED,
	PLANAR
};

use std::io::{File, IoResult};
use std::path::posix::{Path};

pub mod chunk;

// Sample = singular f64 value (independent of channel)
// Clip = Group of samples along time domain (Should always include all channels)
// Separate channels into separate tracks for processing

// Hex constants are stored, read, and written as little endian
const RIFF: u32 = 0x46464952;
const WAVE: u32 = 0x45564157;
const FMT:	u32 = 0x20746D66;
const DATA: u32 = 0x61746164;

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
								order: INTERLEAVED,
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

fn valid_file_path(filename: &str) -> bool{
	if filename.is_empty() {
		println!("Cannot write file with empty filename.");
		return false;
	}
	else if filename.char_at(0) == '/' {
		println!("You do not need / if you're trying to save to a directory.");
		return false;
	}
	true
}

#[allow(unreachable_code)]
pub fn write_file(raw_audio: RawAudio, file_path: &str) -> IoResult<bool> {

	if !valid_file_path(file_path) {
		return Ok(false);
	}

	let path = Path::new(file_path);
	let mut file = File::create(&path);

	let num_of_channels: u16 	= raw_audio.num_of_channels as u16;
	let sampling_rate: u32 		= raw_audio.sampling_rate as u32;
	let data_rate: u32 			= (raw_audio.sampling_rate * raw_audio.num_of_channels * raw_audio.bit_rate / 8) as u32;
	let bit_rate: u16 			= raw_audio.bit_rate as u16;
	let block_size: uint 		= raw_audio.num_of_channels * raw_audio.bit_rate / 8;


	// Assume 44 byte header for now
	// May be wrong...
	let file_size: u32 	=  (4 + 8 + 16 + 8 + raw_audio.samples.len() * block_size / raw_audio.num_of_channels) as u32;
		// = 4 + (8 + fmt_chunk size) + (8 + (data_chunk size * block_size)) (NOTE: 8 bytes are purposely missing for riff_header and file_size)
	
	let data_size: u32 	= (raw_audio.samples.len() * block_size) as u32;
		// = num_of_samples * block_size = num_of_samples * num_of_channels * bit_rate / 8


	// Assume 44 byte header for now
	try!(file.write_le_u32(RIFF));
	try!(file.write_le_u32(file_size));
	try!(file.write_le_u32(WAVE));

	try!(file.write_le_u32(FMT));
	try!(file.write_le_u32(16 as u32));	// 16-byte chunk
	try!(file.write_le_u16(1 as u16)); // PCM
	try!(file.write_le_u16(num_of_channels));
	try!(file.write_le_u32(sampling_rate));
	try!(file.write_le_u32(data_rate));
	try!(file.write_le_u16(block_size as u16));
	try!(file.write_le_u16(bit_rate));

	try!(file.write_le_u32(DATA));
	try!(file.write_le_u32(data_size));

	for sample in raw_audio.samples.iter() {
		let mut pcm_sample = sample * 32768f64;

		if pcm_sample > 32768f64 {
			pcm_sample = 32768f64;
		}
		if pcm_sample < -32768f64 {
			pcm_sample = -32768f64;
		}

		try!(file.write_le_i16(pcm_sample as i16));
	}

	Ok(true)

}


/*
#[cfg(test)]
mod tests {
	use super::*;

	// Tests need fixing
	#[test]
	fn test_write_file() {
		let test_files = vec!(
		"test-pcm-mono.wav",
		"test-pcm-stereo.wav",
		);
		
		for filename in test_files.iter() {
			let read_prefix = "../wav/".to_string();
			let path_to_read = read_prefix.append(*filename);

			let raw_audio = read_file(path_to_read.as_slice());
			println!("{}", raw_audio);

			let write_prefix = "../test/wav/".to_string();
			let path_to_write = write_prefix.append(*filename);

			assert!(write_file(raw_audio, path_to_write.as_slice()));

		}
	}

}*/

