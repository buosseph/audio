use std::fs::File;
use std::io::{
  BufWriter, Read, Seek, Write
};
use std::path::Path;

use aiff::Encoder as AiffEncoder;
use buffer::*;
use codecs::Codec;
use error::*;
use traits::{
  AudioDecoder,
  AudioEncoder
};
use wave::Decoder as WaveDecoder;
use wave::Encoder as WaveEncoder;

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
#[inline]
pub fn load<R: Read+Seek>(reader: &mut R,
                          format: AudioFormat)
-> AudioResult<AudioBuffer> {
  match format {
    AudioFormat::Wave => WaveDecoder::new(reader).decode(),
    AudioFormat::Aiff => {
      let mut decoder = try!(::format::aiff::read(reader));
      let data = try!(decoder.decode());
      Ok(AudioBuffer::from_samples(
        decoder.sample_rate,
        decoder.channels,
        data
      ))
    }
  }
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
    AudioFormat::Wave => WaveEncoder::new(&mut BufWriter::new(writer))
                         .encode(audio),
    AudioFormat::Aiff => AiffEncoder::new(&mut BufWriter::new(writer))
                         .encode(audio)
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
  match format {
    AudioFormat::Wave => WaveEncoder::new(&mut BufWriter::new(writer))
                         .encode_as(audio, codec),
    AudioFormat::Aiff => AiffEncoder::new(&mut BufWriter::new(writer))
                         .encode_as(audio, codec)
  }
}
