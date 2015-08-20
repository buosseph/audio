use std::path::Path;

extern crate audio;

fn main() {
  match audio::open(&Path::new("examples/audio/amen-break.wav")) {
    Ok(audio) => println!("Read {:?}-bit, {:?}hz audio", audio.bit_depth, audio.sample_rate),
    Err(e)    => println!("{:?}", e)
  }

  let aiff = audio::open(&Path::new("examples/audio/amen-break.aiff"));
  if let Some(audio) = aiff.ok() {
    println!("Read {:?}-bit, {:?}hz audio", audio.bit_depth, audio.sample_rate);
  }
}