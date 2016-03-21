use std::io::{
  Cursor,
  Read,
  Seek,
  SeekFrom
};

use byteorder::{
  BigEndian,
  ByteOrder,
  ReadBytesExt,
};
use ::decoder::AudioDecoder;
use codecs::Codec;
use codecs::Codec::*;
use error::*;
use format::aiff::{
  AIFF,
  AIFC,
  FORM,
  FVER,
  COMM,
  SSND
};
use format::aiff::chunks::*;
use format::aiff::chunks::CompressionType::*;
use traits::Chunk;


pub fn read<R: Read + Seek>(reader: &mut R)
-> AudioResult<AudioDecoder> {
  let file_size = try!(read_iff_header(reader)) as usize;
  read_chunks_for_audio(reader, file_size)
}

fn read_iff_header<R: Read>(reader: &mut R) -> AudioResult<i32> {
  // Read and validate IFF header
  let mut iff_header = [0u8; 12];
  try!(reader.read(&mut iff_header));
  if &iff_header[0..4] != FORM {
    return Err(AudioError::Format("Not valid IFF".to_string()));
  }

  // Determine if container is AIFF or AIFF-C
  if &iff_header[8..12] != AIFF
  && &iff_header[8..12] != AIFC {
    return Err(AudioError::Format("Not valid AIFF or AIFF-C".to_string()));
  }

  let file_size = BigEndian::read_i32(&iff_header[4..8]) - 4;
  Ok(file_size)
}

fn identify_chunk(header: &[u8; 4]) -> ChunkType {
  match header {
    FVER => ChunkType::FormatVersion,
    COMM => ChunkType::Common,
    SSND => ChunkType::SoundData,
    _    => ChunkType::Unknown
  }
}

fn read_chunk_header<R: Read>(reader: &mut R)
-> AudioResult<(ChunkType, usize)> {
  let mut header = [0u8; 8];
  try!(reader.read(&mut header));

  let chunk    = identify_chunk(&[header[0], header[1], header[2], header[3]]);
  let mut size = BigEndian::read_i32(&header[4..8]);

  // AIFF chunk sizes must always be even and may not specify the trailing
  // byte in the read chunk size. This can occur in the sound data chunk,
  // textual chunks, the midi chunk, and the application specific chunk.
  // The last two are not supported in this library.
  if size % 2 != 0 {
    size += 1;
  }

  Ok((chunk, size as usize))
}

/// Returns the `Codec` used by the read audio attributes.
fn determine_codec(compression_type: CompressionType,
                   bit_depth: i16)
-> AudioResult<Codec> {
  match (compression_type, bit_depth) {
    // AIFC supports:
    (Raw,     8 ) => Ok(LPCM_U8),
    (ALaw,    16) => Ok(G711_ALAW),
    (MuLaw,   16) => Ok(G711_ULAW),
    (Float32, 32) => Ok(LPCM_F32_BE),
    (Float64, 64) => Ok(LPCM_F64_BE),
    // AIFF supports:
    (Pcm, 8 ) => Ok(LPCM_I8),
    (Pcm, 16) => Ok(LPCM_I16_BE),
    (Pcm, 24) => Ok(LPCM_I24_BE),
    (Pcm, 32) => Ok(LPCM_I32_BE),
    ( _ , _ ) =>
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
  let mut read_fver_chunk = false;
  let mut read_comm_chunk = false;

  let mut buffer = Cursor::new(vec![0u8; file_size]);
  try!(reader.read(buffer.get_mut()));

  let mut iterator = AudioDecoder::new();

  loop {
    // If no sound data chunk is found, then EOF error will be returned
    let (chunk, size) = try!(read_chunk_header(&mut buffer));
    let pos           = buffer.position() as usize;

    match chunk {
      ChunkType::FormatVersion => {
        read_fver_chunk = true;
      },

      ChunkType::Common => {
        let bytes = &(buffer.get_ref()[pos .. pos + size]);
        let chunk = try!(CommonChunk::read(&bytes));

        iterator.bit_depth   = chunk.bit_depth    as u32;
        iterator.sample_rate = chunk.sample_rate  as u32;
        iterator.channels    = chunk.num_channels as u32;

        iterator.codec  = determine_codec(chunk.compression_type,
                                          chunk.bit_depth).ok();
        read_comm_chunk = true;
      },

      ChunkType::SoundData => {
        if !read_comm_chunk {
          return Err(AudioError::Format(
            "File is not valid AIFF \
            (Common chunk must occur before SoundData chunk)".to_string()
          ))
        }

        // First 8 bytes are offset and block_size, skip them
        let bytes = &(buffer.get_ref()[pos .. pos + size]);

        iterator.data.clear();
        iterator.data.extend_from_slice(&bytes[8..]);
        break;
      },

      // Skip all other chunks unnecessary for audio decoding
      _ => {}
    }

    try!(buffer.seek(SeekFrom::Current(size as i64)));
  }

  // Check if required chunks were read
  if try!(is_aifc(iterator.codec.unwrap())) && !read_fver_chunk {
    return Err(AudioError::Format(
      "File is not valid AIFC \
      (Missing required FormatVersion chunk)".to_string()
    ))
  }

  if !read_comm_chunk {
    return Err(AudioError::Format(
      "File is not valid AIFF \
      (Missing required Common chunk)".to_string()
    ))
  }

  Ok(iterator)
}
