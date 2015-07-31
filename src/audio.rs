use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};
use std::path::Path;
use aiff::Decoder as AiffDecoder;
use aiff::Encoder as AiffEncoder;
use buffer::*;
use codecs::Codec;
use error::*;
use traits::{AudioDecoder, AudioEncoder};
use wave::Decoder as WaveDecoder;
use wave::Encoder as WaveEncoder;

/// All supported audio formats.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioFormat {
  /// Audio in WAVE format.
  WAVE,
  /// Audio in AIFF and AIFF-C format.
  AIFF
}

/// Opens and loads the audio file into memory from a path. The necessary decoder
/// is determined by the path file extension. An `AudioError` is returned if the
/// file type is not supported or if an error occurred in the decoding process.
pub fn open(path: &Path) -> AudioResult<AudioBuffer> {
  if let Some(ext) = path.extension() {
    if let Some(file_format) = ext.to_str() {
      let format = match file_format {
        "wav"|"wave"        => AudioFormat::WAVE,
        "aif"|"aiff"|"aifc" => AudioFormat::AIFF,
        _                   => return
          Err(AudioError::FormatError(
            format!("Did not recognize `.{:?}` as an audio file format", ext)
          ))
      };
      // TODO: Test path, see if it's valid and return a useful error message
      let mut file = try!(File::open(path));
      load(&mut file, format)
    }
    else {
      Err(AudioError::FormatError(
        "Did not recognize file format".to_string()
      ))
    }
  }
  else {
    Err(AudioError::FormatError(
      "Did not recognize file format".to_string()
    ))
  }
}

/// Loads the audio from a reader into memory. The necessary decoder
/// is determined by the provided `AudioFormat`. An `AudioError` is 
/// returned if the format is not supported or if an error occurred
/// in the decoding process.
#[inline]
pub fn load<R: Read+Seek>(reader: &mut R, format: AudioFormat) -> AudioResult<AudioBuffer> {
  match format {
    AudioFormat::WAVE => WaveDecoder::new(reader).decode(),
    AudioFormat::AIFF => AiffDecoder::new(reader).decode(),
  }
}

/// Writes the audio file to the provided path. The necessary encoder
/// is determined by the path file extension. An `AudioError` is 
/// returned if the file type is not supported or if an error occurred
/// in the encoding process. 
pub fn save(path: &Path, audio: &AudioBuffer) -> AudioResult<()> {
  if let Some(ext) = path.extension() {
    if let Some(file_format) = ext.to_str() {
      let format = match file_format {
        "wav"|"wave"        => AudioFormat::WAVE,
        "aif"|"aiff"|"aifc" => AudioFormat::AIFF,
        _                   => return
          Err(AudioError::FormatError(
            format!("Did not recognize `.{:?}` as an audio file format", ext)
          ))
      };
      let mut file = try!(File::create(path));
      write(&mut file, audio, format)
    }
    else {
      Err(AudioError::FormatError(
        "Did not recognize file format".to_string()
      ))
    }
  }
  else {
    Err(AudioError::FormatError(
      "Did not recognize file format".to_string()
    ))
  }
}

pub fn save_as(path: &Path, audio: &AudioBuffer, codec: Codec) -> AudioResult<()> {
  if let Some(ext) = path.extension() {
    if let Some(file_format) = ext.to_str() {
      let format = match file_format {
        "wav"|"wave"        => AudioFormat::WAVE,
        "aif"|"aiff"|"aifc" => AudioFormat::AIFF,
        _                   => return
          Err(AudioError::FormatError(
            format!("Did not recognize `.{:?}` as an audio file format", ext)
          ))
      };
      let mut file = try!(File::create(path));
      write_as(&mut file, audio, format, codec)
    }
    else {
      Err(AudioError::FormatError(
        "Did not recognize file format".to_string()
      ))
    }
  }
  else {
    Err(AudioError::FormatError(
      "Did not recognize file format".to_string()
    ))
  }
}

/// Buffers and writes audio to the provided writer. The necessary decoder
/// is determined by the provided `AudioFormat`. An `AudioError` is returned
/// if the format is not supported or if an error occurred in the encoding
/// process.
#[inline]
pub fn write<W: Write>(writer: &mut W, 
                       audio: &AudioBuffer, 
                       format: AudioFormat) -> AudioResult<()> {
  match format {
    AudioFormat::WAVE => WaveEncoder::new(&mut BufWriter::new(writer))
                         .encode(audio),
    AudioFormat::AIFF => AiffEncoder::new(&mut BufWriter::new(writer))
                         .encode(audio)
  }
}

#[inline]
pub fn write_as<W: Write>(writer: &mut W,
                          audio: &AudioBuffer,
                          format: AudioFormat,
                          codec: Codec) -> AudioResult<()> {
  match format {
    AudioFormat::WAVE => WaveEncoder::new(&mut BufWriter::new(writer))
                         .encode_as(audio, codec),
    AudioFormat::AIFF => AiffEncoder::new(&mut BufWriter::new(writer))
                         .encode_as(audio, codec)
  }
}
