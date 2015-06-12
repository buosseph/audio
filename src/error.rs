use std::fmt;
use std::io::Error as IoError;
use std::error::Error;
use byteorder::Error as ByteError;

/// An enumeration for reporting audio errors
#[allow(dead_code)]
#[derive(Debug)]
pub enum AudioError {
  /// The audio file does not match the supported format specification
  FormatError(String),
  /// An IoError occurred during an audio process
  IoError(IoError),
  /// The audio file requires an unsupported feature from the decoder
  UnsupportedError(String),
  /// The end of the audio file has been reached
  AudioEnd
}

impl fmt::Display for AudioError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    use self::AudioError::*;
    match self {
      &FormatError(ref e)       => write!(fmt, "Format error: {}", e),
      &UnsupportedError(ref f)  => write!(fmt, "The decoder does not support the audio format `{}`", f),
      &IoError(ref e)           => e.fmt(fmt),
      &AudioEnd                 => write!(fmt, "The end of the audio file has been reached")
    }
  }
}

impl Error for AudioError {
  fn description(&self) -> &str {
    use self::AudioError::*;
    match *self {
      FormatError(..)       => &"Format error",
      UnsupportedError(..)  => &"Unsupported error",
      IoError(..)           => &"IO error",
      AudioEnd              => &"Audio end"
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      AudioError::IoError(ref e) => Some(e),
      _ => None
    }
  }
}

impl From<IoError> for AudioError {
  fn from(err: IoError) -> AudioError {
    AudioError::IoError(err)
  }
}

impl From<ByteError> for AudioError {
  fn from(err: ByteError) -> AudioError {
    match err {
      ByteError::UnexpectedEOF  => AudioError::AudioEnd,
      ByteError::Io(err)        => AudioError::IoError(err),
    }
  }
}

/// Result type of an audio encoding or decoding process
pub type AudioResult<T> = Result<T, AudioError>;
