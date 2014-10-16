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

// pub trait AudioDecoder {
// }

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