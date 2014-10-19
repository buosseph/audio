use std::io::{File, IoResult};

// Byte order: Big-endian
// For dealing with extended, use FFI? (http://www.onicos.com/staff/iz/formats/aiff.html)

const FORM = 0x464F524D;
const COMM = 0x434F4D4D;
const SSND = 0x53534E44;

pub struct IffHeader {
	// id: u32, // 0x464F524D => "FORM"
	pub size: u32,
	pub form_type: u32, // 0x41494646 => "AIFF"
}

impl IffHeader {
	pub fn read_chunk(file: &mut File) -> IoResult<RiffHeader> {

		let file_size		= try!(file.read_be_u32());
		let file_form_type	= try!(file.read_be_u32());

		// Verify form_type is "AIFF"

		Ok(
			RiffHeader {
				size: file_size,
				form_type: file_form_type,
			}
		)
	}
}


#[allow(dead_code)]
pub struct CommonChunk {
	//id: u32,					// 0x434F4D4D => "COMM"
	pub size: u32,				// Always 18
	pub num_of_channels: u16,
	pub num_of_frames: u32,		// Frame = set of all channels with single sample
	pub bit_rate: u16,
	// pub sampling_rate: extended // Uses 10 byte extended precision float 
}

// Multi-channel samples are always interleaved
pub struct SoundDataChunk {
	// id: u32, // 0x53534E44 => "SSND"
	pub size: u32,
	pub offset: u32,
	pub block_size: u32,
	pub data: Vec<u8>,	// Uninterpreted data -> needs to be able to be read as i16
}

