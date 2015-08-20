use std::path::Path;

extern crate audio;
use audio::AudioBuffer;
use audio::Sample;
use audio::SampleOrder;

fn main() {
  let sample_rate = 44100;
  let mut sine: Vec<Sample> = Vec::new();
  for time in (0..sample_rate * 4).map(|t| t as f32 / sample_rate as f32) {
    // sample = sin(2 * pi * t * f/Fs)
    let sample = (2f32 * std::f32::consts::PI * time * 440f32).sin();
    sine.push(sample);
  }
  let audio = AudioBuffer {
    bit_rate: 16,
    sample_rate: sample_rate,
    channels: 1,
    order: SampleOrder::MONO,
    samples: sine
  };
  match audio::save(&Path::new("examples/audio/my-sine.wav"), &audio) {
    Ok(_) => println!("File saved"),
    Err(e) => println!("{:?}", e)
  }
}