use audio::RawAudio;
use audio::SampleOrder;
use audio::OldFilter;



// Can't directly access fields, else can't update coefficents on parameter changes
struct Lowpass {
	sample_rate: f64,
	cutoff: f64,
	q: f64,
	biquad: cd Biquad
}
impl Lowpass {
	fn new(sample_rate: f64, cutoff: f64, q: f64) -> Self {
		let mut biquad = Biquad::new();
		let mut x = Lowpass {
			sample_rate: sample_rate,
			cutoff: cutoff,
			q: q,
			biquad: biquad
		};
		x.calculate_coefficients();
		x
	}

	fn calculate_coefficients(&mut self) {
		// Intermidiates
		let w0 = ::std::f64::consts::PI_2 * self.cutoff / self.sample_rate;
		let cos_w0 = ::std::num::Float::cos(w0);
		let alpha = ::std::num::Float::sin(w0) / (2f64 * self.q);

		let mut b0 = (1f64 - cos_w0) / 2f64;
		let mut b1 = 1f64 - cos_w0;
		let mut b2 = b0;
		let mut a0 = 1f64 + alpha;
		let mut a1 = -2f64 * cos_w0;
		let mut a2 = 1f64 - alpha;

		b0 /= a0;
		b1 /= a0;
		b2 /= a0;
		a1 /= a0;
		a2 /= a0;

		self.biquad.set_coefficients(b0, b1, b2, a1, a2);
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