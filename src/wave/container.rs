use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use buffer::*;
use buffer::SampleOrder::*;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use codecs::{AudioCodec, Codec, Endian, LPCM, SampleFormat};
use codecs::SampleFormat::*;
use traits::{Chunk, Container};
use wave::chunks::*;
use wave::chunks::WaveChunk::*;
use error::*;

/// WAVE chunk identifiers.
const RIFF: &'static [u8; 4] = b"RIFF";
const WAVE: &'static [u8; 4] = b"WAVE";
const FMT:  &'static [u8; 4] = b"fmt ";
const DATA: &'static [u8; 4] = b"data";

/// Struct containing all necessary information for encoding and decoding
/// bytes to an `AudioBuffer`.
pub struct WaveContainer {
  compression:      CompressionType,
  sample_format:    SampleFormat,
  pub bit_rate:     u32,
  pub sample_rate:  u32,
  pub channels:     u32,
  pub block_size:   u32,
  pub order:        SampleOrder,
  pub samples:      Vec<Sample>
}

impl Container for WaveContainer {
  fn open<R: Read + Seek>(reader: &mut R) -> AudioResult<WaveContainer> {
    // Read and validate riff header
    let mut riff_header: [u8; 12] = [0u8; 12];
    try!(reader.read(&mut riff_header));
    if &riff_header[0..4]  != RIFF
    || &riff_header[8..12] != WAVE {
      return Err(AudioError::FormatError(
        "Not valid WAVE".to_string()
      ));
    }
    let file_size : u32 = LittleEndian::read_u32(&riff_header[4..8]);
    let mut buffer: Cursor<Vec<u8>> = Cursor::new(vec![0u8; file_size as usize]);
    try!(reader.read(buffer.get_mut()));
    // Read all supported chunks
    let mut container =
      WaveContainer {
        compression:    CompressionType::PCM,
        sample_format:  SampleFormat::Signed16,
        bit_rate:       0u32,
        sample_rate:    0u32,
        channels:       1u32,
        block_size:     0u32,
        order:          SampleOrder::MONO,
        samples:        Vec::with_capacity(1024)
      };
    let mut chunk_header    : [u8; 8] = [0u8; 8];
    let mut read_fmt_chunk  : bool    = false;
    let mut read_data_chunk : bool    = false;
    while buffer.position() < file_size as u64 {
      try!(buffer.read(&mut chunk_header));
      let chunk_size  : usize = 
        LittleEndian::read_u32(&chunk_header[4..8]) as usize;
      let pos         : usize = buffer.position() as usize;
      match identify(&chunk_header[0..4]).ok() {
        Some(Format) => {
          let chunk_bytes = &(buffer.get_ref()[pos .. pos + chunk_size]);
          let fmt_chunk = try!(FormatChunk::read(&chunk_bytes));
          container.compression     = fmt_chunk.compression_type;
          container.bit_rate        = fmt_chunk.bit_rate      as u32;
          container.sample_rate     = fmt_chunk.sample_rate;
          container.channels        = fmt_chunk.num_channels  as u32;
          container.block_size      = fmt_chunk.block_size    as u32;
          container.order           =
            if container.channels == 1 {
              SampleOrder::MONO
            } else {
              SampleOrder::INTERLEAVED
            };
          container.sample_format   =
            match container.bit_rate {
              8  => SampleFormat::Unsigned8,
              16 => SampleFormat::Signed16,
              24 => SampleFormat::Signed24,
              32 => SampleFormat::Signed32,
              _ =>
                return Err(AudioError::FormatError(
                  "Audio encoded with invalid sample format".to_string()
                ))
            };
          read_fmt_chunk            = true;
        },
        Some(Data) => {
          if !read_fmt_chunk {
            return Err(AudioError::FormatError(
              "File is not valid WAVE \
              (Format chunk does not occur before Data chunk)".to_string()
            ))
          }
          let chunk_bytes = &(buffer.get_ref()[pos .. pos + chunk_size]);
          container.samples =
            try!(read_codec(chunk_bytes, container.compression,
                            container.sample_format, Endian::LittleEndian));
          read_data_chunk = true;
        },
        None => {}
      }
      try!(buffer.seek(SeekFrom::Current(chunk_size as i64)));
    }
    // Check if required chunks were read
    if !read_fmt_chunk {
      return Err(AudioError::FormatError(
        "File is not valid WAVE (Missing required Format chunk)".to_string()
      ))
    }
    else if !read_data_chunk {
      return Err(AudioError::FormatError(
        "File is not valid WAVE (Missing required Data chunk)".to_string()
      ))
    }
    Ok(container)
  }
  fn create<W: Write>(writer: &mut W, audio: &AudioBuffer, sample_format: SampleFormat, codec: Codec) -> AudioResult<()> {
    // Determine if the sample order of the AudioBuffer is supported by the 
    // wave format.
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
      Unsigned8 |
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
        Codec::LPCM => try!(LPCM::create(audio, sample_format, Endian::LittleEndian)),
      };
    // Wave files created by this library do not support compression, so the
    // format chunk will always be the same size: 16 bytes.
    let fmt_chunk_size  : u32     = 16;
    // Total number of bytes is determined by chunk sizes and the RIFF header,
    // which is always 12 bytes. Every chunk specifies their size but doesn't
    // include the chunk header, the first 8 bytes which contain the chunk
    // identifier and chunk size.
    //
    // Currently, wave files created by this library only contains the necessary
    // chunks for audio playback with no option for adding additional chunks for
    // metadata.
    let total_bytes     : u32     = 12
                                  + (8 + fmt_chunk_size)
                                  + (8 + data.len() as u32);
    // Write the riff header to the writer.
    try!(writer.write(RIFF));
    try!(writer.write_u32::<LittleEndian>(total_bytes - 8));
    try!(writer.write(WAVE));
    // Write fmt chunk to the writer.
    try!(writer.write(FMT));
    try!(writer.write_u32::<LittleEndian>(fmt_chunk_size));
    // Currently, all wave files created by this library will only be encoded
    // using LPCM, the format standard.
    try!(writer.write_u16::<LittleEndian>(1u16));
    try!(writer.write_u16::<LittleEndian>(audio.channels as u16));
    try!(writer.write_u32::<LittleEndian>(audio.sample_rate as u32));
    try!(writer.write_u32::<LittleEndian>(
      audio.sample_rate * audio.channels * audio.bit_rate / 8u32));
    try!(writer.write_u16::<LittleEndian>(
      (audio.channels * audio.bit_rate / 8u32) as u16));
    try!(writer.write_u16::<LittleEndian>(audio.bit_rate as u16));
    // Write data chunk to the writer.
    try!(writer.write(DATA));
    try!(writer.write_u32::<LittleEndian>(
      (audio.samples.len() * ((audio.bit_rate) as usize / 8)) as u32));
    try!(writer.write_all(&data));
    Ok(())
  }
}

/// This function reads the four byte identifier for each WAVE chunk.
#[inline]
fn identify(bytes: &[u8]) -> AudioResult<WaveChunk> {
  match &[bytes[0], bytes[1], bytes[2], bytes[3]] {
    FMT  => Ok(Format),
    DATA => Ok(Data),
    err @ _ => 
      Err(AudioError::FormatError(
        format!("Do not recognize WAVE chunk with identifier {:?}", err)
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
