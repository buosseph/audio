use std::path::Path;

extern crate audio;
use audio::Codec;

fn main() {
  let mut aiff = audio::open(&Path::new("examples/audio/amen-break.aiff")).unwrap();
  for sample in aiff.samples.iter_mut() {
    // Attenuate by -3 db
    *sample = *sample * 0.707946f32;
  }
  // Default wave writes as i16 and don't print any error message
  if let Ok(_) = audio::save(&Path::new("examples/audio/my-break.wav"), &aiff) {
    println!("Saved");
  }

  let mut wave = audio::open(&Path::new("examples/audio/amen-break.wav")).unwrap();
  for sample in wave.samples.iter_mut() {
    // Attenuate by -3 db
    *sample = *sample * 0.707946f32;
  }
  // Write as f64 and print and error message on failure
  match audio::save_as(&Path::new("examples/audio/my-break.aiff"), &wave, Codec::LPCM_F64_BE) {
    Ok(_) => println!("Saved"),
    Err(e) => println!("{:?}", e)
  }
}