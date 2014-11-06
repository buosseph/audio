use std::io::{File, IoResult};

// Byte-order: Little-endian
// Only implment reading 16-bit (i16) data for now

// Hex Contants must be stored as big endian
//const WAVE: u32 = 0x57415645;

pub struct RIFFHeader {
	// id: u32, // 0x52494646 => "RIFF"
	pub size: u32,
	pub format: u32, // 0x57415645 => "WAVE"
}

impl RIFFHeader {
	// Assume 44 byte header for now
	pub fn read_chunk(file: &mut File) -> IoResult<RIFFHeader> {

		let file_size			= try!(file.read_le_u32());
		let file_type_header	= try!(file.read_le_u32());

		// Verify file_type_header is "WAVE"

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
	//id: u32, // 0x666D7420 => "fmt"
	pub size: u32,
	pub compression_code: CompressionCode,
	pub num_of_channels: u16,
	pub sampling_rate: u32,
	pub data_rate: u32,
	pub block_size: u16,
	pub bit_rate: u16,
	// Extra fmt bytes? (unused in PCM)
}

impl FormatChunk {
	// Does not read for id
	pub fn read_chunk(file: &mut File) -> IoResult<FormatChunk> {
		let chunk_size							= try!(file.read_le_u32());
		let compression_code: CompressionCode	= 
			match file.read_le_u16() {
				Ok(code)	=> {
					match code {
						1 => PCM,
						_ => Unknown,
					}
				},
				Err(e)		=> {fail!("Error: {}", e)},
			};
		let num_of_channels						= try!(file.read_le_u16());
		let sampling_rate						= try!(file.read_le_u32());
		let data_rate							= try!(file.read_le_u32());
		let block_size							= try!(file.read_le_u16());
		let bit_rate							= try!(file.read_le_u16());

		Ok(
			FormatChunk {
				//id: 
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
	// id: u32, // 0x64617461 => "data"
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