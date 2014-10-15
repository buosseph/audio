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


pub trait Dynamics {
	fn invert(&mut self) -> bool;
}
impl Dynamics for RawAudio {
	// Create test to write and test checking for phase cancellation
	// Optimization: traverse using merge sort? -> O(log n)
	fn invert(&mut self) -> bool {
		for sample in self.samples.iter_mut() {
			*sample = -*sample;
		}
		true
	}
}