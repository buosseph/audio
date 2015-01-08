use std::io::{File, IoResult};
use super::WAVE;

#[derive(Copy)]
pub struct RIFFHeader {
	pub size: u32,
	pub format: u32,
}

impl RIFFHeader {
	pub fn read_chunk(file: &mut File) -> IoResult<RIFFHeader> {
		let mut buffer: [u8; 8] = [0; 8];
		try!(file.read(&mut buffer));

		let file_size		: u32 = (buffer[3] as u32) << 24 | (buffer[2] as u32) << 16 | (buffer[1] as u32) << 8 | buffer[0] as u32;
		let file_type_header: u32 = (buffer[7] as u32) << 24 | (buffer[6] as u32) << 16 | (buffer[5] as u32) << 8 | buffer[4] as u32;

		if file_type_header != WAVE {
			panic!("File is not valid WAVE.");
		}

		Ok(
			RIFFHeader {
				size: file_size,
				format: file_type_header,
			}
		)
	}
}

#[derive(Show, Copy, PartialEq, Eq)]
pub enum CompressionCode {
	Unknown	= 0,
	PCM		= 1,
}

#[derive(Copy)]
pub struct FormatChunk {
	pub size: u32,
	pub compression_code: CompressionCode,
	pub num_of_channels: u16,
	pub sampling_rate: u32,
	pub data_rate: u32,
	pub block_size: u16,
	pub bit_rate: u16,
}

impl FormatChunk {
	pub fn read_chunk(file: &mut File) -> IoResult<FormatChunk> {
		let chunk_size	: u32 		= try!(file.read_le_u32());
		let buffer		: Vec<u8> 	= try!(file.read_exact(chunk_size as uint));

		let compression_u16: u16 = (buffer[1] as u16) << 8 | buffer[0] as u16;
		let compression_code: CompressionCode =
			match compression_u16 {
				1 => CompressionCode::PCM,
				_ => CompressionCode::Unknown,	// Not supporting any other type than PCM
			};
		let num_of_channels	: u16 	= (buffer[3] as u16) << 8 | buffer[2] as u16;
		let sampling_rate	: u32 	= (buffer[7] as u32) << 24 | (buffer[6] as u32) << 16 | (buffer[5] as u32) << 8 | buffer[4] as u32;
		let data_rate		: u32 	= (buffer[11] as u32) << 24 | (buffer[10] as u32) << 16 | (buffer[9] as u32) << 8 | buffer[8] as u32;
		let block_size		: u16 	= (buffer[13] as u16) << 8 | buffer[12] as u16;
		let bit_rate		: u16 	= (buffer[15] as u16) << 8 | buffer[14] as u16;

		// Don't care for other bytes if PCM

		Ok(
			FormatChunk {
				size: chunk_size,
				compression_code: compression_code,
				num_of_channels: num_of_channels,
				sampling_rate: sampling_rate,
				data_rate: data_rate,
				block_size: block_size,
				bit_rate: bit_rate,
			}
		)
	}
}

// Multi-channel samples are always interleaved
pub struct DataChunk {
	pub size: u32,
	pub data: Vec<u8>,	// Uninterpreted data
}

impl DataChunk {
	pub fn read_chunk(file: &mut File) -> IoResult<DataChunk> {
		let size = try!(file.read_le_u32());
		let data = try!(file.read_exact(size as uint));

		Ok(
			DataChunk {
				size: size,
				data: data,
			}
		)
	}
}