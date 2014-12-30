use audio::RawAudio;
use audio::Utilities;

impl Utilities for RawAudio {

	// Issue: result is half length of desired
	fn stereo_to_mono(&mut self) -> bool {
		match self.channels {
			2 => {
				let mut mono_buffer: Vec<f64> = Vec::with_capacity(self.samples.len() / 2);
				let mut first_channel_value: f64 = 0.;
				for i in range(0, self.samples.len()) {
					// Every second value
					if i % 2 == 1 {
						let second_channel_value: f64 = *self.samples.get_mut(i);
						let mono_value: f64 = (first_channel_value + second_channel_value) / 2.;
						mono_buffer.push(mono_value);
					}
					first_channel_value = *self.samples.get_mut(i);
				}
				self.samples = mono_buffer;

				true
			},
			_ => false
		}
	}

	// Create test to write and test checking for phase cancellation
	fn invert(&mut self) -> bool {
		for sample in self.samples.iter_mut() {
			*sample = -*sample;
		}
		true
	}

	fn reverse_channels(&mut self) -> bool {
		for i in range(0, self.samples.len()) {
			if i % self.channels == self.channels - 1 {

				for j in range(0, self.channels / 2) {
					let left_index = i - ( (i - j) % self.channels );
					let right_index = i - j;

					if left_index != right_index {
						let temp = *self.samples.get_mut(left_index);
						*self.samples.get_mut(left_index) = *self.samples.get_mut(right_index);
						*self.samples.get_mut(right_index) = temp;
					}
				}

			}
		}

		true
	}

	fn reverse(&mut self) -> bool {
		self.samples.reverse();
		self.reverse_channels()
	}

	// Also reverses channels
	fn full_reverse(&mut self) -> bool {
		self.samples.reverse();
		true
	}
}