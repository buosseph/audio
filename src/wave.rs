

use std::str;
use std::io::File;
use std::path::posix::{Path};


pub fn read_file_data(wav_file_path: &str) {

	let path = Path::new(wav_file_path);
	match File::open(&path) {
		Ok(mut wav_file) => {

			// Assume 44 byte header for now
			let riff_header = str::from_utf8_owned(wav_file.read_exact(4).unwrap()).unwrap();
			let file_size = wav_file.read_le_u32().unwrap();
			let file_type_header = str::from_utf8_owned(wav_file.read_exact(4).unwrap()).unwrap();

			let format_chunk_marker = str::from_utf8_owned(wav_file.read_exact(4).unwrap()).unwrap();
			let format_chunk_length = wav_file.read_le_u32().unwrap();
			let format_tag = wav_file.read_le_u16().unwrap();

			let num_of_channels = wav_file.read_le_u16().unwrap();
			let samples_per_sec = wav_file.read_le_u32().unwrap();
			let data_rate = wav_file.read_le_u32().unwrap();
			let block_size = wav_file.read_le_u16().unwrap();
			let bit_rate = wav_file.read_le_u16().unwrap();

			let data_chunk_header = str::from_utf8_owned(wav_file.read_exact(4).unwrap()).unwrap();
			let data_size = wav_file.read_le_u32().unwrap();


			println!(
				"master_riff_chunk:
					{}
					File size: {}
					File type: {}
				{}_chunk:
					Chunk length: {},
					Format: {} (1 = PCM, 3 = IEEE float, ...),
					Channels: {},
					Sample rate: {},
					Data rate: {},
					Block size: {},
					Bit rate: {}
				{}_chunk:
					Data size: {} bytes
				",
				riff_header,
				file_size,
				file_type_header,
				format_chunk_marker,
				format_chunk_length,
				format_tag,
				num_of_channels,
				samples_per_sec,
				data_rate,
				block_size,
				bit_rate,
				data_chunk_header,
				data_size
				);

		}
		Err(e) => fail!("{}", e)
	}

}

