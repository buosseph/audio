use super::Biquad;

// Can't directly access fields, else can't update coefficents on parameter changes
#[allow(dead_code)]
struct Lowpass {
	sample_rate: f64,
	cutoff: f64,
	q: f64,
	biquad: Biquad
}
impl Lowpass {
	// fn new(sample_rate: f64, cutoff: f64, q: f64) -> Self {
	// 	let mut biquad = Biquad::new();
	// 	let mut x = Lowpass {
	// 		sample_rate: sample_rate,
	// 		cutoff: cutoff,
	// 		q: q,
	// 		biquad: biquad
	// 	};
	// 	x.calculate_coefficients();
	// 	x
	// }

	#[allow(dead_code)]
	fn calculate_coefficients(&mut self) {
		// Intermidiates
		let w0 = ::std::f64::consts::PI_2 * self.cutoff / self.sample_rate;
		let cos_w0 = ::std::num::Float::cos(w0);
		let alpha = ::std::num::Float::sin(w0) / (2f64 * self.q);

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
