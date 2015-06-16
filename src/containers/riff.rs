const RIFF: u32 = 0x52494646;
const WAVE: u32 = 0x57415645;
const FMT:  u32 = 0x666D7420;
const DATA: u32 = 0x64617461;

use std::fmt;
use std::io::{Read, Seek};
use buffer::*;
use byteorder::{ByteOrder, ReadBytesExt, BigEndian, LittleEndian};
use codecs::{Codec, AudioCodec, LPCM};
use containers::{Container, Chunk};
use error::*;

/// Enumeration of supported RIFF chunks
enum ChunkType {
  RiffHeader,
  Format,
  Data
}

/// The Resource Interchange File Format (RIFF) is a generic
/// file container format that uses chunks to store data.
/// All bytes used for data are stored in little-endian format,
/// but identifier bytes are in ASCII, big-endian.
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

  #[allow(unused_assignments)]
  fn create(codec: Codec, audio: &AudioBuffer) -> AudioResult<Vec<u8>> {
    /*
     *  TODO: Dealing with bit rates not supported by format.
     *
     *  Audio can be manipulated to be of any bit rate, but codecs don't
     *  support that. It's preferred that the fields in AudioBuffer
     *  reflect the samples stored, so values need to be checked when
     *  encoding. For bit rate, just round up to nearest multiple of 8
     *  that's less than the highest supported bit rate and use that
     *  value for the encoding bit rate.
     */
    match audio.order {
      SampleOrder::MONO    => {},
      SampleOrder::INTERLEAVED => {},
      _     => return Err(AudioError::UnsupportedError("Multi-channel audio must be interleaved in RIFF containers".to_string()))
    }
    let num_of_channels : u16   = audio.channels as u16;
    let sample_rate     : u32   = audio.sample_rate as u32;
    let bit_rate        : u16   = 16; //= audio.bit_rate as u16;
    let mut data: Vec<u8> = match codec {
      Codec::LPCM => try!(LPCM::create(audio)),
    };
    // Rearrange all samples to little endian
    let sample_size = bit_rate as usize / 8;
    if sample_size != 1 {
      for sample_bytes in data.chunks_mut(sample_size) {
        sample_bytes.reverse();
      }
    }
    let data_rate       : u32   = (audio.sample_rate * audio.channels * audio.bit_rate / 8) as u32;
    let block_size      : usize = (audio.channels * audio.bit_rate) as usize / 8;
    let fmt_chunk_size  : u32   = 16;
    let num_of_frames   : usize = audio.samples.len() / audio.channels as usize;
    let data_size       : u32   = (num_of_frames * block_size) as u32;
    let total_bytes     : usize = 44 + data_size as usize; // Always write 44 byte header
    let file_size       : u32   =  (total_bytes - 8) as u32;
      // = 4 + (8 + fmt_chunk size) + (8 + (data_chunk size * block_size)) (NOTE: 8 bytes are purposely missing for riff_header and file_size)
      // = 4 + (WAVE chunks) = total_bytes - 8 (exclude first 8 bytes)
    let mut buffer      : Vec<u8>   = Vec::with_capacity(total_bytes);
    buffer = vec![0u8; total_bytes];
    BigEndian::write_u32(&mut buffer[0..4], RIFF);
    LittleEndian::write_u32(&mut buffer[4..8], file_size);
    BigEndian::write_u32(&mut buffer[8..12], WAVE);
    BigEndian::write_u32(&mut buffer[12..16], FMT);
    LittleEndian::write_u32(&mut buffer[16..20], fmt_chunk_size);
    LittleEndian::write_u16(&mut buffer[20..22], 1u16); // Always encode as PCM
    LittleEndian::write_u16(&mut buffer[22..24], num_of_channels);
    LittleEndian::write_u32(&mut buffer[24..28], sample_rate);
    LittleEndian::write_u32(&mut buffer[28..32], data_rate);
    LittleEndian::write_u16(&mut buffer[32..34], block_size as u16);
    LittleEndian::write_u16(&mut buffer[34..36], bit_rate);
    BigEndian::write_u32(&mut buffer[36..40], DATA);
    LittleEndian::write_u32(&mut buffer[40..44], data_size);
    let mut i = 44; // Because we're only writing WAVs with 44 byte headers
    for byte in data.iter() {
      buffer[i] = *byte;
      i += 1;
    }

    Ok(buffer)
  }
}

/// This function reads the four byte identifier for each RIFF chunk
///
/// If an unsupported chunk is found instead, the identifier bytes are lost
/// and makes reading the remainder of the file for chunks impossible without
/// skipping the length of the chunk indicated by the next four bytes available
/// in the reader.
fn identify<R>(r: &mut R) -> AudioResult<ChunkType> where R: Read + Seek {
  match try!(r.read_u32::<BigEndian>()) {
    RIFF => Ok(ChunkType::RiffHeader),
    FMT  => Ok(ChunkType::Format),
    DATA => Ok(ChunkType::Data),
    err @ _ => Err(AudioError::FormatError(format!("Do not recognize RIFF chunk with identifier 0x{:x}", err)))
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
    let file_size: u32 = LittleEndian::read_u32(&buffer[0..4]);
    let form_type: u32 = BigEndian::read_u32(&buffer[4..8]);
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
    buffer = vec![0u8; size as usize];
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
struct DataChunk {
  size: u32,
  bytes: Vec<u8>,
}

impl Chunk for DataChunk {
  fn read<R: Read + Seek>(r: &mut R) -> AudioResult<DataChunk> {
    let size :u32 = try!(r.read_u32::<LittleEndian>());
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    buffer = vec![0u8; size as usize];
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


#[test]
fn test_le_to_be() {
  let x: u32 = 0x44AC0000;
  assert_eq!(44100, x.swap_bytes());
}

#[test]
fn test_write_bytes() {
  let mut buf = vec![0u8; 8];
  LittleEndian::write_u32(&mut buf[0..4], 44100u32);
  assert_eq!(0x0000AC44, LittleEndian::read_u32(&buf[0..4]));
  assert_eq!(44100u32, LittleEndian::read_u32(&buf[0..4]));
}

#[test]
fn test_read_bytes() {
  let buf = vec![0x9E, 0x59, 0xB5, 0x52];
  LittleEndian::read_i16(&buf[0..2]);
  assert_eq!(0x599E, LittleEndian::read_i16(&buf[0..2]));
  assert_eq!(22942i16, LittleEndian::read_i16(&buf[0..2]));
  assert_eq!(0x52B5, LittleEndian::read_i16(&buf[2..4]));
  assert_eq!(21173i16, LittleEndian::read_i16(&buf[2..4]));
}
