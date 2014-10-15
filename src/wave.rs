//use audio::AudioDecoder;

//use audio::Audio;

// use std::str;
// use std::io::File;
// use std::path::posix::{Path};

/*
	Sample = singular f32 value (independent of channel)
	Clip = Group of samples along time domain (Should always include all channels)

	Separate channels into separate tracks for processing
*/

/*
#[deriving(Show)]
pub enum SampleOrder {
	MONO,
	INTERLEAVED,
	REVERSED,
	PLANAR,
}


#[deriving(Show)]
pub struct RawAudio {
	pub bit_rate: uint,
	pub sampling_rate: uint,
	pub num_of_channels: uint,
	pub order: SampleOrder,
	pub samples: Vec<f32>,
}*/

/*
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

}*/

// pub fn meta_data(audio: Audio) {
// 	println!("bit_rate: {}, sampling_rate: {}, num_of_channels: {}, order: {}", audio.bit_rate, audio.sampling_rate, audio.num_of_channels, audio.order);
// }

/*
#[allow(unreachable_code)]
pub fn read_file(wav_file_path: &str) -> RawAudio {
	
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
		File size: {} bytes
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
			


			// Reading:
			// - Check if PCM
			// - Check bitrate
			// - Check channels and block size
			

			let number_of_samples: uint = data_size as uint / num_of_channels as uint;
			if format_tag == 1 {
				match bit_rate {

					// Uses signed ints (8-bit uses uints)
					16 => {
						match (num_of_channels, block_size) {
							// Stereo
							(2, 4) => {
								let mut data: Vec<f32> = Vec::with_capacity(number_of_samples as uint);
								for _ in range(0, number_of_samples as uint) {
									match wav_file.read_le_i16() {
										Ok(sample) => {
											let float_sample = sample as f32 / 32768f32;
											data.push(float_sample);
										},
										Err(e)	=> {
											fail!("Error: {}", e);
										}
									}
								}

								// Assume interleved for now
								RawAudio{ bit_rate: bit_rate as uint, sampling_rate: samples_per_sec as uint, num_of_channels: num_of_channels as uint, order: INTERLEAVED, samples: data}

							},

							// Mono
							(1, 2) => {
								
								let mut data: Vec<f32> = Vec::with_capacity(number_of_samples as uint);
								for _ in range(0, number_of_samples as uint) {
									match wav_file.read_le_i16() {
										Ok(sample) => {
											let float_sample = sample as f32 / 32768f32;
											data.push(float_sample);
										},
										Err(e)	=> {
											fail!("Error: {}", e);
										}
									}
								}

								RawAudio{ bit_rate: bit_rate as uint, sampling_rate: samples_per_sec as uint, num_of_channels: num_of_channels as uint, order: MONO, samples: data}

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
*/

/*
// Only allow writing as PCM at the moment
pub fn write_file(raw_audio: RawAudio, wav_file_path: &str) -> bool {

	let path = Path::new(wav_file_path);
	let mut wav_file = File::create(&path);

	let num_of_channels: u16 = raw_audio.num_of_channels as u16;
	let samples_per_sec: u32 = raw_audio.sampling_rate as u32;
	let data_rate: u32 = (raw_audio.sampling_rate * raw_audio.num_of_channels * raw_audio.bit_rate / 8) as u32;
	let bit_rate: u16 = raw_audio.bit_rate as u16;
	let block_size: uint = raw_audio.num_of_channels * raw_audio.bit_rate / 8;



	// Assume 44 byte header for now
	let riff_header = "RIFF";
	let file_size: u32 =  (4 + 8 + 16 + 8 + raw_audio.samples.len() * block_size / raw_audio.num_of_channels) as u32;	// = 4 + (8 + fmt_chunk size) + (8 + (data_chunk size * block_size)) (NOTE: 8 bytes are purposely missing for riff_header and file_size)
	let file_type_header = "WAVE";

	// Audio format as determined as function argument? 
	let format_chunk_marker = "fmt ";
	let format_chunk_length: u32 = 16;		// 16 if PCM
	let format_tag: u16 = 1;				// 1 if PCM

	// NOTE: variables above are determined by audio format (e.g. PCM, float, A-Law, etc.)



	let data_chunk_header = "data";
	let data_size: u32 = (raw_audio.samples.len() * block_size / raw_audio.num_of_channels) as u32;		// = data_chunk size * block_size




	wav_file.write_str(riff_header).unwrap();
	wav_file.write_le_u32(file_size).unwrap();
	wav_file.write_str(file_type_header).unwrap();

	wav_file.write_str(format_chunk_marker).unwrap();
	wav_file.write_le_u32(format_chunk_length).unwrap();
	wav_file.write_le_u16(format_tag).unwrap();
	wav_file.write_le_u16(num_of_channels).unwrap();
	wav_file.write_le_u32(samples_per_sec).unwrap();
	wav_file.write_le_u32(data_rate).unwrap();
	wav_file.write_le_u16(block_size as u16).unwrap();
	wav_file.write_le_u16(bit_rate).unwrap();

	wav_file.write_str(data_chunk_header).unwrap();
	wav_file.write_le_u32(data_size).unwrap();



	for sample in raw_audio.samples.iter() {

		let mut pcm_sample = sample * 32768f32;

		if pcm_sample > 32767f32 {
			pcm_sample = 32767f32;
		}
		if pcm_sample < -32768f32 {
			pcm_sample = -32768f32;
		}

		wav_file.write_le_i16(pcm_sample as i16).unwrap();

	}

	true

}*/

/*
// Process
impl RawAudio {

	//	Can't use decibel values until std::num::Float is further implemented
	//	gain = 10^(decibels / 20), decibels = 20 * log10(gain)
	//	Amplify by 7.94328 (18db)
	//	Allows clipping
	pub fn amplify(&mut self, gain: f32) -> bool {
		for sample in self.samples.mut_iter() {
			*sample = *sample * gain;
		}
		true
	}

	// Issue: result is half length of desired
	pub fn stereo_to_mono(&mut self) -> bool {
		match self.num_of_channels {
			2 => {
				let mut mono_buffer: Vec<f32> = Vec::with_capacity(self.samples.len() / 2);
				let mut first_channel_value: f32 = 0.;
				for i in range(0, self.samples.len()) {
					// Every second value
					if i % 2 == 1 {
						let second_channel_value: f32 = *self.samples.get_mut(i);
						let mono_value: f32 = (first_channel_value + second_channel_value) / 2.;
						mono_buffer.push(mono_value);
					}
					first_channel_value = *self.samples.get_mut(i);
				}
				self.samples = mono_buffer;

				true
			},
			_ => false
		}
	}

	// Create test to write and test checking for phase cancellation
	pub fn invert(&mut self) -> bool {
		for sample in self.samples.mut_iter() {
			*sample = -*sample;
		}
		true
	}

	pub fn reverse_channels(&mut self) -> bool {
		for i in range(0, self.samples.len()) {
			if i % self.num_of_channels == self.num_of_channels - 1 {

				for j in range(0, self.num_of_channels / 2) {
					let left_index = i - ( (i - j) % self.num_of_channels );
					let right_index = i - j;

					if left_index != right_index {
						let temp = *self.samples.get_mut(left_index);
						*self.samples.get_mut(left_index) = *self.samples.get_mut(right_index);
						*self.samples.get_mut(right_index) = temp;
					}
				}

			}
		}

		true
	}

	pub fn reverse(&mut self) -> bool {
		self.samples.reverse();
		self.reverse_channels()
	}

	// Also reverses channels
	pub fn full_reverse(&mut self) -> bool {
		self.samples.reverse();
		true
	}

	pub fn delay(&mut self) -> bool {
		let delay: f32 = 1.0;
		let decay: f32 = 0.5;

		let blockSize = (self.sampling_rate as f32 * delay) as int;

		if blockSize < 1 || blockSize > self.samples.len() as int {
			return false
		}

		for sample in self.samples.mut_iter() {
			*sample = *sample + *sample * decay;
		}
		return true

	}
}
*/

/*
#[cfg(test)]
mod tests {
	use super::*;

	// Tests need fixing
	#[test]
	fn test_write_file() {
		let test_files = vec!(
		"test-pcm-mono.wav",
		"test-pcm-stereo.wav",
		);
		
		for filename in test_files.iter() {
			let read_prefix = "../wav/".to_string();
			let path_to_read = read_prefix.append(*filename);

			let raw_audio = read_file(path_to_read.as_slice());
			println!("{}", raw_audio);

			let write_prefix = "../test/wav/".to_string();
			let path_to_write = write_prefix.append(*filename);

			assert!(write_file(raw_audio, path_to_write.as_slice()));

		}
	}

}*/

