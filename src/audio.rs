// TODOs:
//	- Add raw data reader, may not need own module
//	- Unit tests!
//	- Look into other file types.


/** Terminology (To avoid future confusion)
 *	Sample 	- A single recorded value independent of channel
 *	Frame	- A set of samples, one from each channel, to be played simultaneously
 *	Clip	- A set of frames representing an interval of time within or containing the entire read sound
 */

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
	pub samples: Vec<f64>,
}

impl RawAudio {
	pub fn print_meta_data(&self) {
		println!("bit_rate: {}, sampling_rate: {}, num_of_channels: {}, order: {}", self.bit_rate, self.sampling_rate, self.num_of_channels, self.order);
	}

	pub fn print_samples(&self) {
		println!("Samples: {}", self.samples);
	}
}

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
}