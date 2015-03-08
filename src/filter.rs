use audio::RawAudio;
use audio::SampleOrder;
use audio::OldFilter;

/// A trait for filters
pub trait Filter {
	/// Constructs new filter
	fn new() -> Self;
	
	/// Returns filtered sample value and stores input and output to memory
	fn tick(&mut self, sample: f64) -> f64;
	// fn tick(&mut self, buffer: &Vec<f64>);

	// For abstract filter struct (later)
	// /// Sets the coefficients, the feedforwards and feedbacks, for the filter.
	// set_coefficients(&mut self, b: &Vec<f64>, a: &Vec<f64>);

	/// Resets memory of all previous input and output to zero
	fn flush_memory(&mut self);
}

/// Second-order filter
///
/// A `Biquad` is a type of second-order `Filter` that uses the following equation:
/// > y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]
pub struct Biquad {
	x_z1: f64,
	x_z2: f64,
	y_z1: f64,
	y_z2: f64,
	b0: f64,
	b1: f64,
	b2: f64,
	a1: f64,
	a2: f64
}
impl Biquad {
	fn set_coefficients(&mut self, b0: f64, b1: f64, b2: f64, a1: f64, a2: f64) {
		self.b0 = b0;
		self.b1 = b1;
		self.b2 = b2;
		self.a1 = a1;
		self.a2 = a2;
	}
}
impl Filter for Biquad {
	fn new() -> Self {
		Biquad {
			x_z1: 0f64,
			x_z2: 0f64,
			y_z1: 0f64,
			y_z2: 0f64,
			b0: 1f64,
			b1: 0f64,
			b2: 0f64,
			a1: 0f64,
			a2: 0f64
		}
	}
	fn tick(&mut self, sample: f64) -> f64 {
		let output = self.b0 * sample
			+ self.b1 * self.x_z1 + self.b2 * self.x_z2
			- self.a1 * self.y_z1 - self.a2 * self.y_z2;

		self.x_z2 = self.x_z1;
		self.x_z1 = sample;
		self.y_z2 = self.y_z1;
		self.y_z1 = output;

		output
	}
	fn flush_memory(&mut self) {
		self.x_z1 = 0f64;
		self.x_z2 = 0f64;
		self.y_z1 = 0f64;
		self.y_z2 = 0f64;
	}
}

#[deprecated]
impl OldFilter for RawAudio {

	fn biquad_lowpass(&mut self, cutoff: f64) {
		let sampling_frequency = self.sample_rate as f64;
		let q = 0.71f64;	// like Bitwig

		// Intermidiates
		let w0: f64 = ::std::f64::consts::PI_2 * cutoff / sampling_frequency;
		let cos_w0 = ::std::num::Float::cos(w0);
		let sin_w0 = ::std::num::Float::sin(w0);
		let alpha = sin_w0 / (2f64 * q);

		let b0 = (1f64 - cos_w0) / 2f64;
		let b1 = 1f64 - cos_w0;
		let b2 = b0;
		let a0 = 1f64 + alpha;
		let a1 = -2f64 * cos_w0;
		let a2 = 1f64 - alpha;

		// Coefficients
		let cx0 = b0/a0;
		let cx1 = b1/a0;
		let cx2 = b2/a0;
		let cy1 = a1/a0;
		let cy2 = a2/a0;

		for i in range(0, self.channels) {
			let mut xn0 = 0f64;	// x[n]
			let mut xn1 = 0f64;	// x[n-1]
			let mut xn2;		// x[n-2]
			let mut yn0 = 0f64;	// y[n]
			let mut yn1 = 0f64;	// y[n-1]
			let mut yn2;		// y[n-2]

			match self.order {
				SampleOrder::INTERLEAVED => {
					for j in range(0, self.samples.len()) {
						if j % self.channels == i {
							xn2 = xn1;
							xn1 = xn0;
							xn0 = self.samples[j];

							yn2 = yn1;
							yn1 = yn0;
							yn0 = cx0 * xn0 + cx1 * xn1 + cx2 * xn2 - cy1 * yn1 - cy2 * yn2;

							self.samples[j] = yn0;
						}
					}
				},
				_ => {panic!("Samples must be INTERLEAVED for filter")}
			}
		}
	}

	fn biquad_highpass(&mut self, cutoff: f64) {
		let sampling_frequency = self.sample_rate as f64;
		let q = 0.71f64;	// like Bitwig

		// Intermidiates
		let w0: f64 = ::std::f64::consts::PI_2 * cutoff / sampling_frequency;
		let cos_w0 = ::std::num::Float::cos(w0);
		let sin_w0 = ::std::num::Float::sin(w0);
		let alpha = sin_w0 / (2f64 * q);

		// let b0 = (1f64 - cos_w0) / 2f64;		// creates fun hard distortion!
		let b0 = (1f64 + cos_w0) / 2f64;
		let b1 = -1f64 * (1f64 + cos_w0);
		let b2 = b0;
		let a0 = 1f64 + alpha;
		let a1 = -2f64 * cos_w0;
		let a2 = 1f64 - alpha;

		// Coefficients
		let cx0 = b0/a0;
		let cx1 = b1/a0;
		let cx2 = b2/a0;
		let cy1 = a1/a0;
		let cy2 = a2/a0;

		for i in range(0, self.channels) {
			let mut xn0 = 0f64;	// x[n]
			let mut xn1 = 0f64;	// x[n-1]
			let mut xn2;		// x[n-2]
			let mut yn0 = 0f64;	// y[n]
			let mut yn1 = 0f64;	// y[n-1]
			let mut yn2;		// y[n-2]

			match self.order {
				SampleOrder::INTERLEAVED => {
					for j in range(0, self.samples.len()) {
						if j % self.channels == i {
							xn2 = xn1;
							xn1 = xn0;
							xn0 = self.samples[j];

							yn2 = yn1;
							yn1 = yn0;
							yn0 = cx0 * xn0 + cx1 * xn1 + cx2 * xn2 - cy1 * yn1 - cy2 * yn2;

							self.samples[j] = yn0;
						}
					}
				},
				_ => {panic!("Samples must be INTERLEAVED for filter")}
			}
		}
	}

	// No bandwidth control (bw)
	// Most similar to AUBandpass
	fn biquad_bandpass(&mut self, cutoff: f64) {
		let sampling_frequency = self.sample_rate as f64;
		let q = 0.71f64;

		// Intermidiates
		let w0: f64 = ::std::f64::consts::PI_2 * cutoff / sampling_frequency;
		let cos_w0 = ::std::num::Float::cos(w0);
		let sin_w0 = ::std::num::Float::sin(w0);
		let alpha = sin_w0 / (2f64 * q);

		let b0 = q * alpha;
		let b1 = 0f64;
		let b2 = -1f64 * b0;
		let a0 = 1f64 + alpha;
		let a1 = -2f64 * cos_w0;
		let a2 = 1f64 - alpha;

		// Coefficients
		let cx0 = b0/a0;
		let cx1 = b1/a0;
		let cx2 = b2/a0;
		let cy1 = a1/a0;
		let cy2 = a2/a0;

		for i in range(0, self.channels) {
			let mut xn0 = 0f64;	// x[n]
			let mut xn1 = 0f64;	// x[n-1]
			let mut xn2;		// x[n-2]
			let mut yn0 = 0f64;	// y[n]
			let mut yn1 = 0f64;	// y[n-1]
			let mut yn2;		// y[n-2]

			match self.order {
				SampleOrder::INTERLEAVED => {
					for j in range(0, self.samples.len()) {
						if j % self.channels == i {
							xn2 = xn1;
							xn1 = xn0;
							xn0 = self.samples[j];

							yn2 = yn1;
							yn1 = yn0;
							yn0 = cx0 * xn0 + cx1 * xn1 + cx2 * xn2 - cy1 * yn1 - cy2 * yn2;

							self.samples[j] = yn0;
						}
					}
				},
				_ => {panic!("Samples must be INTERLEAVED for filter")}
			}
		}
	}

	fn biquad_bandpass_bw(&mut self, cutoff: f64) {
		let sampling_frequency = self.sample_rate as f64;
		let q = 0.71f64;
		let bw = 1f64;	// in octaves

		// Intermidiates
		let w0: f64 = ::std::f64::consts::PI_2 * cutoff / sampling_frequency;
		let cos_w0 = ::std::num::Float::cos(w0);
		let sin_w0 = ::std::num::Float::sin(w0);
		let alpha: f64 = sin_w0 /
			::std::num::Float::sinh(
				::std::num::Float::ln(2f64)
				/ 2f64 * bw * w0 / sin_w0
			);

		let b0 = q * alpha;
		let b1 = 0f64;
		let b2 = -1f64 * b0;
		let a0 = 1f64 + alpha;
		let a1 = -2f64 * cos_w0;
		let a2 = 1f64 - alpha;

		// Coefficients
		let cx0 = b0/a0;
		let cx1 = b1/a0;
		let cx2 = b2/a0;
		let cy1 = a1/a0;
		let cy2 = a2/a0;

		for i in range(0, self.channels) {
			let mut xn0 = 0f64;	// x[n]
			let mut xn1 = 0f64;	// x[n-1]
			let mut xn2;		// x[n-2]
			let mut yn0 = 0f64;	// y[n]
			let mut yn1 = 0f64;	// y[n-1]
			let mut yn2;		// y[n-2]

			match self.order {
				SampleOrder::INTERLEAVED => {
					for j in range(0, self.samples.len()) {
						if j % self.channels == i {
							xn2 = xn1;
							xn1 = xn0;
							xn0 = self.samples[j];

							yn2 = yn1;
							yn1 = yn0;
							yn0 = cx0 * xn0 + cx1 * xn1 + cx2 * xn2 - cy1 * yn1 - cy2 * yn2;

							self.samples[j] = yn0;
						}
					}
				},
				_ => {panic!("Samples must be INTERLEAVED for filter")}
			}
		}
	}

	// Need verification
	fn biquad_notch(&mut self, cutoff: f64) {
		let sampling_frequency = self.sample_rate as f64;
		let q = 0.71f64;

		// Intermidiates
		let w0: f64 = ::std::f64::consts::PI_2 * cutoff / sampling_frequency;
		let cos_w0 = ::std::num::Float::cos(w0);
		let sin_w0 = ::std::num::Float::sin(w0);
		let alpha = sin_w0 / (2f64 * q);

		let b0 = 1f64;
		let b1 = -2f64 * cos_w0;
		let b2 = 1f64;
		let a0 = 1f64 + alpha;
		let a1 = -2f64 * cos_w0;
		let a2 = 1f64 - alpha;

		// Coefficients
		let cx0 = b0/a0;
		let cx1 = b1/a0;
		let cx2 = b2/a0;
		let cy1 = a1/a0;
		let cy2 = a2/a0;

		for i in range(0, self.channels) {
			let mut xn0 = 0f64;	// x[n]
			let mut xn1 = 0f64;	// x[n-1]
			let mut xn2;		// x[n-2]
			let mut yn0 = 0f64;	// y[n]
			let mut yn1 = 0f64;	// y[n-1]
			let mut yn2;		// y[n-2]

			match self.order {
				SampleOrder::INTERLEAVED => {
					for j in range(0, self.samples.len()) {
						if j % self.channels == i {
							xn2 = xn1;
							xn1 = xn0;
							xn0 = self.samples[j];

							yn2 = yn1;
							yn1 = yn0;
							yn0 = cx0 * xn0 + cx1 * xn1 + cx2 * xn2 - cy1 * yn1 - cy2 * yn2;

							self.samples[j] = yn0;
						}
					}
				},
				_ => {panic!("Samples must be INTERLEAVED for filter")}
			}
		}
	}
}