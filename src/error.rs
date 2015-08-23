use std::fmt;
use std::io::Error as IoError;
use std::error::Error;
use byteorder::Error as ByteError;

/// Result type of an audio encoding or decoding process
pub type AudioResult<T> = Result<T, AudioError>;

/// An enumeration for reporting audio errors
#[allow(dead_code)]
#[derive(Debug)]
pub enum AudioError {
  /// An audio file does not match the supported format specification
  Format(String),
  /// Any underlying IO error occurred during an audio process
  Io(IoError),
  /// An audio process requires use of an unsupported feature
  Unsupported(String),
  /// The end of the audio file was reached
  AudioEnd
}

impl fmt::Display for AudioError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    fmt.write_str(self.description())
  }
}

impl Error for AudioError {
  fn description(&self) -> &str {
    use self::AudioError::*;
    match *self {
      Format(ref s)      => s,
      Unsupported(ref s) => s,
      Io(ref e)          => e.description(),
      AudioEnd           => "Unexpected end of audio"
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      AudioError::Io(ref error) => Some(error),
      _ => None
    }
  }
}

impl From<IoError> for AudioError {
  fn from(err: IoError) -> AudioError {
    AudioError::Io(err)
  }
}

impl From<ByteError> for AudioError {
  fn from(err: ByteError) -> AudioError {
    match err {
      ByteError::UnexpectedEOF  => AudioError::AudioEnd,
      ByteError::Io(err)        => AudioError::Io(err),
    }
  }
}

#[cfg(test)]
mod conversions {
  use super::*;
  use std::error::Error;
  use std::io;

  #[test]
  fn from_io() {
    let original = io::Error::new(io::ErrorKind::Other, "other");
    let original_description = original.description().to_owned();
    let error = AudioError::from(original);
    assert!(error.cause().is_some());
    assert_eq!(error.cause().unwrap().description(), original_description);
    assert_eq!(error.description(), original_description); 
  }

  #[test]
  fn from_byteorder_eof() {
    use byteorder::Error::UnexpectedEOF;

    let original = UnexpectedEOF;
    let error = AudioError::from(original);
    assert!(error.cause().is_none());
    assert_eq!(AudioError::AudioEnd.description(), error.description());
  }

  #[test]
  fn from_byteorder_io() {
    use byteorder::Error::Io as ByteIoError;

    let original = io::Error::new(io::ErrorKind::Other, "other");
    let byte_error = ByteIoError(original);
    let byte_error_description = byte_error.description().to_owned();
    let error = AudioError::from(byte_error);
    assert!(error.cause().is_some());
    assert_eq!(error.cause().unwrap().description(), byte_error_description);
    assert_eq!(error.description(), byte_error_description);
  }
}
