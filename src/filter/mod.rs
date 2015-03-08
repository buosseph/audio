pub mod biquad;

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
	fn clear(&mut self);
}