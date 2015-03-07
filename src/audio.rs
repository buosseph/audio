// TODOs:
//	- Implement Decoder structs so users can still have access to meta data?
//	- Implement more chunks, specifically for meta data and comments used by Audacity
//	- IDX?
//	- Add raw data reader? (may not need own module)
//	- Look into other file types (it only gets worse from here!)


/** Terminology (To avoid future confusion)
 *	Sample 	- A single recorded value independent of channel
 *	Frame	- A set of samples, one from each channel, to be played simultaneously
 *	Clip	- A set of frames representing an interval of time within or containing the entire read sound
 */

use std::fmt;
use std::old_io::{IoError};
use std::error::{FromError};
use std::ascii::OwnedAsciiExt;

/// An enumeration for keeping track of how samples are organized in the loaded audio.
/// Multichannel samples are usually interleaved, but other orderings are included if they
/// are needed in the furutre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleOrder {
	MONO,
	INTERLEAVED,
	REVERSED,		// Have yet to see anything using these...
	PLANAR,
}

impl fmt::Display for SampleOrder {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(
			formatter,
			"{}",
			self
		)
	}
}

// Rename to AudioBuffer? Struct name is kinda confusing
/// Holds all samples and necessary audio data for processing and encoding.
#[derive(Clone)]
pub struct RawAudio {
	pub bit_rate: uint,
	pub sample_rate: uint,
	pub channels: uint,
	pub order: SampleOrder,
	pub samples: Vec<f64>,
}

impl fmt::Debug for RawAudio {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(
			formatter,
			"RawAudio: \n\tbit_rate: {},\n\tsample_rate: {},\n\tchannels: {},\n\torder: {},\n\t{} samples\n",
			self.bit_rate,
			self.sample_rate,
			self.channels,
			self.order,
			self.samples.len()
		)
	}
}

/// Result type of an audio encoding or decoding process
pub type AudioResult<T> = Result<T, AudioError>;

/// An enumeration for reporting audio errors
#[derive(Debug)]
pub enum AudioError {
	/// The audio file does not match the supported format specification
	FormatError(String),

	/// An IoError occurred during an audio process
	IoError(IoError),

	/// The audio file requires an unsupported feature from the decoder
	UnsupportedError(String)
}

impl FromError<IoError> for AudioError {
	fn from_error(err: IoError) -> AudioError {
		AudioError::IoError(err)
	}
}

/// Loads the audio file into memory from a path. The necessary decoder
/// is determined by the path file extension. An 'AudioError' is 
/// returned if the file type is not supported or if an error occurred
/// in the decoding process.

pub fn load(path: &Path) -> AudioResult<RawAudio> {
	let extension = path.extension_str().map_or("".to_string(), |ext| ext.to_string().into_ascii_lowercase());
	match extension.as_slice() {
		"wav" 	=> super::wave::decoder::read_file(path),
		"aiff" 	=> super::aiff::decoder::read_file(path),
		_ 		=> return Err(AudioError::UnsupportedError("Did not recognize file extension as valid".to_string()))
	}
}

/// Writes the audio file to the provided path. The necessary encoder
/// is determined by the path file extension. An `AudioError` is 
/// returned if the file type is not supported or if an error occurred
/// in the encoding process. 
pub fn save(audio: &RawAudio, path: &Path) -> AudioResult<bool> {
	let extension = path.extension_str().map_or("".to_string(), |ext| ext.to_string().into_ascii_lowercase());
	match extension.as_slice() {
		"wav" 	=> super::wave::encoder::write_file(audio, path),
		"aiff" 	=> super::aiff::encoder::write_file(audio, path),
		_ 		=> return Err(AudioError::UnsupportedError("Did not recognize file extension as valid".to_string()))
	}
}



// pub trait AudioDecoder {
// 	fn read_file(&mut self, file_path: &str) -> IoResult<RawAudio>;
// 	fn le_u8_array_to_i16(array: &[u8; 2]) -> i16{
// 		(array[1] as i16) << 8 | array[0] as i16
// 	}
// 	fn be_u8_array_to_i16(array: &[u8; 2]) -> i16{
// 		(array[0] as i16) << 8 | array[1] as i16
// 	}
// }
// #[test]
// fn test_le_u8_array_to_i16() {
// 	let array: [u8; 4] = [0x24, 0x17, 0x1e, 0xf3];
// 	let case1: &[u8; 2] = &[array[0], array[1]];
// 	let case2: &[u8; 2] = &[array[2], array[3]];
// 	assert_eq!(5924i16, le_u8_array_to_i16(case1));
// 	assert_eq!(-3298i16, le_u8_array_to_i16(case2));
// }


pub trait Utilities {
	fn stereo_to_mono(&mut self) -> bool;
	fn invert(&mut self) -> bool;
	fn reverse_channels(&mut self) -> bool;
	fn reverse(&mut self) -> bool;
	fn full_reverse(&mut self) -> bool;
}

pub trait Dynamics {
	fn amplify(&mut self, gain: f64) -> bool;
}

pub trait Filter {
	// Reference: http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt
	fn biquad_lowpass(&mut self, cutoff: f64);
	fn biquad_highpass(&mut self, cutoff: f64);
	fn biquad_bandpass(&mut self, cutoff: f64);
	fn biquad_bandpass_bw(&mut self, cutoff: f64);
	fn biquad_notch(&mut self, cutoff: f64);
}