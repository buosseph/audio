use std::path::Path;
use buffer::*;
use error::*;

/// Loads the audio file into memory from a path. The necessary decoder
/// is determined by the path file extension. An 'AudioError' is 
/// returned if the file type is not supported or if an error occurred
/// in the decoding process.
pub fn load(path: &Path) -> AudioResult<AudioBuffer> {
  if let Some(format) = path.extension() {
    match format {
      //"wav"   =>
      //"aiff"  => 
      _       => Err(AudioError::FormatError(format!("A decoder for {:?} is not available.", format)))
    }
  }
  else {
    Err(AudioError::FormatError("Did not recognize file format".to_string()))
  }
}


/// Writes the audio file to the provided path. The necessary encoder
/// is determined by the path file extension. An `AudioError` is 
/// returned if the file type is not supported or if an error occurred
/// in the encoding process. 
pub fn save(audio: &AudioBuffer, path: &Path) -> AudioResult<bool> {
  if let Some(format) = path.extension() {
    match format {
      //"wav"   => { ...; true}
      //"aiff"  => { ...; true}
      _       => Err(AudioError::FormatError(format!("A decoder for {:?} is not available.", format)))
    }
  }
  else {
    Err(AudioError::FormatError("Did not recognize file format".to_string()))
  }
}

/*
pub trail AudioDecoder {

}
*/
