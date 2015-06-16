use std::path::Path;
use std::fs::File;
use buffer::*;
use error::*;
use std::io::{Read, Seek, Write, BufReader};
use wave::Decoder as WaveDecoder;
use wave::Encoder as WaveEncoder;
use aiff::Decoder as AiffDecoder;
use aiff::Encoder as AiffEncoder;

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
      // TODO: Test path, see if it's valid and return a useful error message
      let file = try!(File::open(path));
      load(file, format)
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
    AudioFormat::WAV  => WaveDecoder::new(reader).decode(),
    AudioFormat::AIFF => AiffDecoder::new(reader).decode(),
  }
}

/// Writes the audio file to the provided path. The necessary encoder
/// is determined by the path file extension. An `AudioError` is 
/// returned if the file type is not supported or if an error occurred
/// in the encoding process. 
#[allow(unused_variables)]
pub fn save(path: &Path, audio: &AudioBuffer) -> AudioResult<()> {
  if let Some(ext) = path.extension() {
    if let Some(file_format) = ext.to_str() {
      let format = match file_format {
        "wav"|"wave"  => AudioFormat::WAV,
        "aif"|"aiff"  => AudioFormat::AIFF,
        _ => return Err(AudioError::FormatError(format!("Did not recognize `.{:?}` as an audio file format", ext)))
      };
      let mut file = try!(File::create(path));
      write(&mut file, audio, format)
    }
    else {
      Err(AudioError::FormatError("Did not recognize file format".to_string()))
    }
  }
  else {
    Err(AudioError::FormatError("Did not recognize file format".to_string()))
  }
}

pub fn write<W: Write>(writer: &mut W, audio: &AudioBuffer, format: AudioFormat) -> AudioResult<()> {
  match format {
    AudioFormat::WAV  => WaveEncoder::new(writer).encode(audio),
    AudioFormat::AIFF => AiffEncoder::new(writer).encode(audio),
  }
}

/// Trait which all decoders must implement
pub trait AudioDecoder {
  fn decode(self) -> AudioResult<AudioBuffer>;
  //fn open_container(&mut self) -> AudioResult<Vec<u8>>;
  //fn read_codec(codec: Codec, data: Vec<u8>) -> AudioResult<Vec<Sample>>;
}

/// Trait which all encoders must implement
pub trait AudioEncoder {
  fn encode(&mut self, audio: &AudioBuffer) -> AudioResult<()>;
}
