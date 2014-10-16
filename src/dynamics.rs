use audio::RawAudio;
use audio::Dynamics;

impl Dynamics for RawAudio {
	//	Can't use decibel values until std::num::Float is further implemented
	//	gain = 10^(decibels / 20), decibels = 20 * log10(gain)
	//	Amplify by 7.94328 (18db)
	//	Allows clipping
	fn amplify(&mut self, gain: f64) -> bool {
		for sample in self.samples.iter_mut() {
			*sample = *sample * gain;
		}
		true
	}
}