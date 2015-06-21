use std::io::{Read, Seek, SeekFrom};
use buffer::*;
use byteorder::{ByteOrder, ReadBytesExt, BigEndian};
use codecs::{Codec, AudioCodec, LPCM};
use traits::{Container, Chunk};
use aiff::chunks::*;
use error::*;

const FORM: i32 = 0x464F524D;
const AIFF: i32 = 0x41494646;
const COMM: i32 = 0x434F4D4D;
const SSND: i32 = 0x53534E44;

pub struct AiffContainer {
  compression: CompressionType,
  pub bit_rate: u32,
  pub sample_rate: u32,
  pub channels: u32,
  pub num_frames: u32,
  pub order: SampleOrder,
  pub bytes: Vec<u8>
}

impl Container for AiffContainer {
  fn open<R: Read + Seek>(r: &mut R) -> AudioResult<AiffContainer> {
    let header: &mut[u8] = &mut[0u8; 12];
    try!(r.read(header));
    if BigEndian::read_i32(&header[0..4])  != FORM
    || BigEndian::read_i32(&header[8..12]) != AIFF {
      return Err(AudioError::FormatError("Not valid AIFF".to_string()));
    }
    let file_size = BigEndian::read_i32(&header[4..8]) as usize;
    let mut pos: i64 = 12i64;
    let mut compression: CompressionType = CompressionType::PCM;
    let mut bit_rate    : u32 = 0u32;
    let mut sample_rate : u32 = 0u32;
    let mut num_channels: u32 = 0u32;
    let mut num_frames  : u32 = 0u32;
    let mut bytes: Vec<u8> = Vec::new();
    let mut comm_chunk_read = false;
    let mut ssnd_chunk_read = false;
    while pos < file_size as i64 {
      pos += 4i64;
      match identify(r).ok() {
        Some(AiffChunk::Common) => {
          let chunk = try!(CommonChunk::read(r));
          compression = CompressionType::PCM; // only option in AIFF, not AIFC
          bit_rate = chunk.bit_rate as u32;
          sample_rate = chunk.sample_rate as u32;
          num_channels = chunk.num_channels as u32;
          num_frames = chunk.num_sample_frames;
          comm_chunk_read = true;
          pos += chunk.size as i64;
        },
        Some(AiffChunk::SoundData) => {
          if !comm_chunk_read {
            return Err(AudioError::FormatError(
              "File is not valid AIFF \
              (Common chunk does not occur before SoundData chunk)".to_string()
            ))
          }
          let chunk = try!(SoundDataChunk::read(r));
          bytes = chunk.data.to_vec();
          ssnd_chunk_read = true;
          pos += chunk.size as i64;
        },
        None => {
          let size = try!(r.read_i32::<BigEndian>());
          pos += size as i64;
          let new_pos = r.seek(SeekFrom::Current(pos)).ok().expect("Error while seeking in reader");
          if new_pos > file_size as u64 {
            return Err(AudioError::FormatError(
              "Some chunk trying to read past end of file".to_string()
            ));
          }
        }
      }
    }
    if !comm_chunk_read {
      return Err(AudioError::FormatError(
        "File is not valid AIFF \
        (Missing required Common chunk)".to_string()
      ))
    }
    else if !ssnd_chunk_read {
      return Err(AudioError::FormatError(
        "File is not valid AIFF \
        (Missing required SoundData chunk)".to_string()
      ))
    }
    let sample_order =
      if num_channels == 1u32 {
        SampleOrder::MONO
      } else {
        SampleOrder::INTERLEAVED
      };
    Ok(AiffContainer {
      compression:  compression,
      bit_rate:     bit_rate,
      sample_rate:  sample_rate,
      channels:     num_channels,
      num_frames:   num_frames,
      order:        sample_order,
      bytes:        bytes
    })
  }

  fn read_codec(&mut self) -> AudioResult<Vec<Sample>> {
    let codec = match self.compression {
      CompressionType::PCM => Codec::LPCM,
      _ => return Err(AudioError::UnsupportedError(
        "This file uses an unsupported codec".to_string()
      ))
    };
    match codec {
      Codec::LPCM => LPCM::read(&mut self.bytes, &self.bit_rate, &self.channels)
    }
  }

  fn create(codec: Codec, audio: &AudioBuffer) -> AudioResult<Vec<u8>> {
    match audio.order {
      SampleOrder::MONO    => {},
      SampleOrder::INTERLEAVED => {},
      _     => return Err(AudioError::UnsupportedError(
        "Multi-channel audio must be interleaved in IFF containers".to_string()
      ))
    }
    let num_channels : i16   = audio.channels     as i16;
    let sample_rate  : u32   = audio.sample_rate  as u32;
    let bit_rate     : i16   = audio.bit_rate     as i16;
    let data: Vec<u8> = match codec {
      Codec::LPCM => try!(LPCM::create(audio)),
    };
    let frame_size        : usize   = (num_channels * bit_rate) as usize / 8;
    let num_frames        : u32     = (audio.samples.len() / num_channels as usize) as u32;
    let extended          : Vec<u8> = convert_to_ieee_extended(sample_rate as f64);
    let comm_chunk_size   : i32     = 18; // COMM chunk always 18 since we're not adding padding or compresion
    let offset            : u32     = 0;
    let block_size        : u32     = 0;
    let data_size         : u32     = num_frames * frame_size as u32;
    let ssnd_chunk_size   : i32     = 8i32 + data_size as i32;
    let total_bytes       : u32     = 12u32 + (comm_chunk_size as u32 + 8u32) + (ssnd_chunk_size as u32 + 8u32);
    let file_size         : u32     = total_bytes - 8;
    let mut buffer        : Vec<u8> = Vec::with_capacity(total_bytes as usize);
    for _ in 0..buffer.capacity() { buffer.push(0u8); }
    debug_assert_eq!(buffer.capacity(), buffer.len());
    BigEndian::write_i32(&mut buffer[0..4], FORM);
    BigEndian::write_u32(&mut buffer[4..8], file_size);
    BigEndian::write_i32(&mut buffer[8..12], AIFF);
    BigEndian::write_i32(&mut buffer[12..16], COMM);
    BigEndian::write_i32(&mut buffer[16..20], comm_chunk_size);
    BigEndian::write_i16(&mut buffer[20..22], num_channels);
    BigEndian::write_u32(&mut buffer[22..26], num_frames);
    BigEndian::write_i16(&mut buffer[26..28], bit_rate);
    for (j,i) in (28..38).enumerate() {
      buffer[i] = extended[j];
    }
    BigEndian::write_i32(&mut buffer[38..42], SSND);
    BigEndian::write_i32(&mut buffer[42..46], ssnd_chunk_size);
    BigEndian::write_u32(&mut buffer[46..50], offset);
    BigEndian::write_u32(&mut buffer[50..54], block_size);
    let mut i = 54; // Because we're only writing AIFFs with 54 byte headers
    for byte in data.iter() {
      buffer[i] = *byte;
      i += 1;
    }
    Ok(buffer)
  }
}

/// This function reads the four character code for each AIFF chunk
///
/// If an unsupported chunk is found instead, the bytes are consumed,
/// which makes reading the remainder of the file for chunks impossible without
/// skipping the length of the chunk indicated by the next four bytes available
/// in the reader.
fn identify<R: Read + Seek>(r: &mut R) -> AudioResult<AiffChunk> {
  match try!(r.read_i32::<BigEndian>()) {
    COMM => Ok(AiffChunk::Common),
    SSND => Ok(AiffChunk::SoundData),
    err @ _ => Err(AudioError::FormatError(format!("Do not recognize AIFF chunk with identifier 0x{:x}", err)))
  }
}
