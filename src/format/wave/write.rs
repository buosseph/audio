use std::io::Write;

use byteorder::{
  LittleEndian,
  ByteOrder,
  WriteBytesExt
};
use ::encoder::AudioEncoder;
use error::*;
use format::wave::{
  RIFF,
  WAVE
};
use format::wave::chunks::*;

pub fn write<W: Write>(writer: &mut W, encoder: &mut AudioEncoder)
-> AudioResult<()> {
  // Determine if codec is supported by container and if data is non-PCM.
  let data_non_pcm: bool = try!(is_supported(encoder.codec));

  // Encode audio samples using codec.
  let data: Vec<u8> = try!(encoder.encode());
  let fmt_chunk_size = FormatChunk::calculate_size(encoder, encoder.codec);
  let mut total_bytes: u32 = 12 + (8 + fmt_chunk_size)
                                + (8 + data.len() as u32);

  // Files encoded with non-PCM data must include a fact chunk.
  if data_non_pcm {
    total_bytes += 12;
  }

  // Write the riff header to the writer.
  try!(writer.write(RIFF));
  try!(writer.write_u32::<LittleEndian>(total_bytes - 8));
  try!(writer.write(WAVE));

  // Write fmt chunk to the writer.
  try!(FormatChunk::write(writer, encoder, encoder.codec));

  // Write fact chunk to writer if data is non-PCM
  if data_non_pcm {
    try!(FactChunk::write(writer, encoder));
  }

  // Write data chunk to the writer.
  try!(DataChunk::write(writer, &data));
  Ok(())
}
