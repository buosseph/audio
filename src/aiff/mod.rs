pub mod chunk;
pub mod decoder;
pub mod encoder;

// Hex constants are stored, read, and written as big endian
const FORM: i32 = 0x464F524D;
const AIFF: i32 = 0x41494646;
const COMM: i32 = 0x434F4D4D;
const SSND: i32 = 0x53534E44;


#[cfg(test)]
mod tests {
	#[test]
	fn test_read_write_eq() {
		use super::*;
		// Stereo
		let data = "Warrior Concerto - no meta.aiff";
		let mut audio = decoder::read_file(data).unwrap();
		let total_samples = audio.samples.len();
		let channels = audio.num_of_channels;
		let bit_rate = audio.bit_rate;
		let sample_rate = audio.sampling_rate;

		let written = encoder::write_file(audio, "tmp.aiff").unwrap();
		assert!(written);

		let verify = decoder::read_file("tmp.aiff").unwrap();

		// Assert written file is same length as read file!
		assert_eq!(total_samples, verify.samples.len());
		assert_eq!(channels, verify.num_of_channels);
		assert_eq!(bit_rate, verify.bit_rate);
		assert_eq!(sample_rate, verify.sampling_rate);
	}
}