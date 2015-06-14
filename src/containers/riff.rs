const RIFF: u32 = 0x52494646;
const WAVE: u32 = 0x57415645;
const FMT:  u32 = 0x666D7420;
const DATA: u32 = 0x64617461;

use std::fmt;
use std::io::{Read, Seek};
use buffer::*;
use byteorder::{ReadBytesExt, LittleEndian};
use codecs::{Codec, AudioCodec, LPCM};
use containers::{Container, Chunk};
use error::*;

#[test]
fn test_le_to_be() {
  let x: u32 = 0x44AC0000;
  assert_eq!(44100, x.swap_bytes());
}

/// Enumeration of supported RIFF chunks
enum ChunkType {
  RiffHeader,
  Format,
  Data
}

/// The Resource Interchange File Format (RIFF) is a generic
/// file container format that uses chunks to store data.
/// All bytes are stored in little-endian format.
pub struct RiffContainer {
  compression: CompressionType,
  pub bit_rate: u32,
  pub sample_rate: u32,
  pub channels: u32,
  pub block_size: u32,
  pub order: SampleOrder,
  pub bytes: Vec<u8>
}

#[allow(unused_variables)]
impl Container for RiffContainer {
  fn open<R: Read + Seek>(r: &mut R) -> AudioResult<RiffContainer> {
    let header_chunk_type = try!(identify(r));
    let header            = try!(RiffHeader::read(r));
    let fmt_chunk_type    = try!(identify(r));
    let fmt_chunk         = try!(FormatChunk::read(r));
    let data_chunk_type   = try!(identify(r));
    // Rearrange all samples so that it's in big endian
    let mut data_chunk    = try!(DataChunk::read(r));
    let sample_size = fmt_chunk.bit_rate as usize / 8;
    if sample_size != 1 {
      for sample_bytes in data_chunk.bytes.chunks_mut(sample_size) {
        sample_bytes.reverse();
      }
    }
    let sample_order
      = if fmt_chunk.num_of_channels == 1u16 {
        SampleOrder::MONO
      } else {
        SampleOrder::INTERLEAVED
      };
    Ok(RiffContainer {
      compression:  fmt_chunk.compression_type,
      bit_rate:     fmt_chunk.bit_rate as u32,
      sample_rate:  fmt_chunk.sample_rate,
      channels:     fmt_chunk.num_of_channels as u32,
      block_size:   fmt_chunk.block_size as u32,
      order:        sample_order,
      bytes:        data_chunk.bytes
    })
  }

  fn read_codec(&mut self) -> AudioResult<Vec<Sample>> {
    let codec = match self.compression {
      CompressionType::PCM => Codec::LPCM,
      _ => return Err(AudioError::UnsupportedError("This file uses an unsupported codec".to_string()))
    };
    match codec {
      Codec::LPCM => LPCM::read(&mut self.bytes, &self.bit_rate, &self.channels)
    }
  }

  fn create(codec: Codec, audio: &AudioBuffer) -> AudioResult<Vec<u8>> {
    match codec {
      Codec::LPCM => Ok(vec![]),
    }
  }
}

/// This function reads the four byte identifier for each RIFF chunk
///
/// If an unsupported chunk is found instead, the identifier bytes are lost
/// and makes reading the remainder of the file for chunks impossible without
/// skipping the length of the chunk indicated by the next four bytes available
/// in the reader.
fn identify<R>(r: &mut R) -> AudioResult<ChunkType> where R: Read + Seek {
  match try!(r.read_u32::<LittleEndian>()) {
    RIFF => Ok(ChunkType::RiffHeader),
    FMT  => Ok(ChunkType::Format),
    DATA => Ok(ChunkType::Data),
    _ => Err(AudioError::FormatError("Do not recognize RIFF chunk".to_string()))
  }
}

/// All RIFF containers start with a RIFF chunk, which contains
/// subchunks. The file format and size are specified here.
#[derive(Debug, Clone, Copy)]
struct RiffHeader {
  size: u32,
  format: u32,
}

impl Chunk for RiffHeader {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<RiffHeader> {
    let buffer: &mut[u8] = &mut[0u8; 8];
    try!(r.read(buffer));
    // Converting to little endian
    let file_size   : u32
      = (buffer[3] as u32) << 24
      | (buffer[2] as u32) << 16
      | (buffer[1] as u32) << 8
      |  buffer[0] as u32;
    let form_type   : u32
      = (buffer[7] as u32) << 24
      | (buffer[6] as u32) << 16
      | (buffer[5] as u32) << 8
      |  buffer[4] as u32;
    if form_type != WAVE {
      return Err(AudioError::FormatError("This is not a valid WAV file".to_string()))
    }
    Ok(
      RiffHeader {
        size: file_size,
        format: form_type,
      }
    )
  }
}

/// Enumeration of supported compression codes in the RIFF format chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompressionType {
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
struct FormatChunk {
  size: u32,
  compression_type: CompressionType,
  num_of_channels: u16,
  sample_rate: u32,
  data_rate: u32,
  block_size: u16,
  bit_rate: u16,
}

impl Chunk for FormatChunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<FormatChunk> {
    let size :u32 = try!(r.read_u32::<LittleEndian>());
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    try!(r.read(&mut buffer));
    let compression_code : u16
      = (buffer[1] as u16) << 8
      |  buffer[0] as u16;
    let compression_type: CompressionType
      = match compression_code {
        1 => CompressionType::PCM,
        _ => CompressionType::Unknown,  // Not supporting any other type than PCM
      };
    let num_of_channels : u16
      = (buffer[3] as u16) << 8
      |  buffer[2] as u16;
    let sample_rate     : u32
      = (buffer[7] as u32) << 24
      | (buffer[6] as u32) << 16
      | (buffer[5] as u32) << 8
      |  buffer[4] as u32;
    let data_rate       : u32
      = (buffer[11] as u32) << 24
      | (buffer[10] as u32) << 16
      | (buffer[9] as u32)  << 8
      |  buffer[8] as u32;
    let block_size      : u16
      = (buffer[13] as u16) << 8
      |  buffer[12] as u16;
    let bit_rate        : u16
      = (buffer[15] as u16) << 8
      |  buffer[14] as u16;

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
struct DataChunk {
  size: u32,
  bytes: Vec<u8>,
}

impl Chunk for DataChunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<DataChunk> {
    let size :u32 = try!(r.read_u32::<LittleEndian>());
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    try!(r.read(&mut buffer));
    Ok(
      DataChunk {
        size: size,
        bytes: buffer,
      }
    )
  }
}
