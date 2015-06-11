extern crate audio;

#[test]
fn test_load() {
  use std::path::Path;
  use audio::AudioBuffer;
  use audio::error::AudioResult;

  let mut path = Path::new(".gitignore");
  let mut result: AudioResult<AudioBuffer> = audio::load(path);
  assert!(result.is_err());

  path = Path::new("Cargo.toml");
  result = audio::load(path);
  assert!(result.is_err());
}