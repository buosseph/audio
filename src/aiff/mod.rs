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
		
		let folder: String = String::from_str("test/aiff/");
		let files = vec![
			"i16-pcm-mono.aiff",
			"i16-pcm-stereo.aiff",
			"Warrior Concerto - no meta.aiff"
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

			let written = encoder::write_file(&audio, &Path::new("tmp.aiff")).unwrap();
			assert!(written);

			let verify = decoder::read_file(&Path::new("tmp.aiff")).unwrap();

			// Assert written file is same length as read file!
			assert_eq!(total_samples, verify.samples.len());
			assert_eq!(channels, verify.channels);
			assert_eq!(bit_rate, verify.bit_rate);
			assert_eq!(sample_rate, verify.sample_rate);
			assert_eq!(sample_order, verify.order);
		}
	}
}