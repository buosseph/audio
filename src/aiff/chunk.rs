use std::old_io::{File, IoResult};
use super::AIFF;

#[derive(Copy)]
pub struct IFFHeader {
	pub size: i32,
	pub form_type: i32,
}

impl IFFHeader {
	#[allow(deprecated)]
	pub fn read_chunk(file: &mut File) -> IoResult<IFFHeader> {
		let mut buffer: [u8; 8] = [0; 8];
		try!(file.read(&mut buffer));

		let file_size		: i32 = (buffer[0] as i32) << 24 | (buffer[1] as i32) << 16 | (buffer[2] as i32) << 8 | buffer[3] as i32;
		let file_form_type	: i32 = (buffer[4] as i32) << 24 | (buffer[5] as i32) << 16 | (buffer[6] as i32) << 8 | buffer[7] as i32;

		if file_form_type != AIFF {
			panic!("File is not valid AIFF.");
		}

		Ok(
			IFFHeader {
				size: file_size,
				form_type: file_form_type
			}
		)
	}
}

// Required Chunk
#[derive(Copy)]
pub struct CommonChunk {
	pub size: i32,
	pub num_of_channels: i16,
	pub num_of_frames: u32,
	pub bit_rate: i16,
	pub sample_rate: f64 		// Represented as 10 byte extended precision float 
}

impl CommonChunk {
	#[allow(deprecated)]
	pub fn read_chunk(file: &mut File) -> IoResult<CommonChunk> {
		let chunk_size 	: i32 		= try!(file.read_be_i32());
		let buffer		: Vec<u8> 	= try!(file.read_exact(chunk_size as uint));

		let num_of_channels	: i16 	= (buffer[0] as i16) << 8 | buffer[1] as i16;
		let num_of_frames	: u32 	= (buffer[2] as u32) << 24 | (buffer[3] as u32) << 16 | (buffer[4] as u32) << 8 | buffer[5] as u32;
		let bit_rate		: i16 	= (buffer[6] as i16) << 8 | buffer[7] as i16;
		let extended		: &[u8] = &buffer[8..18];
		let sample_rate		: f64 	= convert_from_ieee_extended(extended);

		Ok(
			CommonChunk {
				size: chunk_size,
				num_of_channels: num_of_channels,
				num_of_frames: num_of_frames,
				bit_rate: bit_rate,
				sample_rate: sample_rate
			}
		)
	}
}

// Multi-channel samples are always interleaved
// Required Chunk
pub struct SoundDataChunk {
	pub size: i32,			// Includes offset and block_size => (data_size + 8)
	pub offset: u32,
	pub block_size: u32,
	pub data: Vec<u8>
}

impl SoundDataChunk {
	#[allow(deprecated)]
	pub fn read_chunk(file: &mut File) -> IoResult<SoundDataChunk> {
		let chunk_size 	: i32 = try!(file.read_be_i32());
		let offset 		: u32 = try!(file.read_be_u32());
		let block_size 	: u32 = try!(file.read_be_u32());

		if offset > 0 || block_size > 0 {
			panic!("Can't read block-aligned data!");
		}

		let data: Vec<u8> = try!(file.read_exact(chunk_size as uint - 8));

		Ok(
			SoundDataChunk {
				size: chunk_size,
				offset: offset,
				block_size: block_size,
				data: data
			}
		)
	}
}

fn ieee_u32_to_f64(num: u32) -> f64 {
	((num - 2147483647u32 - 1) as i32) as f64 + 2147483648f64
}

fn convert_from_ieee_extended(bytes: &[u8]) -> f64 {
	let mut num: f64;
	let mut exponent: int;
	let mut hi_mant: u32;
	let mut low_mant: u32;

	exponent = ( ((bytes[0] as u16 & 0x7f) << 8) | (bytes[1] & 0xff) as u16 ) as int;
	hi_mant = 	(bytes[2] as u32 & 0xff)	<< 24
			| 	(bytes[3] as u32 & 0xff)	<< 16
			| 	(bytes[4] as u32 & 0xff)	<< 8
			| 	(bytes[5] as u32 & 0xff);
	low_mant = 	(bytes[6] as u32 & 0xff) 	<< 24
			| 	(bytes[7] as u32 & 0xff) 	<< 16
			| 	(bytes[8] as u32 & 0xff) 	<< 8
			| 	(bytes[9] as u32 & 0xff);

	if exponent == 0 && hi_mant == 0 && low_mant == 0 {
		return 0f64;
	}

	if exponent == 0x7fff {
		panic!("Sampling rate is not a number!");
	}
	else {
		exponent -= 16383;
		exponent -= 31;
		num	= ::std::num::Float::ldexp(ieee_u32_to_f64(hi_mant), exponent);		
		exponent -= 32;
		num  += ::std::num::Float::ldexp(ieee_u32_to_f64(low_mant), exponent);
	}

	if bytes[0] & 0x80 > 0 {
		return -num;
	}
	else {
		return num;
	}
}
