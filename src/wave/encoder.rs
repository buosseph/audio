use audio::RawAudio;

use std::io::{File, IoResult};
use std::path::posix::{Path};

use super::{RIFF, WAVE, FMT, DATA};

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
		let mut pcm_sample = *sample * 32768f64;

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