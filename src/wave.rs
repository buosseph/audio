

use std::str;
use std::io::File;
use std::path::posix::{Path};


/*pub mod pcm {
	pub struct PCM<T> {
		num_of_channels: uint;
		data: Vec<T>
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


#[allow(unreachable_code)]
pub fn get_audio(wav_file_path: &str) {
	
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


			/* Reading:
			 * - Check if PCM
			 * - Check bitrate
			 * - Check channels and block size
			 */

			if format_tag == 1 {
				match bit_rate {

					// Uses signed ints (8-bit uses uints)
					16 => {
						match (num_of_channels, block_size) {
							// Stereo
							(2, 4) => {

								// Vec holds each channel sample independently for now (e.g. data[0] = L, data[1] = R)
								let mut data: Vec<i16> = Vec::with_capacity(data_size as uint);
								for i in range(0, data_size) {
									match wav_file.read_le_i16() {
										Ok(sample) => {
											if i % 2 == 0 {
												println!("L: {}", sample);
											}
											else {
												println!("R: {}", sample);
											}
											data.push(sample);
										},
										Err(e)	=> {
											println!("{}", e);	// EOF
											break; 
										}
									}
								}

								// Can't return Vec as will be different types depending on bitrate
								//data

							},

							// Mono
							(1, 2) => {

								//unimplemented!();

								// Is this suppose to be unsigned?
								let mut data: Vec<u16> = Vec::with_capacity(data_size as uint);
								for i in range(0, data_size) {
									match wav_file.read_le_u16() {
										Ok(sample) => {
											println!("{}: {}", i, sample);
											data.push(sample);
										},
										Err(e)	=> {
											println!("{}", e);	// EOF
											break;
										}
									}
								}

								// Can't return Vec as will be different types depending on bitrate
								//data

							},

							(_, _) => {
								fail!("This file is encoded using an unsupported number of channels.");
							}
						}

					},

					_ => {
						fail!("This file is encoded using an unsupported bitrate.");
					}
				}
			}
			else {
				fail!("This file is not encoded using PCM.");
			}
			
		}
		Err(e) => fail!("{}", e)
	}

}

pub fn write_test_wav(filename: &str) {

	let path = Path::new(filename);
	let mut wav_file = File::create(&path);

	// Assume 44 byte header for now
	let riff_header = "RIFF";
	let file_size: u32 = 88244;
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
	let data_size: u32 = 44100;		// 1 second



	wav_file.write_str(riff_header).unwrap();
	wav_file.write_le_u32(file_size).unwrap();
	wav_file.write_str(file_type_header).unwrap();

	wav_file.write_str(format_chunk_marker).unwrap();
	wav_file.write_le_u32(format_chunk_length).unwrap();
	wav_file.write_le_u16(format_tag).unwrap();
	wav_file.write_le_u16(num_of_channels).unwrap();
	wav_file.write_le_u32(samples_per_sec).unwrap();
	wav_file.write_le_u32(data_rate).unwrap();
	wav_file.write_le_u16(block_size).unwrap();
	wav_file.write_le_u16(bit_rate).unwrap();

	wav_file.write_str(data_chunk_header).unwrap();
	wav_file.write_le_u32(data_size).unwrap();

	for i in range(0, data_size) {
		wav_file.write_le_i16(i as i16).unwrap();
	}

}

#[cfg(test)]
mod tests {
	use std::str;
	use std::io::File;
	use std::path::posix::{Path};

	pub fn write_test_wav(filename: &str) {

		let path = Path::new(filename);
		let mut wav_file = File::create(&path);

		// Assume 44 byte header for now
		let riff_header = "RIFF";
		let file_size: u32 = 88244;
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
		let data_size: u32 = 44100;		// 1 second



		wav_file.write_str(riff_header).unwrap();
		wav_file.write_le_u32(file_size).unwrap();
		wav_file.write_str(file_type_header).unwrap();

		wav_file.write_str(format_chunk_marker).unwrap();
		wav_file.write_le_u32(format_chunk_length).unwrap();
		wav_file.write_le_u16(format_tag).unwrap();
		wav_file.write_le_u16(num_of_channels).unwrap();
		wav_file.write_le_u32(samples_per_sec).unwrap();
		wav_file.write_le_u32(data_rate).unwrap();
		wav_file.write_le_u16(block_size).unwrap();
		wav_file.write_le_u16(bit_rate).unwrap();

		wav_file.write_str(data_chunk_header).unwrap();
		wav_file.write_le_u32(data_size).unwrap();

		for i in range(0, data_size) {
			wav_file.write_le_u16(i as u16).unwrap();
		}

	}
}

