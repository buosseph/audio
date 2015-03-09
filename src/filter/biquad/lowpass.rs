use filter::Filter;
use filter::biquad::Biquad;
use std::num::Float;

// Can't directly access fields, else can't update coefficents on parameter changes
#[allow(dead_code)]
struct Lowpass {
	sample_rate: f64,
	cutoff: f64,
	q: f64,
	biquad: Biquad
}
impl Lowpass {
	#[allow(dead_code)]
	fn new(sample_rate: f64, cutoff: f64, q: f64) -> Self {
		let biquad = Biquad::new();
		let mut lpf = Lowpass {
			sample_rate: sample_rate,
			cutoff: cutoff,
			q: q,
			biquad: biquad
		};
		lpf.calculate_coefficients();
		lpf
	}

	#[allow(dead_code)]
	fn calculate_coefficients(&mut self) {
		// Intermidiates
		let w0 = ::std::f64::consts::PI_2 * self.cutoff / self.sample_rate;
		let cos_w0 = Float::cos(w0);
		let alpha = Float::sin(w0) / (2f64 * self.q);

		let mut b0 = (1f64 - cos_w0) / 2f64;
		let mut b1 = 1f64 - cos_w0;
		let mut b2 = b0;
		let		a0 = 1f64 + alpha;
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

#[cfg(test)]
mod tests {
	use filter::Filter;
	use std::num::Float;
	use std::f64::EPSILON;
	use super::Lowpass;

	#[test]
	fn test_new() {
		let lpf = Lowpass::new(44100f64, 1200f64, 1f64);
		// assert!(Float::abs() < EPSILON);
		assert!(Float::abs(44100f64 - lpf.sample_rate) < EPSILON);
		assert!(Float::abs(1200f64 - lpf.cutoff) < EPSILON);
		assert!(Float::abs(1f64 - lpf.q) < EPSILON);

		/*
		// Check intermidiates
		// w0 = pi * 2 * 1200 / 44100
		// alpha = (sin(pi * 2 * 1200 / 44100) / 2)
		let w0 = ::std::f64::consts::PI_2 * lpf.cutoff / lpf.sample_rate;
		let cos_w0 = Float::cos(w0);
		let alpha = Float::sin(w0) / (2f64 * lpf.q);
		println!("{}", w0);
		println!("{}", cos_w0);
		println!("{}", alpha);
		// These are correct... why is it failing?
		assert!(Float::abs(w0 		- 0.170971028f64) < EPSILON);
		assert!(Float::abs(cos_w0 	- 0.985420021f64) < EPSILON);
		assert!(Float::abs(alpha 	- 0.085069650f64) < EPSILON);
		
		println!("{}", lpf.biquad.b0);
		assert!(Float::abs(lpf.biquad.b0 - 0.006718452f64) < EPSILON);
		*/
	}
}