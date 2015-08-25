use std::path::Path;

extern crate audio;

fn main() {
  match audio::open(&Path::new("examples/audio/amen-break.wav")) {
    Ok(audio) => println!("Read {}ms of audio", audio.duration()),
    Err(e)    => println!("{:?}", e)
  }

  let aiff = audio::open(&Path::new("examples/audio/amen-break.aiff"));
  if let Some(audio) = aiff.ok() {
    println!("Read {}ms of audio", audio.duration());
  }
}