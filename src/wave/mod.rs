pub mod chunk;
pub mod decoder;
pub mod encoder;

// Hex constants are stored, read, and written as little endian
const RIFF: u32 = 0x46464952;
const WAVE: u32 = 0x45564157;
const FMT:	u32 = 0x20746D66;
const DATA: u32 = 0x61746164;

fn le_u8_array_to_i16(array: &[u8; 2]) -> i16{
	(array[1] as i16) << 8 | array[0] as i16
}
#[test]
fn test_le_u8_array_to_i16() {
	let array: [u8; 4] = [0x24, 0x17, 0x1e, 0xf3];
	let case1: &[u8; 2] = &[array[0], array[1]];
	let case2: &[u8; 2] = &[array[2], array[3]];
	assert_eq!(5924i16, le_u8_array_to_i16(case1));
	assert_eq!(-3298i16, le_u8_array_to_i16(case2));
}


#[cfg(test)]
mod tests {
	#[test]
	fn test_read_write_eq() {
		use super::*;

		let folder: String = String::from_str("test/wav/");
		let files = vec![
			"i16-pcm-mono.wav",
			"i16-pcm-stereo.wav",
			"Warrior Concerto - no meta.wav"
		];

		for file in files.iter() {
			let mut path: String = folder.clone();
			path.push_str(*file);

			let audio = decoder::read_file(path.as_slice()).unwrap();
			let total_samples = audio.samples.len();
			let channels = audio.channels;
			let bit_rate = audio.bit_rate;
			let sample_rate = audio.sample_rate;

			let written = encoder::write_file(audio, "tmp.wav").unwrap();
			assert!(written);

			let verify = decoder::read_file("tmp.wav").unwrap();

			// Assert written file is same length as read file!
			assert_eq!(total_samples, verify.samples.len());
			assert_eq!(channels, verify.channels);
			assert_eq!(bit_rate, verify.bit_rate);
			assert_eq!(sample_rate, verify.sample_rate);
		}
	}
}