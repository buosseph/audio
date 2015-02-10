pub mod chunk;
pub mod decoder;
pub mod encoder;

// Hex constants are stored, read, and written as little endian
const RIFF: u32 = 0x46464952;
const WAVE: u32 = 0x45564157;
const FMT:	u32 = 0x20746D66;
const DATA: u32 = 0x61746164;

#[cfg(test)]
mod tests {
	#[test]
	fn test_read_write_eq() {
		use super::*;

		let folder: String = String::from_str("tests/wav/");
		let files = vec![
			"i16-pcm-mono.wav",
			"i16-pcm-stereo.wav",
			"Warrior Concerto - no meta.wav",
		];

		for file in files.iter() {
			let mut path: String = folder.clone();
			path.push_str(*file);

			let audio = decoder::read_file(&Path::new(path.as_slice())).unwrap();
			let total_samples = audio.samples.len();
			let channels = audio.channels;
			let bit_rate = audio.bit_rate;
			let sample_rate = audio.sample_rate;
			let sample_order = audio.order;

			let written = encoder::write_file(&audio, &Path::new("tmp.wav")).unwrap();
			assert!(written);

			let verify = decoder::read_file(&Path::new("tmp.wav")).unwrap();

			// Assert written file is same length as read file!
			assert_eq!(total_samples, verify.samples.len());
			assert_eq!(channels, verify.channels);
			assert_eq!(bit_rate, verify.bit_rate);
			assert_eq!(sample_rate, verify.sample_rate);
			assert_eq!(sample_order, verify.order);
		}
	}

	#[test]
	fn test_read_i32() {
		use super::*;

		let folder: String = String::from_str("tests/wav/");
		let files = vec![
			"stereo440-i32-44100.wav",
			"mono440-i32-44100.wav",
		];

		for file in files.iter() {
			let mut path: String = folder.clone();
			path.push_str(*file);

			let audio = decoder::read_file(&Path::new(path.as_slice())).unwrap();
			let channels = audio.channels;
			let bit_rate = audio.bit_rate;
			let sample_rate = audio.sample_rate;

			if file.eq(&"stereo440-i32-44100.wav".to_string()) {
				assert_eq!(channels, 2);
			}
			else {
				assert_eq!(channels, 1);
			}
			assert_eq!(bit_rate, 32);
			assert_eq!(sample_rate, 44100);
		}
	}
}