use std::io::{Read, Seek, SeekFrom, Write};
use buffer::*;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt, BigEndian};
use codecs::{Endian, Codec, AudioCodec, LPCM, SampleFormat};
use traits::{Container, Chunk};
use aiff::chunks::*;
use error::*;

/// AIFF chunk identifiers
const FORM: &'static [u8; 4] = b"FORM";
const AIFF: &'static [u8; 4] = b"AIFF";
const COMM: &'static [u8; 4] = b"COMM";
const SSND: &'static [u8; 4] = b"SSND";

/// Struct containing all necessary information
/// for encoding and decoding bytes to an `AudioBuffer`
pub struct AiffContainer {
  compression:      CompressionType,
  sample_format:    SampleFormat,
  pub bit_rate:     u32,
  pub sample_rate:  u32,
  pub channels:     u32,
  pub num_frames:   u32,
  pub order:        SampleOrder,
  pub bytes:        Vec<u8>
}

impl Container for AiffContainer {
  fn open<R: Read + Seek>(reader: &mut R) -> AudioResult<AiffContainer> {
    let header: &mut[u8] = &mut[0u8; 12];
    try!(reader.read(header));
    if &header[0..4]  != FORM
    || &header[8..12] != AIFF {
      return Err(AudioError::FormatError("Not valid AIFF".to_string()));
    }
    let     file_size : usize = BigEndian::read_i32(&header[4..8]) as usize;
    let mut pos       : i64   = 12i64;
    let mut container = 
      AiffContainer {
        compression:  CompressionType::PCM,
        sample_format: SampleFormat::Signed16,
        bit_rate:     0u32,
        sample_rate:  0u32,
        channels:     0u32,
        num_frames:   0u32,
        order:        SampleOrder::MONO,
        bytes:        Vec::new()
      };
    let mut comm_chunk_read       = false;
    let mut ssnd_chunk_read       = false;
    let mut chunk_size;
    let mut chunk_buffer;
    while pos < file_size as i64 {
      pos += 4i64;
      match identify(reader).ok() {
        Some(AiffChunk::Common) => {
          chunk_size    = try!(reader.read_i32::<BigEndian>());
          chunk_buffer  = vec![0u8; chunk_size as usize];
          try!(reader.read(&mut chunk_buffer));
          let chunk     = try!(CommonChunk::read(&chunk_buffer));
          container.compression     = CompressionType::PCM; // only option in AIFF, not AIFC
          container.bit_rate        = chunk.bit_rate      as u32;
          container.sample_rate     = chunk.sample_rate   as u32;
          container.channels        = chunk.num_channels  as u32;
          container.num_frames      = chunk.num_frames;
          comm_chunk_read           = true;
          pos                      += chunk_size          as i64;
        },
        Some(AiffChunk::SoundData) => {
          if !comm_chunk_read {
            return Err(AudioError::FormatError(
              "File is not valid AIFF \
              (Common chunk does not occur before SoundData chunk)".to_string()
            ))
          }
          chunk_size            = try!(reader.read_i32::<BigEndian>());
          chunk_buffer          = vec![0u8; chunk_size as usize];
          try!(reader.read(&mut chunk_buffer));
          // let offset      : u32 = BigEndian::read_u32(&chunk_buffer[0..4]);
          // let block_size  : u32 = BigEndian::read_u32(&chunk_buffer[4..8]);
          container.bytes       = chunk_buffer[8..].to_vec();
          ssnd_chunk_read       = true;
          pos                  += chunk_size as i64;
        },
        None => {
          chunk_size  = try!(reader.read_i32::<BigEndian>());
          pos        += chunk_size as i64;
          let new_pos = reader.seek(SeekFrom::Current(pos)).ok()
            .expect("Error while seeking in reader");
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
    container.order =
      if container.channels == 1u32 {
        SampleOrder::MONO
      } else {
        SampleOrder::INTERLEAVED
      };
    container.sample_format =
      match container.bit_rate {
        8  => SampleFormat::Signed8, // AIFF only supports i8, AIFC supports u8
        16 => SampleFormat::Signed16,
        24 => SampleFormat::Signed24,
        32 => SampleFormat::Signed32,
        _ =>
          return Err(AudioError::FormatError(
            "Audio encoded with invalid sample format".to_string()
          ))
      };
    Ok(container)
  }

  #[inline]
  fn read_codec(&mut self) -> AudioResult<Vec<Sample>> {
    let codec = match self.compression {
      CompressionType::PCM => Codec::LPCM,
      _ =>
        return Err(AudioError::UnsupportedError(
          "This file uses an unsupported codec".to_string()
        ))
    };
    match codec {
      Codec::LPCM => LPCM::read(&mut self.bytes, Endian::BigEndian, &self.bit_rate, &self.channels)
    }
  }

  fn create(codec: Codec, audio: &AudioBuffer) -> AudioResult<Vec<u8>> {
    match audio.order {
      SampleOrder::MONO    => {},
      SampleOrder::INTERLEAVED => {},
      _     =>
        return Err(AudioError::UnsupportedError(
          "Multi-channel audio must be interleaved in IFF containers".to_string()
        ))
    }
    let header_size     : u32     = 54; // Num bytes before audio samples. Always write 54 bytes
    let comm_chunk_size : i32     = 18; // COMM chunk always 18 since we're not adding padding or compresion
    let ssnd_chunk_size : i32     = 8
                                  + (audio.samples.len() as u32 * audio.bit_rate / 8) as i32;
    let total_bytes     : u32     = 12
                                  + (comm_chunk_size as u32 + 8)
                                  + (ssnd_chunk_size as u32 + 8);
    let data            : Vec<u8> =
      match codec {
        Codec::LPCM => try!(LPCM::create(audio, Endian::BigEndian)),
      };
    debug_assert_eq!(total_bytes, header_size + audio.samples.len() as u32 * audio.bit_rate / 8);
    let mut buffer      : Vec<u8> = Vec::with_capacity(total_bytes as usize);
    try!(buffer.write(FORM));
    try!(buffer.write_u32::<BigEndian>(total_bytes - 8));
    try!(buffer.write(AIFF));
    try!(buffer.write(COMM));
    try!(buffer.write_i32::<BigEndian>(comm_chunk_size));
    try!(buffer.write_i16::<BigEndian>(audio.channels as i16));
    try!(buffer.write_u32::<BigEndian>(audio.samples.len() as u32 / audio.channels));
    try!(buffer.write_i16::<BigEndian>(audio.bit_rate as i16));
    try!(buffer.write(&convert_to_ieee_extended(audio.sample_rate as f64)));
    try!(buffer.write(SSND));
    try!(buffer.write_i32::<BigEndian>(ssnd_chunk_size));
    try!(buffer.write_u32::<BigEndian>(0u32));  // offset. For now, always 0
    try!(buffer.write_u32::<BigEndian>(0u32));  // block_size. For now, always 0
    try!(buffer.write(&data));
    debug_assert_eq!(total_bytes as usize, buffer.len());
    Ok(buffer)
  }
}

/// This function reads the four character code for each AIFF chunk
///
/// If an unsupported chunk is found instead, the bytes are consumed,
/// which makes reading the remainder of the file for chunks impossible without
/// skipping the length of the chunk indicated by the next four bytes available
/// in the reader.
#[inline]
fn identify<R: Read + Seek>(reader: &mut R) -> AudioResult<AiffChunk> {
  let mut buffer = [0u8; 4];
  try!(reader.read(&mut buffer));
  match &buffer {
    COMM => Ok(AiffChunk::Common),
    SSND => Ok(AiffChunk::SoundData),
    err @ _ => 
      Err(AudioError::FormatError(
        format!("Do not recognize AIFF chunk with identifier {:?}", err)
      ))
  }
}
