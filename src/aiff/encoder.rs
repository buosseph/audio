use audio::RawAudio;

use std::io::{File, IoResult};
use std::path::posix::{Path};

use super::{FORM, AIFF, COMM, SSND};

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
	let bit_rate: u16 			= raw_audio.bit_rate as u16;


	let offset: u32 			= 0;
	let block_size: uint 		= raw_audio.num_of_channels * raw_audio.bit_rate / 8;
	let data_size: u32 	= (raw_audio.samples.len() * block_size) as u32;
	let ssnd_chunk_size: u32 = 8 + data_size;


	let total_bytes = 12 + 26 + 8 + ssnd_chunk_size;	// = [FORM: 12] + [COMM: 26] + [SSND: 8 + chunk_size]
	let file_size: u32 = total_bytes - 8;
	let num_of_frames: u32 = (raw_audio.samples.len() / raw_audio.num_of_channels) as u32;
	let sample_rate_buffer: Vec<u8> = vec![64, 13, -84, 68, 0, 0, 0, 0, 0, 0];	// 22050, to match test files; still need to fix!

	// FORM
	try!(file.write_be_i32(FORM));
	try!(file.write_be_u32(file_size));		// = total bytes - 8
	try!(file.write_be_i32(AIFF));

	// COMM
	try!(file.write_be_i32(COMM));
	try!(file.write_be_u32(18 as u32));		// COMM chunk always 18;
	try!(file.write_be_u16(num_of_channels));
	try!(file.write_be_u32(num_of_frames));
	try!(file.write_be_u16(bit_rate));
	try!(file.write(sample_rate_buffer.as_slice()));

	// SSND
	try!(file.write_be_i32(SSND));
	try!(file.write_be_u32(ssnd_chunk_size));
	try!(file.write_be_u32(offset));
	try!(file.write_be_u32(0));		// always 0 according to http://www.onicos.com/staff/iz/formats/aiff.html

	for sample in raw_audio.samples.iter() {
		let mut pcm_sample = *sample * 32768f64;

		if pcm_sample > 32768f64 {
			pcm_sample = 32768f64;
		}
		if pcm_sample < -32768f64 {
			pcm_sample = -32768f64;
		}

		try!(file.write_be_i16(pcm_sample as i16));
	}

	Ok(true)

}