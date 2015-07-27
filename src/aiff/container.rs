use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use buffer::*;
use buffer::SampleOrder::*;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use codecs::{AudioCodec, Codec, Endian, LPCM, SampleFormat};
use codecs::SampleFormat::*;
use traits::{Chunk, Container};
use aiff::chunks::*;
use aiff::chunks::AiffChunk::*;
use error::*;

/// AIFF chunk identifiers.
const FORM: &'static [u8; 4] = b"FORM";
const AIFF: &'static [u8; 4] = b"AIFF";
const COMM: &'static [u8; 4] = b"COMM";
const SSND: &'static [u8; 4] = b"SSND";

/// Struct containing all necessary information for encoding and decoding
/// bytes to an `AudioBuffer`.
pub struct AiffContainer {
  compression:      CompressionType,
  sample_format:    SampleFormat,
  pub bit_rate:     u32,
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
    if &iff_header[0..4]  != FORM
    || &iff_header[8..12] != AIFF {
      return Err(AudioError::FormatError(
        "Not valid AIFF".to_string()
      ));
    }
    let file_size : i32 = BigEndian::read_i32(&iff_header[4..8]);
    let mut buffer: Cursor<Vec<u8>> = Cursor::new(vec![0u8; file_size as usize]);
    try!(reader.read(buffer.get_mut()));
    // Read all supported chunks
    let mut container = 
      AiffContainer {
        compression:    CompressionType::PCM,
        sample_format:  SampleFormat::Signed16,
        bit_rate:       0u32,
        sample_rate:    0u32,
        channels:       1u32,
        num_frames:     0u32,
        order:          SampleOrder::MONO,
        samples:        Vec::with_capacity(1024)
      };
    let mut chunk_header    : [u8; 8] = [0u8; 8];
    let mut read_comm_chunk : bool    = false;
    let mut read_ssnd_chunk : bool    = false;
    while buffer.position() < file_size as u64 {
      try!(buffer.read(&mut chunk_header));
      let chunk_size  : usize =
        BigEndian::read_i32(&chunk_header[4..8]) as usize;
      let pos         : usize = buffer.position() as usize;
      match identify(&chunk_header[0..4]).ok() {
        Some(Common) => {
          let chunk_bytes = &(buffer.get_ref()[pos .. pos + chunk_size]);
          let comm_chunk  = try!(CommonChunk::read(&chunk_bytes));
          container.compression     = CompressionType::PCM; // only option in AIFF, not AIFC
          container.bit_rate        = comm_chunk.bit_rate      as u32;
          container.sample_rate     = comm_chunk.sample_rate   as u32;
          container.channels        = comm_chunk.num_channels  as u32;
          container.num_frames      = comm_chunk.num_frames;
          container.order           =
            if container.channels == 1 {
              SampleOrder::MONO
            } else {
              SampleOrder::INTERLEAVED
            };
          container.sample_format   =
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
          read_comm_chunk           = true;
        },
        Some(SoundData) => {
          if !read_comm_chunk {
            return Err(AudioError::FormatError(
              "File is not valid AIFF \
              (Common chunk does not occur before SoundData chunk)".to_string()
            ))
          }
          let chunk_bytes = &(buffer.get_ref()[pos .. pos + chunk_size]);
          // let offset      : u32 = BigEndian::read_u32(&chunk_bytes[0..4]);
          // let block_size  : u32 = BigEndian::read_u32(&chunk_bytes[4..8]);
          container.samples =
            try!(read_codec(&chunk_bytes[8..], container.compression,
                            container.sample_format, Endian::BigEndian));
          read_ssnd_chunk       = true;
        },
        None => {}
      }
      try!(buffer.seek(SeekFrom::Current(chunk_size as i64)));
    }
    // Check if required chunks were read
    if !read_comm_chunk {
      return Err(AudioError::FormatError(
        "File is not valid AIFF \
        (Missing required Common chunk)".to_string()
      ))
    }
    else if !read_ssnd_chunk {
      return Err(AudioError::FormatError(
        "File is not valid AIFF \
        (Missing required SoundData chunk)".to_string()
      ))
    }
    Ok(container)
  }
  fn create<W: Write>(writer: &mut W, audio: &AudioBuffer, sample_format: SampleFormat, codec: Codec) -> AudioResult<()> {
    // Determine if the sample order of the AudioBuffer is supported by the 
    // aiff format.
    match audio.order {
      MONO        => {},
      INTERLEAVED => {},
      _           => 
        return Err(AudioError::UnsupportedError(
          "Multi-channel audio must be interleaved in RIFF containers".to_string()
        ))
    }
    // Determine if sample format is supported by wave format.
    match sample_format {
      Signed8   |
      Signed16  |
      Signed24  |
      Signed32  => {},
      sf @ _    => 
        return Err(AudioError::FormatError(
          format!("Wave format does not support {:?} sample format", sf)
        ))
    }
    // Convert the audio samples to the format of the corresponding codec.
    let data            : Vec<u8> = 
      match codec {
        Codec::LPCM => try!(LPCM::create(audio, sample_format, Endian::BigEndian)),
      };
    // Aiff files created by this library do not support compression, so the
    // comm chunk will always be the same size: 18 bytes.
    let comm_chunk_size : i32     = 18;
    // The ssnd chunk contains additional information besides the audio data.
    let ssnd_chunk_size : i32     = 8 
                                  + data.len() as i32;
    // Total number of bytes is determined by chunk sizes and the IFF header,
    // which is always 12 bytes. Every chunk specifies their size but doesn't
    // include the chunk header, the first 8 bytes which contain the chunk
    // identifier and chunk size.
    //
    // Currently, wave files created by this library only contains the necessary
    // chunks for audio playback with no option for adding additional chunks for
    // metadata.
    let total_bytes     : u32     = 12 
                                  + (8 + comm_chunk_size as u32)
                                  + (8 + ssnd_chunk_size as u32);
    // Write the iff header to the writer.
    try!(writer.write(FORM));
    try!(writer.write_u32::<BigEndian>(total_bytes - 8));
    try!(writer.write(AIFF));
    // Write comm chunk to the writer.
    try!(writer.write(COMM));
    try!(writer.write_i32::<BigEndian>(comm_chunk_size));
    try!(writer.write_i16::<BigEndian>(audio.channels as i16));
    try!(writer.write_u32::<BigEndian>(audio.samples.len() as u32 / audio.channels));
    try!(writer.write_i16::<BigEndian>(audio.bit_rate as i16));
    try!(writer.write(&convert_to_ieee_extended(audio.sample_rate as f64)));
    // Write ssnd chunk to the writer.
    try!(writer.write(SSND));
    try!(writer.write_i32::<BigEndian>(ssnd_chunk_size));
    try!(writer.write_u32::<BigEndian>(0u32));   // offset. For now, always 0
    try!(writer.write_u32::<BigEndian>(0u32));   // block_size. For now, always 0
    try!(writer.write_all(&data));
    Ok(())
  }
}

/// This function reads the four byte identifier for each AIFF chunk.
#[inline]
fn identify(bytes: &[u8]) -> AudioResult<AiffChunk> {
  match &[bytes[0], bytes[1], bytes[2], bytes[3]] {
    COMM => Ok(Common),
    SSND => Ok(SoundData),
    err @ _ => 
      Err(AudioError::FormatError(
        format!("Do not recognize AIFF chunk with identifier {:?}", err)
      ))
  }
}
fn read_codec(bytes: &[u8],
              compression: CompressionType,
              sample_format: SampleFormat,
              endian: Endian) -> AudioResult<Vec<Sample>> {
  let codec = match compression {
    CompressionType::PCM => Codec::LPCM,
    _ =>
      return Err(AudioError::UnsupportedError(
        "This file uses an unsupported codec".to_string()
      ))
  };
  match codec {
    Codec::LPCM => LPCM::read(bytes, sample_format, endian)
  }
}
