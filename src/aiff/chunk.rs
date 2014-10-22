use std::io::{File, IoResult};

// Byte order: Big-endian
// For dealing with extended, use FFI? (http://www.onicos.com/staff/iz/formats/aiff.html)

pub struct IFFHeader {
	// id: u32, // 0x464F524D => "FORM"
	pub size: i32, // size + 8 = total size including this and id
	pub form_type: i32, // 0x41494646 => "AIFF"
}

impl IFFHeader {
	pub fn read_chunk(file: &mut File) -> IoResult<IFFHeader> {

		let file_size		= try!(file.read_be_i32());
		let file_form_type	= try!(file.read_be_i32());

		// Verify form_type is "AIFF"
		Ok(
			IFFHeader {
				size: file_size,
				form_type: file_form_type,
			}
		)
	}
}


// Required Chunk
#[allow(dead_code)]
pub struct CommonChunk {
	//id: u32,					// 0x434F4D4D => "COMM"
	pub size: i32,				// Always 18
	pub num_of_channels: i16,
	pub num_of_frames: u32,		// Frame = set of all channels with single sample
	pub bit_rate: i16,
	pub sampling_rate: Vec<u8> // : extended // Uses 10 byte extended precision float 
}

impl CommonChunk {
	pub fn read_chunk(file: &mut File) -> IoResult<CommonChunk> {
		let size 			= try!(file.read_be_i32());
		let num_of_channels = try!(file.read_be_i16());
		let num_of_frames 	= try!(file.read_be_u32());
		let bit_rate 		= try!(file.read_be_i16());

		// Convert to sampling_rate as uint
		let extended 		= file.read_exact(10).unwrap();

		Ok(
			CommonChunk {
				size: size,
				num_of_channels: num_of_channels,
				num_of_frames: num_of_frames,
				bit_rate: bit_rate,
				sampling_rate: extended,
			}
		)
	}
}

// Multi-channel samples are always interleaved
// Required Chunk
#[allow(dead_code)]
pub struct SoundDataChunk {
	// id: u32, // 0x53534E44 => "SSND"
	pub size: i32, // Includes offset and block_size => (data_size + 8)
	pub offset: u32,
	pub block_size: u32,
	//pub data: Vec<u8>,	// Uninterpreted data -> needs to be able to be read as i16
}

impl SoundDataChunk {
	pub fn read_chunk(file: &mut File) -> IoResult<SoundDataChunk> {
		let size = try!(file.read_be_i32());
		let offset = try!(file.read_be_u32());
		let block_size = try!(file.read_be_u32());

		// Check block-aligning of data before reading it

		Ok(
			SoundDataChunk {
				size: size,
				offset: offset,
				block_size: block_size,
			}
		)
	}
}
