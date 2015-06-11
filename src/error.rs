use std::fmt;
use std::io::Error as IoError;
use std::error::Error;

/// An enumeration for reporting audio errors
#[allow(dead_code)]
#[derive(Debug)]
pub enum AudioError {
  /// The audio file does not match the supported format specification
  FormatError(String),
  /// An IoError occurred during an audio process
  IoError(IoError),
  /// The audio file requires an unsupported feature from the decoder
  UnsupportedError(String)
}

impl fmt::Display for AudioError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    use self::AudioError::*;
    match self {
      &FormatError(ref e)       => write!(fmt, "Format error: {}", e),
      &UnsupportedError(ref f)  => write!(fmt, "The decoder does not support the audio format `{}`", f),
      &IoError(ref e)           => e.fmt(fmt)
    }
  }
}

impl Error for AudioError {
  fn description(&self) -> &str {
    use self::AudioError::*;
    match *self {
      FormatError(..)       => &"Format error",
      UnsupportedError(..)  => &"Unsupported error",
      IoError(..)           => &"IO error"
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

/// Result type of an audio encoding or decoding process
pub type AudioResult<T> = Result<T, AudioError>;
