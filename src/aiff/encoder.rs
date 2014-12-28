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

	let num_of_channels	: u16 	= raw_audio.num_of_channels as u16;
	let sampling_rate	: uint 	= raw_audio.sampling_rate;
	let bit_rate		: u16 	= raw_audio.bit_rate as u16;


	let offset			: u32 	= 0;
	let block_size		: uint 	= raw_audio.num_of_channels * raw_audio.bit_rate / 8;
	let data_size		: u32 	= (raw_audio.samples.len() * block_size) as u32;
	let ssnd_chunk_size	: u32 	= 8 + data_size;


	let total_bytes 					= 12 + 26 + 8 + ssnd_chunk_size;	// = [FORM: 12] + [COMM: 26] + [SSND: 8 + chunk_size]
	let file_size			: u32 		= total_bytes - 8;
	let num_of_frames		: u32 		= (raw_audio.samples.len() / raw_audio.num_of_channels) as u32;
	let sample_rate_buffer	: Vec<u8> 	= convert_to_ieee_extended(sampling_rate);

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

fn ieee_f64_to_u32(num: f64) -> u32 {
	((((num - 2147483648f64) as i32) + 2147483647i32) + 1) as u32
}

fn convert_to_ieee_extended(sample_rate: uint) -> Vec<u8>{
	if sample_rate == 0 {
		let vec: Vec<u8> = vec![0,0,0,0,0,0,0,0,0,0];
		return vec;
	}

	let mut num 	: f64 = sample_rate as f64;
	let mut exponent: int;
	let mut f_mant 	: f64;
	let mut fs_mant	: f64;
	let mut hi_mant	: u32;
	let mut low_mant: u32;


	let sign: int = match num < 0f64 {
		true => { num *= -1f64; 0x8000 },
		false => { 0x0000 }
	};

	let tuple = ::std::num::FloatMath::frexp(num);
	f_mant = tuple.0;
	exponent = tuple.1;

	if exponent > 16384 || !(f_mant < 1f64) {
		exponent 	= (sign|0x7fff) as int;
		hi_mant 	= 0;
		low_mant 	= 0;
	}
	else {
		exponent += 16382;
		if exponent < 0 {
			f_mant 		= ::std::num::FloatMath::ldexp(f_mant, exponent);
			exponent 	= 0;
		}

		exponent 	|= sign as int;
		f_mant 		= ::std::num::FloatMath::ldexp(f_mant, 32);
		fs_mant 	= ::std::num::Float::floor(f_mant);
		hi_mant 	= ieee_f64_to_u32(fs_mant);
		f_mant 		= ::std::num::FloatMath::ldexp(f_mant - fs_mant, 32);
		fs_mant 	= ::std::num::Float::floor(f_mant);
		low_mant 	= ieee_f64_to_u32(fs_mant);
	}

	let vec: Vec<u8> = vec![
		(	exponent 		>> 8	)	as u8,
		(	exponent				)	as u8,
		(	hi_mant 		>> 24 	) 	as u8,
		(	hi_mant 		>> 16 	) 	as u8,
		(	hi_mant 		>> 8 	) 	as u8,
			hi_mant 					as u8,
		(	low_mant 		>> 24 	) 	as u8,
		(	low_mant 		>> 16 	) 	as u8,
		(	low_mant 		>> 8 	) 	as u8,
			low_mant 					as u8
	];

	return vec;
}
