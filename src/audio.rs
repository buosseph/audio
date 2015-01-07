// TODOs:
//	- Add raw data reader, may not need own module
//	- Unit tests!
//	- Look into other file types.


/** Terminology (To avoid future confusion)
 *	Sample 	- A single recorded value independent of channel
 *	Frame	- A set of samples, one from each channel, to be played simultaneously
 *	Clip	- A set of frames representing an interval of time within or containing the entire read sound
 */

use std::fmt;

#[derive(Show, Clone, Copy)]
pub enum SampleOrder {
	MONO,
	INTERLEAVED,
	REVERSED,
	PLANAR,
}

#[derive(Clone)]
pub struct RawAudio {
	pub bit_rate: uint,
	pub sample_rate: uint,
	pub channels: uint,
	pub order: SampleOrder,
	pub samples: Vec<f64>,
}

impl RawAudio {
	pub fn print_samples(&self) {
		println!("Samples: {}", self.samples);
	}
}

#[experimental = "waiting on Show stability"]
impl fmt::Show for RawAudio {
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



// pub trait AudioDecoder {
// 	fn read_file(&mut self, file_path: &str) -> IoResult<RawAudio>;

// 	fn le_u8_array_to_i16(array: &[u8; 2]) -> i16{
// 		(array[1] as i16) << 8 | array[0] as i16
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
	fn one_pole_lowpass(&mut self, cutoff: f64);
	fn one_pole_highpass(&mut self, cutoff: f64);
	fn one_pole_bandpass(&mut self, cutoff: f64);
	fn one_pole_bandpass_bw(&mut self, cutoff: f64);
	fn one_pole_notch(&mut self, cutoff: f64);
}