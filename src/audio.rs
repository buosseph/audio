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

impl Filter for RawAudio {
	// Only works for mono, need to update for stereo usage
	fn one_pole_lowpass(&mut self, cutoff: f64) {
		let mut sampling_frequency = 44100f64;	// assumption
		let mut db_gain = 0f64;
		let mut q = 0.71f64;	// like Bitwig

		let a = ::std::num::Float::sqrt(
			::std::num::Float::powi( (db_gain / 20f64), 10 )
		);
		let w0: f64 = ::std::f64::consts::PI_2 * cutoff / sampling_frequency;
		let cos_w0 = ::std::num::FloatMath::cos(w0);
		let sin_w0 = ::std::num::FloatMath::sin(w0);
		let alpha = sin_w0 / (2f64 * q);

		// y[n] = (b0/a0)*x[n] + (b1/a0)*x[n-1] + (b2/a0)*x[n-2] 
							// - (a1/a0)*y[n-1] - (a2/a0)*y[n-2]

		let b0 = (1f64 - cos_w0) / 2f64;
		let b1 = 1f64 - cos_w0;
		let b2 = b0;
		let a0 = 1f64 + alpha;
		let a1 = -2f64 * cos_w0;
		let a2 = 1f64 - alpha;

		// lowpass coefficients
		let cx0 = b0/a0;
		let cx1 = b1/a0;
		let cx2 = b2/a0;
		let cy1 = a1/a0;
		let cy2 = a2/a0;

		let mut xn0 = 0f64;	// x[n]
		let mut xn1 = 0f64;	// x[n-1]
		let mut xn2 = 0f64;	// x[n-2]
		let mut yn0 = 0f64;	// y[n]
		let mut yn1 = 0f64;	// y[n-1]
		let mut yn2 = 0f64;	// y[n-2]

		// Assume mono
		for i in range(0, self.samples.len()) {
			xn2 = xn1;
			xn1 = xn0;
			xn0 = self.samples[i];

			yn2 = yn1;
			yn1 = yn0;
			yn0 = cx0 * xn0 + cx1 * xn1 + cx2 * xn2 - cy1 * yn1 - cy2 * yn2;

			self.samples[i] = yn0;
		}
	}
}