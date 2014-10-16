use std::io::{File, IoResult};

// Unused
pub trait Chunk {
	fn read_chunk(file: &mut File) -> IoResult<Self>;
}

// Only implment reading 16-bit (i16) data for now


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
	pub data: Vec<f64>,
}

impl DataChunk {
	// Does not read for id
	// For now don't read actual sample data
	pub fn read_chunk(file: &mut File) -> IoResult<DataChunk> {
		// let double_word = wav_file.read_exact(4).unwrap();
		// let data_chunk_header = from_utf8(double_word.as_slice()).unwrap();

		let size = try!(file.read_le_u32()); // Read this many bytes for data

		Ok(
			DataChunk {
				size: size,
				data: vec![0f64],	// temporary
			}
		)
	}
}