use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use aiff::{AIFF, AIFC, AIFC_VERSION_1, FORM, FVER, COMM, SSND};
use aiff::chunks::*;
use aiff::chunks::AiffChunk::*;
use aiff::chunks::CompressionType::*;
use buffer::*;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use codecs::Codec;
use codecs::Codec::*;
use error::*;
use sample::*;
use sample::SampleOrder::*;
use traits::{Chunk, Container};

/// Struct containing all necessary information for encoding and decoding
/// bytes to an `AudioBuffer`.
pub struct AiffContainer {
  codec:            Codec,
  pub bit_depth:    u32,
  pub sample_rate:  u32,
  pub channels:     u32,
  pub num_frames:   u32,
  pub order:        SampleOrder,
  pub samples:      Vec<Sample>
}

impl Container for AiffContainer {
  fn open<R: Read + Seek>(reader: &mut R) -> AudioResult<AiffContainer> {
    // Read and validate IFF header
    let mut iff_header: [u8; 12] = [0u8; 12];
    try!(reader.read(&mut iff_header));
    if &iff_header[0..4] != FORM {
      return Err(AudioError::Format(
        "Not valid IFF".to_string()
      ));
    }
    // Determine if container is AIFF or AIFF-C
    if &iff_header[8..12] != AIFF && &iff_header[8..12] != AIFC {
      return Err(AudioError::Format(
        "Not valid AIFF or AIFF-C".to_string()
      ));
    }
    let file_size: i32 = BigEndian::read_i32(&iff_header[4..8]) - 4;
    let mut buffer: Cursor<Vec<u8>> = Cursor::new(vec![0u8; file_size as usize]);
    try!(reader.read(buffer.get_mut()));

    // Read all supported chunks
    let mut container = 
      AiffContainer {
        codec:          LPCM_I16_BE,
        bit_depth:      0u32,
        sample_rate:    0u32,
        channels:       1u32,
        num_frames:     0u32,
        order:          SampleOrder::Mono,
        samples:        Vec::with_capacity(1024)
      };
    let mut chunk_header    : [u8; 8] = [0u8; 8];
    let mut read_fver_chunk : bool    = false;
    let mut read_comm_chunk : bool    = false;
    let mut read_ssnd_chunk : bool    = false;
    while buffer.position() < file_size as u64 {
      try!(buffer.read(&mut chunk_header));
      let mut chunk_size: usize =
        BigEndian::read_i32(&chunk_header[4..8]) as usize;
      // AIFF chunk sizes must always be even and may not specify the trailing
      // byte in the read chunk_size. This can occur in the sound data chunk,
      // textual chunks, the midi chunk, and the application specific chunk.
      // The last two are not supported in this library.
      if chunk_size % 2 != 0 {
        chunk_size += 1;
      }
      let pos: usize = buffer.position() as usize;
      match identify(&chunk_header[0..4]).ok() {
        Some(FormatVersion) => {
          read_fver_chunk = true;
        }
        Some(Common) => {
          let chunk_bytes = &(buffer.get_ref()[pos .. pos + chunk_size]);
          let comm_chunk  = try!(CommonChunk::read(&chunk_bytes));
          container.bit_depth       = comm_chunk.bit_depth    as u32;
          container.sample_rate     = comm_chunk.sample_rate  as u32;
          container.channels        = comm_chunk.num_channels as u32;
          container.num_frames      = comm_chunk.num_frames;
          container.order           =
            if container.channels == 1 {
              SampleOrder::Mono
            } else {
              SampleOrder::Interleaved
            };
          container.codec           =
            try!(determine_codec(comm_chunk.compression_type,
                                 comm_chunk.bit_depth));
          read_comm_chunk           = true;
        },
        Some(SoundData) => {
          if !read_comm_chunk {
            return Err(AudioError::Format(
              "File is not valid AIFF \
              (Common chunk does not occur before SoundData chunk)".to_string()
            ))
          }
          let chunk_bytes   = &(buffer.get_ref()[pos .. pos + chunk_size]);
          // let offset      : u32 = BigEndian::read_u32(&chunk_bytes[0..4]);
          // let block_size  : u32 = BigEndian::read_u32(&chunk_bytes[4..8]);
          container.samples = try!(read_codec(&chunk_bytes[8..], container.codec));
          read_ssnd_chunk   = true;
        },
        None => {}
      }
      try!(buffer.seek(SeekFrom::Current(chunk_size as i64)));
    }

    // Check if required chunks were read
    if try!(is_aifc(container.codec)) && !read_fver_chunk {
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
    if !read_ssnd_chunk {
      return Err(AudioError::Format(
        "File is not valid AIFF \
        (Missing required SoundData chunk)".to_string()
      ))
    }
    Ok(container)
  }
  fn create<W: Write>(writer: &mut W, audio: &AudioBuffer, codec: Codec) -> AudioResult<()> {
    // Determine if the sample order of the AudioBuffer is supported by the 
    // aiff format.
    match audio.order {
      Mono        => {},
      Interleaved => {},
      _           => 
        return Err(AudioError::Unsupported(
          "Multi-channel audio must be interleaved in RIFF containers".to_string()
        ))
    }

    // Determine if codec is supported by container and if it's supported by
    // aiff or aiff-c.
    let aifc: bool    = try!(is_aifc(codec));
    // Encode audio samples using codec.
    let data: Vec<u8> = try!(write_codec(audio, codec));
    let comm_chunk_size: u32 = try!(CommonChunk::calculate_size(codec)) as u32;
    // The ssnd chunk contains 8 additional bytes besides the audio data.
    let ssnd_chunk_size: u32 = 8 + data.len() as u32;
    let mut total_bytes: u32  = 12 + (8 + comm_chunk_size)
                                   + (8 + ssnd_chunk_size);
    // Aiff-c files must include a format version chunk.
    if aifc {
      total_bytes += 12;
    }

    // Write the iff header to the writer.
    try!(writer.write(FORM));
    try!(writer.write_u32::<BigEndian>(total_bytes - 8));
    if aifc {
      try!(writer.write(AIFC));
      // Write form version chunk to writer. Requried by aiff-c.
      try!(writer.write(FVER));
      try!(writer.write_u32::<BigEndian>(4));
      try!(writer.write_u32::<BigEndian>(AIFC_VERSION_1));
    }
    else {
      try!(writer.write(AIFF));
    }
    // Write comm chunk to the writer.
    try!(CommonChunk::write(writer, audio, codec));
    // Write ssnd chunk to the writer.
    try!(SoundDataChunk::write(writer, &data));
    Ok(())
  }
}

// Private functions

/// This function reads the four byte identifier for each AIFF chunk.
#[inline]
fn identify(bytes: &[u8]) -> AudioResult<AiffChunk> {
  match &[bytes[0], bytes[1], bytes[2], bytes[3]] {
    FVER => Ok(FormatVersion),
    COMM => Ok(Common),
    SSND => Ok(SoundData),
    err @ _ => 
      Err(AudioError::Format(
        format!("Do not recognize AIFF chunk with identifier {:?}", err)
      ))
  }
}

/// Determines if codec is supported by container.
fn is_supported(codec: Codec) -> AudioResult<bool> {
  match codec {
    LPCM_U8      |
    LPCM_I8      |
    LPCM_I16_BE  |
    LPCM_I24_BE  |
    LPCM_I32_BE  |
    LPCM_F32_BE  |
    LPCM_F64_BE  |
    G711_ALAW    |
    G711_ULAW    => Ok(true),
    c @ _ =>
      return Err(AudioError::Unsupported(
        format!("Aiff does not support the {:?} codec", c)
      ))
  }
}

/// Returns the `Codec` used by the read audio attributes.
fn determine_codec(compression_type: CompressionType, bit_depth: i16) -> AudioResult<Codec> {
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
    ( _ , _ ) => return
      Err(AudioError::Unsupported(
        "Audio encoded with unsupported codec".to_string()
      ))
  }
}

/// Returns samples read using the given codec. If the container does not
/// support a codec, an error is returned.
fn read_codec(bytes: &[u8], codec: Codec) -> AudioResult<Vec<Sample>> {
  match is_supported(codec) {
    Ok(_)  => ::codecs::decode(bytes, codec),
    Err(e) => Err(e)
  }
}

/// Returns samples as bytes created using the given codec. If the container
/// does not support a codec, an error is returned.
fn write_codec(audio: &AudioBuffer, codec: Codec) -> AudioResult<Vec<u8>> {
  match is_supported(codec) {
    Ok(_)  => ::codecs::encode(audio, codec),
    Err(e) => Err(e)
  }
}
