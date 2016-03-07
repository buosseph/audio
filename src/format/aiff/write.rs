use std::io::Write;

use byteorder::{
  BigEndian,
  ByteOrder,
  WriteBytesExt,
};
use ::encoder::AudioEncoder;
use error::*;
use format::aiff::{
  AIFF,
  AIFC,
  AIFC_VERSION_1,
  FORM,
  FVER
};
use format::aiff::chunks::*;

pub fn write<W: Write>(writer: &mut W, encoder: &mut AudioEncoder)
-> AudioResult<()> {
  // Determine if codec is supported by container and if it's supported by
  // aiff or aiff-c.
  let aifc: bool    = try!(is_aifc(encoder.codec));

  // Encode audio samples using codec.
  let data: Vec<u8> = try!(encoder.encode());
  let comm_chunk_size: u32 = try!(CommonChunk::calculate_size(encoder.codec)) as u32;

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
  try!(CommonChunk::write(writer, &encoder));


  // Write ssnd chunk to the writer.
  try!(SoundDataChunk::write(writer, &data));
  Ok(())
}
