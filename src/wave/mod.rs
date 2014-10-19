//use audio::AudioDecoder;
//use audio::{RawAudio, SampleOrder};

use std::str::from_utf8;
use std::io::{File};
use std::path::posix::{Path};

pub mod chunk;

// Sample = singular f64 value (independent of channel)
// Clip = Group of samples along time domain (Should always include all channels)
// Separate channels into separate tracks for processing


const RIFF: u32 = 0x52494646;


// All functions need to be rewritten!

pub fn read_file_data(wav_file_path: &str) {
	let path = Path::new(wav_file_path);
	let mut wav_file = match File::open(&path) {
		Ok(file)	=> file,
		Err(e)		=> fail!("File error: {}", e),
	};

	// Assume 44 byte header for now
	let double_word = wav_file.read_exact(4).unwrap();
	let riff_header = from_utf8(double_word.as_slice()).unwrap();

	let file_size = wav_file.read_le_u32().unwrap();
	let double_word = wav_file.read_exact(4).unwrap();
	let file_type_header = from_utf8(double_word.as_slice()).unwrap();


	let double_word = wav_file.read_exact(4).unwrap();
	let format_chunk_marker = from_utf8(double_word.as_slice()).unwrap();

	let fmt = chunk::FormatChunk::read_chunk(&mut wav_file).unwrap();

	// Quicker to read next few bytes rather than entire DataChunk
	let double_word = wav_file.read_exact(4).unwrap();
	let data_chunk_header = from_utf8(double_word.as_slice()).unwrap();
	let data_size = wav_file.read_le_u32().unwrap(); // In bytes

	println!(
		"master_riff_chunk:
			{}
			File size: {}
			File type: {}
		{}_chunk:
			Chunk length: {},
			Format: {} (1 = PCM, 3 = IEEE float, ...),
			Channels: {},
			Sample rate: {},
			Data rate: {},
			Block size: {},
			Bit rate: {}
		{}_chunk:
			Data size: {} bytes
		",
		riff_header,
		file_size,
		file_type_header,
		format_chunk_marker,
		fmt.size,
		fmt.compression_code,
		fmt.num_of_channels,
		fmt.sampling_rate,
		fmt.data_rate,
		fmt.block_size,
		fmt.bit_rate,
		data_chunk_header,
		data_size,
		);

}

// Incomplete (won't compile)
#[allow(unreachable_code)]
pub fn read_file(wav_file_path: &str) -> RawAudio {
	
	let path = Path::new(wav_file_path);
	let mut wav_file = match File::open(&path) {
		Ok(file)	=> file,
		Err(e)		=> fail!("File error: {}", e),
	};

	// Assume 44 byte header for now
	let double_word = wav_file.read_exact(4).unwrap();
	let riff_header = from_utf8(double_word.as_slice()).unwrap();
	let file_size = wav_file.read_le_u32().unwrap();
	let double_word = wav_file.read_exact(4).unwrap();
	let file_type_header = from_utf8(double_word.as_slice()).unwrap();


	let double_word = wav_file.read_exact(4).unwrap();
	let format_chunk_marker = from_utf8(double_word.as_slice()).unwrap();
	let fmt = chunk::FormatChunk::read_chunk(&mut wav_file).unwrap();


	let double_word = wav_file.read_exact(4).unwrap();
	let data_chunk_marker = from_utf8(double_word.as_slice()).unwrap();
	let data = chunk::DataChunk::read_chunk(&mut wav_file).unwrap();
			


	// Reading:
	// - Check if PCM
	// - Check bitrate
	// - Check channels and block size
	

	let number_of_samples: uint = data.size as uint / fmt.num_of_channels as uint;
	if fmt.compression_code == 1 {
		match fmt.bit_rate as uint {

			// Uses signed ints (8-bit uses uints)
			16 => {
				match (fmt.num_of_channels as uint, fmt.block_size as uint) {
					// Stereo
					// (2, 4) => {
					// 	let mut data: Vec<f64> = Vec::with_capacity(number_of_samples as uint);
					// 	for _ in range(0, number_of_samples as uint) {
					// 		match wav_file.read_le_i16() {
					// 			Ok(sample) => {
					// 				let float_sample = sample as f32 / 32768f32;
					// 				data.push(float_sample);
					// 			},
					// 			Err(e)	=> {
					// 				fail!("Error: {}", e);
					// 			}
					// 		}
					// 	}

					// 	// Assume interleved for now
					// 	RawAudio{ bit_rate: bit_rate as uint, sampling_rate: samples_per_sec as uint, num_of_channels: num_of_channels as uint, order: INTERLEAVED, samples: data}

					// },

					// Mono
					(1, 2) => {
						
						let mut samples: Vec<f64> = Vec::with_capacity(number_of_samples as uint);
						for _ in range(0, number_of_samples as uint) {
							match data.data.read_le_i16() {
								Ok(sample) => {
									let float_sample = sample as f64 / 32768f64;
									data.push(float_sample);
								},
								Err(e)	=> {
									fail!("Error: {}", e);
								}
							}
						}

						RawAudio {
							bit_rate: fmt.bit_rate as uint,
							sampling_rate: fmt.sampling_rate as uint,
							num_of_channels: fmt.num_of_channels as uint,
							order: INTERLEAVED,
							samples: samples,
						}
					},

					(_, _) => {
						fail!("This file is encoded using an unsupported number of channels.");
					}
				}

			},

			_ => {
				fail!("This file is encoded using an unsupported bitrate.");
			}
		}
	}
	else {
		fail!("This file is not encoded using PCM.");
	}

}


/*
// Only allow writing as PCM at the moment
pub fn write_file(raw_audio: RawAudio, wav_file_path: &str) -> bool {

	let path = Path::new(wav_file_path);
	let mut wav_file = File::create(&path);

	let num_of_channels: u16 = raw_audio.num_of_channels as u16;
	let samples_per_sec: u32 = raw_audio.sampling_rate as u32;
	let data_rate: u32 = (raw_audio.sampling_rate * raw_audio.num_of_channels * raw_audio.bit_rate / 8) as u32;
	let bit_rate: u16 = raw_audio.bit_rate as u16;
	let block_size: uint = raw_audio.num_of_channels * raw_audio.bit_rate / 8;



	// Assume 44 byte header for now
	let riff_header = "RIFF";
	let file_size: u32 =  (4 + 8 + 16 + 8 + raw_audio.samples.len() * block_size / raw_audio.num_of_channels) as u32;	// = 4 + (8 + fmt_chunk size) + (8 + (data_chunk size * block_size)) (NOTE: 8 bytes are purposely missing for riff_header and file_size)
	let file_type_header = "WAVE";

	// Audio format as determined as function argument? 
	let format_chunk_marker = "fmt ";
	let format_chunk_length: u32 = 16;		// 16 if PCM
	let format_tag: u16 = 1;				// 1 if PCM

	// NOTE: variables above are determined by audio format (e.g. PCM, float, A-Law, etc.)



	let data_chunk_header = "data";
	let data_size: u32 = (raw_audio.samples.len() * block_size / raw_audio.num_of_channels) as u32;		// = data_chunk size * block_size




	wav_file.write_str(riff_header).unwrap();
	wav_file.write_le_u32(file_size).unwrap();
	wav_file.write_str(file_type_header).unwrap();

	wav_file.write_str(format_chunk_marker).unwrap();
	wav_file.write_le_u32(format_chunk_length).unwrap();
	wav_file.write_le_u16(format_tag).unwrap();
	wav_file.write_le_u16(num_of_channels).unwrap();
	wav_file.write_le_u32(samples_per_sec).unwrap();
	wav_file.write_le_u32(data_rate).unwrap();
	wav_file.write_le_u16(block_size as u16).unwrap();
	wav_file.write_le_u16(bit_rate).unwrap();

	wav_file.write_str(data_chunk_header).unwrap();
	wav_file.write_le_u32(data_size).unwrap();



	for sample in raw_audio.samples.iter() {

		let mut pcm_sample = sample * 32768f32;

		if pcm_sample > 32767f32 {
			pcm_sample = 32767f32;
		}
		if pcm_sample < -32768f32 {
			pcm_sample = -32768f32;
		}

		wav_file.write_le_i16(pcm_sample as i16).unwrap();

	}

	true

}*/


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

