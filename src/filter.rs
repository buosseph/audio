use audio::RawAudio;
use audio::SampleOrder;
use audio::Filter;

impl Filter for RawAudio {
	// y[n] = (b0/a0)*x[n] + (b1/a0)*x[n-1] + (b2/a0)*x[n-2] 
						// - (a1/a0)*y[n-1] - (a2/a0)*y[n-2]

	fn one_pole_lowpass(&mut self, cutoff: f64) {
		let sampling_frequency = self.sample_rate as f64;
		let q = 0.71f64;	// like Bitwig

		// Intermidiates
		let w0: f64 = ::std::f64::consts::PI_2 * cutoff / sampling_frequency;
		let cos_w0 = ::std::num::FloatMath::cos(w0);
		let sin_w0 = ::std::num::FloatMath::sin(w0);
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
}