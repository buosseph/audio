use std::path::Path;
use std::fs::File;
use buffer::*;
use error::*;
use std::io::{Read, Seek, BufReader};
use wave::Decoder as WaveDecoder;

/// An enumeration of all supported audio formats
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioFormat {
  /// Audio in WAVE format
  WAV,
  /// Audio in AIFF format
  AIFF
}

/// Opens and loads the audio file into memory from a path. The necessary decoder
/// is determined by the path file extension. An `AudioError` is 
/// returned if the file type is not supported or if an error occurred
/// in the decoding process.
pub fn open(path: &Path) -> AudioResult<AudioBuffer> {
  if let Some(ext) = path.extension() {
    if let Some(file_format) = ext.to_str() {
      let format = match file_format {
        "wav"|"wave"  => AudioFormat::WAV,
        "aif"|"aiff"  => AudioFormat::AIFF,
        _ => return Err(AudioError::FormatError(format!("Did not recognize `.{:?}` as an audio file format", ext)))
      };
      let file = try!(File::open(path));
      load(file, AudioFormat::WAV)
    }
    else {
      Err(AudioError::FormatError("Did not recognize file format".to_string()))
    }
  }
  else {
    Err(AudioError::FormatError("Did not recognize file format".to_string()))
  }
}

/// Loads the audio from a reader into memory. The necessary decoder
/// is determined by the provided `AudioFormat`. An `AudioError` is 
/// returned if the format is not supported or if an error occurred
/// in the decoding process.
///
/// A reader, in this case, is any struct that implements the `Read` and
/// `Seek` traits. One example would be a `File`.
pub fn load<R: Read+Seek>(reader: R, format: AudioFormat) -> AudioResult<AudioBuffer> {
  match format {
    AudioFormat::WAV  => WaveDecoder::new(BufReader::new(reader)).decode(),
    //AudioFormat::AIFF => unimplemented!(),
    _ => Err(AudioError::FormatError(format!("A decoder for {:?} is not available", format)))
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
      _       => Err(AudioError::FormatError(format!("A decoder for {:?} is not available", format)))
    }
  }
  else {
    Err(AudioError::FormatError("Did not recognize file format".to_string()))
  }
}

/// Trait which all decoders must implement in order to return an `AudioBuffer` and metadata
pub trait AudioDecoder {
  fn bit_rate(&self) -> AudioResult<u8>;
  fn sample_rate(&self) -> AudioResult<u32>;
  fn channels(&self) -> AudioResult<u32>;
  fn sample_order(&self) -> AudioResult<SampleOrder>;
  fn open_container(&mut self) -> AudioResult<Vec<u8>>;
  //fn read_codec(codec: Codec, data: Vec<u8>) -> AudioResult<Vec<Sample>>;

  fn decode(self) -> AudioResult<AudioBuffer>;
}
