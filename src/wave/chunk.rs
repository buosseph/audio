use std::io::{File, IoResult};

use super::WAVE;

// Byte-order: Little-endian

// TODO: Replace fails with IoErrors

pub struct RIFFHeader {
	pub size: u32,
	pub format: u32,
}

impl RIFFHeader {
	pub fn read_chunk(file: &mut File) -> IoResult<RIFFHeader> {
		let file_size			= try!(file.read_le_u32());
		let file_type_header	= try!(file.read_le_u32());

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

#[allow(dead_code)]
#[deriving(Show)]
pub enum CompressionCode {
	Unknown	= 0,
	PCM		= 1,
}

#[allow(dead_code)]
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
		let chunk_size							= try!(file.read_le_u32());
		let compression_code: CompressionCode	= 
			match file.read_le_u16() {
				Ok(code)	=> {
					match code {
						1 => CompressionCode::PCM,
						_ => CompressionCode::Unknown,
					}
				},
				Err(e)		=> {panic!("Error: {}", e)},
			};
		let num_of_channels						= try!(file.read_le_u16());
		let sampling_rate						= try!(file.read_le_u32());
		let data_rate							= try!(file.read_le_u32());
		let block_size							= try!(file.read_le_u16());
		let bit_rate							= try!(file.read_le_u16());

		// Extra bytes (unused in PCM)
		if chunk_size > 16 {
			let extra_length = chunk_size - 16;
			for _ in range(0, extra_length) {
				try!(file.read_u8());
			}
		}

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

// Not used, bad implementation
// Multi-channel samples are always interleaved
pub struct DataChunk {
	pub size: u32,
	pub data: Vec<u8>,	// Uninterpreted data -> needs to be able to be read as i16
}

impl DataChunk {
	// Does not read for id
	pub fn read_chunk(file: &mut File) -> IoResult<DataChunk> {
		// let double_word = wav_file.read_exact(4).unwrap();
		// let data_chunk_header = from_utf8(double_word.as_slice()).unwrap();

		let size = try!(file.read_le_u32());
		let data = try!(file.read_exact(size as uint));	// Data still not interprete based on bit_rate

		Ok(
			DataChunk {
				size: size,
				data: data,
			}
		)
	}
}