//! WAVE Chunks
use std::fmt;
use std::io::{Read, Seek};
use byteorder::{ByteOrder, ReadBytesExt, LittleEndian};
use traits::Chunk;
use error::*;

/// Enumeration of WAVE chunks
pub enum WaveChunk {
  Format,
  Data,
  //Fact
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

/// The format chunck contains most of the audio realted meta data in `WAV` files
#[derive(Debug, Clone, Copy)]
pub struct FormatChunk {
  pub size: u32,
  pub compression_type: CompressionType,
  pub num_of_channels: u16,
  pub sample_rate: u32,
  pub data_rate: u32,
  pub block_size: u16,
  pub bit_rate: u16,
}

impl Chunk for FormatChunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<FormatChunk> {
    let size :u32 = try!(r.read_u32::<LittleEndian>());
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    for _ in 0..buffer.capacity() { buffer.push(0u8); }
    try!(r.read(&mut buffer));
    let compression_code : u16 = LittleEndian::read_u16(&buffer[0..2]);
    let compression_type : CompressionType
      = match compression_code {
        1 => CompressionType::PCM,
        _ => CompressionType::Unknown,  // Not supporting any other type than PCM
      };
    let num_of_channels : u16 = LittleEndian::read_u16(&buffer[2..4]);
    let sample_rate     : u32 = LittleEndian::read_u32(&buffer[4..8]);
    let data_rate       : u32 = LittleEndian::read_u32(&buffer[8..12]);
    let block_size      : u16 = LittleEndian::read_u16(&buffer[12..14]);
    let bit_rate        : u16 = LittleEndian::read_u16(&buffer[14..16]);

    // Don't care for other bytes if PCM

    Ok(
      FormatChunk {
        size: size,
        compression_type: compression_type,
        num_of_channels: num_of_channels,
        sample_rate: sample_rate,
        data_rate: data_rate,
        block_size: block_size,
        bit_rate: bit_rate,
      }
    )
  }
}

/// The data chunk contains the coded audio data. Multi-channel data are
/// always interleaved in `WAV` files.
#[allow(dead_code)]
pub struct DataChunk {
  pub size: u32,
  pub bytes: Vec<u8>,
}

impl Chunk for DataChunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<DataChunk> {
    let size :u32 = try!(r.read_u32::<LittleEndian>());
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    for _ in 0..buffer.capacity() { buffer.push(0u8); }
    let num_read_bytes = try!(r.read(&mut buffer));
    debug_assert_eq!(size as usize, num_read_bytes);
    Ok(
      DataChunk {
        size: size,
        bytes: buffer,
      }
    )
  }
}

/*
/// The fact chunk contains the number of
/// samples in the file. This chunk is 
/// required when using a non-PCM codec.
struct FactChunk {
  size: u32,
  num_samples_per_channel: u32  
}

impl Chunk for FactChunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<FactChunk> {
    let size :u32 = try!(r.read_u32::<LittleEndian>());
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    for _ in 0..buffer.capacity() { buffer.push(0u8); }
    let num_read_bytes = try!(r.read(&mut buffer));
    let num_samples_per_channel = LittleEndian::read_u32(&buffer);
    debug_assert_eq!(size as usize, num_read_bytes);
    Ok(
      FactChunk {
        size: size,
        num_samples_per_channel: num_samples_per_channel,
      }
    )
  }
}
*/

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
