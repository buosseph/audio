use std::io::{
  Cursor,
  Read,
  Seek,
  SeekFrom
};

use byteorder::{
  LittleEndian,
  ByteOrder,
  ReadBytesExt,
};
use ::decoder::AudioDecoder;
use codecs::Codec;
use codecs::Codec::*;
use error::*;
use format::wave::{
  RIFF,
  WAVE,
  FMT,
  DATA,
  FACT
};
use format::wave::chunks::*;
use traits::Chunk;


pub fn read<R: Read + Seek>(reader: &mut R)
-> AudioResult<AudioDecoder> {
  let file_size = try!(read_riff_header(reader)) as usize;
  read_chunks_for_audio(reader, file_size)
}

fn read_riff_header<R: Read>(reader: &mut R) -> AudioResult<u32> {
  // Read and validate RIFF header
  let mut riff_header = [0u8; 12];
  try!(reader.read(&mut riff_header));

  if &riff_header[0..4]  != RIFF
  || &riff_header[8..12] != WAVE {
    return Err(AudioError::Format("Not valid WAVE".to_string()));
  }

  let file_size = LittleEndian::read_u32(&riff_header[4..8]) - 4;
  Ok(file_size)
}

fn identify_chunk(header: &[u8; 4]) -> ChunkType {
  match header {
    FMT  => ChunkType::Format,
    FACT => ChunkType::Fact,
    DATA => ChunkType::Data,
    _    => ChunkType::Unknown
  }
}

fn read_chunk_header<R: Read>(reader: &mut R)
-> AudioResult<(ChunkType, usize)> {
  let mut header = [0u8; 8];
  try!(reader.read(&mut header));

  let chunk = identify_chunk(&[header[0], header[1], header[2], header[3]]);
  let size  = LittleEndian::read_u32(&header[4..8]);

  Ok((chunk, size as usize))
}

/// Returns the `Codec` used by the read audio attributes.
fn determine_codec(format_tag: FormatTag,
                   bit_depth: u16)
-> AudioResult<Codec> {
  match (format_tag, bit_depth) {
    (FormatTag::Pcm,    8) => Ok(LPCM_U8),
    (FormatTag::Pcm,   16) => Ok(LPCM_I16_LE),
    (FormatTag::Pcm,   24) => Ok(LPCM_I24_LE),
    (FormatTag::Pcm,   32) => Ok(LPCM_I32_LE),
    (FormatTag::ALaw,   8) => Ok(G711_ALAW),
    (FormatTag::MuLaw,  8) => Ok(G711_ULAW),
    (FormatTag::Float, 32) => Ok(LPCM_F32_LE),
    (FormatTag::Float, 64) => Ok(LPCM_F64_LE),
    (_, _) =>
      return Err(AudioError::Unsupported(
        "Audio encoded with unsupported codec".to_string()
      ))
  }
}

// Seeks through the reader to find the required chunks for audio decoding, all
// other chunks are ignored and skipped.
fn read_chunks_for_audio<R: Read + Seek>(reader: &mut R,
                                         file_size: usize)
-> AudioResult<AudioDecoder> {
  let mut read_fmt_chunk  = false;
  let mut read_fact_chunk = false;

  let mut buffer = Cursor::new(vec![0u8; file_size]);
  try!(reader.read(buffer.get_mut()));

  let mut iterator = AudioDecoder::new();

  loop {
    // If no data chunk is found, then EOF erorr will be returned
    let (chunk, size) = try!(read_chunk_header(&mut buffer));
    let pos           = buffer.position() as usize;

    match chunk {
      ChunkType::Format => {
        let bytes = &(buffer.get_ref()[pos .. pos + size]);
        let chunk = try!(FormatChunk::read(&bytes));

        iterator.bit_depth   = chunk.bit_depth    as u32;
        iterator.sample_rate = chunk.sample_rate  as u32;
        iterator.channels    = chunk.num_channels as u32;
        // iterator.num_frames  = chunk.num_frames;

        iterator.codec  = try!(determine_codec(chunk.format_tag,
                                               chunk.bit_depth));

        read_fmt_chunk = true;
        if chunk.format_tag == FormatTag::Pcm {
          // Don't need to check for Fact chunk
          read_fact_chunk = true;
        }
      },

      ChunkType::Fact => {
        // Don't actually use it, but we do need to check if it exists.
        // let chunk_bytes   = &(buffer.get_ref()[pos .. pos + chunk_size]);
        // let num_samples_per_channel = LittleEndian::read_u32(&chunk_bytes[0..4]);
        read_fact_chunk = true
      },

      ChunkType::Data => {
        if !read_fmt_chunk {
          return Err(AudioError::Format(
            "File is not valid WAVE \
            (Format chunk does not occur before Data chunk)".to_string()
          ))
        }

        let bytes = &(buffer.get_ref()[pos .. pos + size]);

        iterator.data.clear();
        iterator.data.extend_from_slice(&bytes[..]);
        break;
      },

      // Skip all other chunks unnecessary for audio decoding
      _ => {}
    }

    try!(buffer.seek(SeekFrom::Current(size as i64)));
  }

  // Check if required chunks were read
  if !read_fmt_chunk {
    return Err(AudioError::Format(
      "File is not valid WAVE (Missing required Format chunk)".to_string()
    ))
  }
  if !read_fact_chunk {
    return Err(AudioError::Format(
      "File is not valid WAVE \
      (Missing Fact chunk for non-PCM data)".to_string()
    ))
  }

  Ok(iterator)
}
