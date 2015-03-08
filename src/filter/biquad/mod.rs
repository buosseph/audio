use super::Filter;

mod lowpass;

/// A single channel, second-order filter
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
	fn clear(&mut self) {
		self.x_z1 = 0f64;
		self.x_z2 = 0f64;
		self.y_z1 = 0f64;
		self.y_z2 = 0f64;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tick() {
		let input_sample = 0.55f64;

		let mut biquad = Biquad::new();
		assert_eq!(0.55f64, biquad.tick(input_sample));

		biquad.set_coefficients(0.5f64, 0.4f64, 0.3f64, 0.2f64, 0.1f64);
		assert_eq!(0.275f64, biquad.tick(input_sample));
		assert_eq!(0.44f64, biquad.tick(input_sample));
		assert_eq!(0.5445f64, biquad.tick(input_sample));
	}
}