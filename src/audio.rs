use std::fs::File;
use std::io::{
  Read,
  Seek,
  Write
};
use std::path::Path;

use buffer::*;
use codecs::Codec;
use encoder::AudioEncoder;
use error::*;

/// All supported audio formats.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioFormat {
  /// Waveform Audio File Format
  Wave,
  /// Audio Interchange File Format
  Aiff
}

/// Opens and loads the audio file into memory from a `Path`.
///
/// The necessary decoder is determined by the `Path` file extension. An
/// `AudioError` is returned if the file type is not supported or if an error
/// occurred in the decoding process.
///
/// Examples
///
/// ```
/// use std::path::Path;
///
/// let path = Path::new("tests/wav/Warrior Concerto - no meta.wav");
/// assert_eq!(audio::open(path).is_ok(), true);
///
/// let path = Path::new("tests/aiff/Warrior Concerto - no meta.aiff");
/// assert_eq!(audio::open(path).is_ok(), true);
/// ```
pub fn open(path: &Path) -> AudioResult<AudioBuffer> {
  let ext = path.extension().and_then(|s| s.to_str());
  if let Some(file_format) = ext {
    let format = match file_format {
      "wav"|"wave"        => AudioFormat::Wave,
      "aif"|"aiff"|"aifc" => AudioFormat::Aiff,
      f_ext @ _           => return
        Err(AudioError::Format(
          format!("Did not recognize audio file format .{}", f_ext)
        ))
    };
    let mut file = try!(File::open(path));
    load(&mut file, format)
  }
  else {
    Err(AudioError::Format(
      format!("Did not recognize file {}", path.display())
    ))
  }
}

/// Loads the audio from a reader into memory.
///
/// The necessary decoder is determined by the provided `AudioFormat`. An
/// `AudioError` is returned if the format is not supported or if an error
/// occurred in the decoding process.
///
/// Examples
///
/// ```
/// use std::path::Path;
/// use std::fs::File;
///
/// use audio::AudioFormat;
///
/// let path = Path::new("tests/wav/Warrior Concerto - no meta.wav");
/// let mut file = File::open(path).ok().unwrap();
/// assert_eq!(audio::load(&mut file, AudioFormat::Wave).is_ok(), true);
///
/// let path = Path::new("tests/aiff/Warrior Concerto - no meta.aiff");
/// let mut file = File::open(path).ok().unwrap();
/// assert_eq!(audio::load(&mut file, AudioFormat::Aiff).is_ok(), true);
/// ```
#[inline]
pub fn load<R: Read+Seek>(reader: &mut R,
                          format: AudioFormat)
-> AudioResult<AudioBuffer> {
  let decoder =
    match format {
      AudioFormat::Wave => try!(::format::wave::read(reader)),
      AudioFormat::Aiff => try!(::format::aiff::read(reader))
    };

  Ok(AudioBuffer::from_samples(
    decoder.sample_rate,
    decoder.channels,
    decoder.collect()
  ))
}

/// Saves an `AudioBuffer` to a `Path`.
///
/// The necessary encoder is determined by the `Path` file extension and uses
/// the default codec of the `AudioFormat`. An `AudioError` is returned if the
/// file type is not supported or if an error occurred in the encoding process.
pub fn save(path: &Path,
            audio: &AudioBuffer)
-> AudioResult<()> {
  let ext = path.extension().and_then(|s| s.to_str());
  if let Some(file_format) = ext {
    let format = match file_format {
      "wav"|"wave"        => AudioFormat::Wave,
      "aif"|"aiff"|"aifc" => AudioFormat::Aiff,
      f_ext @ _           => return
        Err(AudioError::Format(
          format!("Did not recognize audio file format .{}", f_ext)
        ))
    };
    let mut file = try!(File::create(path));
    write(&mut file, audio, format)
  }
  else {
    Err(AudioError::Format(
      format!("Did not recognize file {}", path.display())
    ))
  }
}

/// Saves an `AudioBuffer` to a `Path` using a specified `Codec`.
///
/// The necessary encoder is determined by the `Path` file extension and uses
/// the given `Codec`. An `AudioError` is returned if the file type is not
/// supported, the `Codec` is not supported by the `AudioFormat`, or if an
/// error occurred in the encoding process.
pub fn save_as(path: &Path,
               audio: &AudioBuffer, codec: Codec)
-> AudioResult<()> {
  let ext = path.extension().and_then(|s| s.to_str());
  if let Some(file_format) = ext {
    let format = match file_format {
      "wav"|"wave"        => AudioFormat::Wave,
      "aif"|"aiff"|"aifc" => AudioFormat::Aiff,
      f_ext @ _           => return
        Err(AudioError::Format(
          format!("Did not recognize audio file format .{}", f_ext)
        ))
    };
    let mut file = try!(File::create(path));
    write_as(&mut file, audio, format, codec)
  }
  else {
    Err(AudioError::Format(
      format!("Did not recognize file {}", path.display())
    ))
  }
}

/// Buffers and writes an `AudioBuffer` to a writer using a specified
/// `AudioFormat`.
///
/// The necessary encoder is determined by the given `AudioFormat` and uses
/// the default codec of the `AudioFormat`. An `AudioError` is returned if an
/// error occurred in the encoding process.
#[inline]
pub fn write<W: Write>(writer: &mut W,
                       audio: &AudioBuffer,
                       format: AudioFormat)
-> AudioResult<()> {
  match format {
    AudioFormat::Wave => {
      let mut encoder = AudioEncoder::from_buffer(audio, Codec::LPCM_I16_LE);
      ::format::wave::write(writer, &mut encoder)
    },
    AudioFormat::Aiff => {
      let mut encoder = AudioEncoder::from_buffer(audio, Codec::LPCM_I16_BE);
      ::format::aiff::write(writer, &mut encoder)
    }
  }
}

/// Buffers and writes an `AudioBuffer` to a writer using a specified
/// `AudioFormat` and `Codec`.
///
/// The necessary encoder is determined by the given `AudioFormat` and uses
/// the given `Codec`. An `AudioError` is returned if the `Codec` is not
/// supported by the `AudioFormat` or if an error occurred in the encoding
/// process.
#[inline]
pub fn write_as<W: Write>(writer: &mut W,
                          audio: &AudioBuffer,
                          format: AudioFormat,
                          codec: Codec)
-> AudioResult<()> {
  let mut encoder = AudioEncoder::from_buffer(audio, codec);
  match format {
    AudioFormat::Wave => {
      ::format::wave::write(writer, &mut encoder)
    },
    AudioFormat::Aiff => {
      ::format::aiff::write(writer, &mut encoder)
    }
  }
}
