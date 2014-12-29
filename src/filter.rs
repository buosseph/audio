use audio::RawAudio;
use audio::Filter;

impl Filter for RawAudio {
	// Only works for mono, need to update for stereo usage
	fn one_pole_lowpass(&mut self, cutoff: f64) {
		let mut sampling_frequency = self.sampling_rate as f64;
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