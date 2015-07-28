//! WAVE Chunks
use std::fmt;
use byteorder::{ByteOrder, ReadBytesExt, LittleEndian};
use traits::Chunk;
use error::*;

/// Format tag for the wave extensible format. Unlike chunk identifiers,
/// this is read as little endian data since it is within the chunk.
const WAVE_FORMAT_EXTENSIBLE: u16 = 0xFFFE;

/// Supported WAVE chunks
pub enum WaveChunk {
  Format,
  Data
}

/// Supported compression codes in the WAVE format chunk
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
/// This chunk provides most of the information required to decode the sampled
/// data.
///
/// The format chunk can be of three different sizes: 16, 18, and 40 bytes. If
/// the data is encoded as LPCM, then the chunk will be 16 bytes long. If the
/// data is encoded using any other codec, then the chunk will be 18 bytes long.
/// Non-LPCM data also requires the presence of a fact chunk within the file.
///
/// Wave files also have an extensible format which provided additional data
/// to eliminate ambiguities in the standard format. The `WAVE_EXTENSIBLE_FORMAT`
/// requires the chunk to be 40 bytes long, and moves the compression type
/// information later in the chunk.
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
    let mut format_tag: u16 = LittleEndian::read_u16(&buffer[0..2]);
    if format_tag == WAVE_FORMAT_EXTENSIBLE {
      format_tag = LittleEndian::read_u16(&buffer[24..26])
    }
    let compression_type : CompressionType = 
      match format_tag {
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
