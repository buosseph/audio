//! WAVE Chunks
use std::fmt;
use byteorder::{ByteOrder, ReadBytesExt, LittleEndian};
use traits::Chunk;
use error::*;

/// Enumeration of supported WAVE chunks
pub enum WaveChunk {
  Format,
  Data
}

/// Enumeration of supported compression codes in the WAVE format chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
  Unknown = 0,
  PCM     = 1
}

impl fmt::Display for CompressionType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(fmt, "{}", self)
  }
}

/// The WAVE Format Chunk.
///
/// This chunk provides most of the information
/// required to decode the sampled data.
#[derive(Debug, Clone, Copy)]
pub struct FormatChunk {
  pub compression_type: CompressionType,
  pub num_channels:     u16,
  pub sample_rate:      u32,
  pub data_rate:        u32,
  pub block_size:       u16,
  pub bit_rate:         u16,
}

impl Chunk for FormatChunk {
  #[inline]
  fn read(buffer: &[u8]) -> AudioResult<FormatChunk> {
    let compression_type : CompressionType = 
      match LittleEndian::read_u16(&buffer[0..2]) {
        1 => CompressionType::PCM,
        _ => CompressionType::Unknown,
      };
    Ok(
      FormatChunk {
        compression_type: compression_type,
        num_channels:     LittleEndian::read_u16(&buffer[2..4]),
        sample_rate:      LittleEndian::read_u32(&buffer[4..8]),
        data_rate:        LittleEndian::read_u32(&buffer[8..12]),
        block_size:       LittleEndian::read_u16(&buffer[12..14]),
        bit_rate:         LittleEndian::read_u16(&buffer[14..16]),
      }
    )
  }
}

#[cfg(test)]
mod tests {
  use byteorder::{ByteOrder, LittleEndian};

  #[test]
  fn write_bytes() {
    let mut buf = vec![0u8; 8];
    LittleEndian::write_u32(&mut buf[0..4], 44100u32);
    assert_eq!(0x0000AC44, LittleEndian::read_u32(&buf[0..4]));
    assert_eq!(44100u32, LittleEndian::read_u32(&buf[0..4]));
  }

  #[test]
  fn read_bytes() {
    let buf = vec![0x9E, 0x59, 0xB5, 0x52];
    LittleEndian::read_i16(&buf[0..2]);
    assert_eq!(0x599E, LittleEndian::read_i16(&buf[0..2]));
    assert_eq!(22942i16, LittleEndian::read_i16(&buf[0..2]));
    assert_eq!(0x52B5, LittleEndian::read_i16(&buf[2..4]));
    assert_eq!(21173i16, LittleEndian::read_i16(&buf[2..4]));
  }
}
