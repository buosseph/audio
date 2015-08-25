use std::path::Path;

extern crate audio;
use audio::AudioBuffer;
use audio::Sample;

fn main() {
  let sample_rate = 44100;
  let seconds = 2;
  let mut sine: Vec<Sample> = Vec::new();
  for time in (0..sample_rate * seconds).map(|t| t as f32 / sample_rate as f32) {
    // sample = sin(2 * pi * t * f/Fs)
    let sample = (2f32 * std::f32::consts::PI * time * 440f32).sin();
    sine.push(sample);
  }
  let audio = AudioBuffer::from_samples(sample_rate, 1, sine);
  match audio::save(&Path::new("examples/audio/my-sine.wav"), &audio) {
    Ok(_) => println!("File saved"),
    Err(e) => println!("{:?}", e)
  }
}