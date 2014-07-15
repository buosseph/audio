

use std::str;
use std::io::File;
use std::path::posix::{Path};

/*
pub mod pcm {
	pub struct PCM {
		data
	}
}*/

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
			let data_size = wav_file.read_le_u32().unwrap(); // Read this many bytes for data


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


pub fn get_audio(wav_file_path: &str) -> Vec<i16> {
	
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

			let num_of_channels = wav_file.read_le_u16().unwrap();	// 1 = mono, 2 = stereo
			let samples_per_sec = wav_file.read_le_u32().unwrap();	// 44100, 88200, etc...
			let data_rate = wav_file.read_le_u32().unwrap();		// = samples_per_sec * num_of_channels * bit_rate / 8
			let block_size = wav_file.read_le_u16().unwrap();	// 2 = 1 byte (mono), 4 = 2 bytes (L+R), use this to determine how to read/ = num_of_channels * bit_rate / 8
			let bit_rate = wav_file.read_le_u16().unwrap();		// If 16+ data is signed, 

			let data_chunk_header = str::from_utf8_owned(wav_file.read_exact(4).unwrap()).unwrap();
			let data_size = wav_file.read_le_u32().unwrap(); // Read this many bytes for data


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


			// Assume 16-bit, uses signed ints, doesn't distinguish channels
			let size = data_size as uint / 2;
			let mut data: Vec<i16> = Vec::with_capacity(size);
			loop {
				match wav_file.read_le_i16() {
					Ok(sample) => {
						println!("{}", sample);
						data.push(sample);
					},
					Err(e)	=> {
						// EOF
						println!("Error: {}", e);
						break;
					}
				}
			}
			data
			
			

		}
		Err(e) => fail!("{}", e)
	}

}


[cfg(test)]
mod tests {
	pub fn write_test_wav(filename: &str) {

		let path = Path::new(filename);
		let mut wav_file = File::create(&path);

		// Assume 44 byte header for now
		let riff_header = "RIFF";
		let file_size: u32 = 44100;		// 1 second
		let file_type_header = "WAVE";

		let format_chunk_marker = "fmt ";
		let format_chunk_length: u32 = 16;
		let format_tag: u16 = 1;
		let num_of_channels: u16 = 1;
		let samples_per_sec: u32 = 44100;
		let data_rate: u32 = 88200;
		let block_size: u16 = 2;
		let bit_rate: u16 = 16;

		let data_chunk_header = "data";
		let data_size: u32 = 44100;



		wav_file.write_str(riff_header);
		wav_file.write_le_u32(file_size);
		wav_file.write_str(file_type_header);

		wav_file.write_str(format_chunk_marker);
		wav_file.write_le_u32(format_chunk_length);
		wav_file.write_le_u16(format_tag);
		wav_file.write_le_u16(num_of_channels);
		wav_file.write_le_u32(samples_per_sec);
		wav_file.write_le_u32(data_rate);
		wav_file.write_le_u16(block_size);
		wav_file.write_le_u16(bit_rate);

		wav_file.write_str(data_chunk_header);
		wav_file.write_le_u32(data_size);

		for i in range(0, data_size) {
			wav_file.write_le_u16(i as u16);
		}
		
	}
}

