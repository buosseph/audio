//! The Audio Interchange File Format
//!
//! AIFF files use the Interchange File Format (IFF), a generic file container
//! format that uses chunks to store data. All bytes are stored in big-endian
//! format.
//!
//! References
//! - [McGill University](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/AIFF/AIFF.html)
//! - [AIFF Spec](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/AIFF/Docs/AIFF-1.3.pdf)
//! - [AIFF/AIFFC Spec from Apple](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/AIFF/Docs/MacOS_Sound-extract.pdf)

use std::io::{
  Error,
  ErrorKind,
  Read,
  Seek,
  SeekFrom,
  Write
};

use ::aiff::chunks::*;
use buffer::AudioBuffer;
use byteorder::{
  BigEndian,
  ByteOrder,
  ReadBytesExt,
  WriteBytesExt
};
use codecs::Codec;
use codecs::Codec::*;
use error::*;
use sample::*;

mod chunks;

/// AIFF/AIFC chunk identifiers.
const FORM: &'static [u8; 4] = b"FORM";
const AIFF: &'static [u8; 4] = b"AIFF";
const AIFC: &'static [u8; 4] = b"AIFC";
const FVER: &'static [u8; 4] = b"FVER";
const COMM: &'static [u8; 4] = b"COMM";
const SSND: &'static [u8; 4] = b"SSND";

/// AIFF-C Version 1 timestamp for the FVER chunk.
const AIFC_VERSION_1: u32 = 0xA2805140;


// TODO: Remove duplicate constants
// AIFF-C compression types and strings
const NONE: (&'static [u8; 4], &'static [u8]) =
  (b"NONE", b"not compressed");
const RAW : (&'static [u8; 4], &'static [u8]) =
  (b"raw ", b"");
const ULAW: (&'static [u8; 4], &'static [u8]) =
  (b"ulaw", &[0xB5, 0x6C, 0x61, 0x77, 0x20, 0x32, 0x3A, 0x31]); // µLaw 2:1
const ALAW: (&'static [u8; 4], &'static [u8]) =
  (b"alaw", b"ALaw 2:1");
const FL32: (&'static [u8; 4], &'static [u8]) =
  (b"fl32", b"IEEE 32-bit float");
const FL64: (&'static [u8; 4], &'static [u8]) =
  (b"fl64", b"IEEE 64-bit float");

/// All supported AIFF format variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FormatVariant {
  /// Standard AIFF.
  ///
  /// This format only supports uncompressed LPCM codecs.
  Aiff,
  /// AIFF-C
  ///
  /// This format supports compressed codecs.
  Aifc
}

// ----------------------------------------------------------
// Decoding
// ----------------------------------------------------------

/// Searches for a IFF FORM header, from the start of the given reader.
///
/// This function positions the reader at the start of the IFF header,
/// which must be read and validated using `validate_container`.
fn find_iff_header<R: Read + Seek>(reader: &mut R) -> AudioResult<()> {
  try!(reader.seek(SeekFrom::Start(0)));

  let res = find_chunk(reader, FORM);
  if let Some(e) = res.err() {
    use std::error::Error;
    if e.description() == "Chunk not found" {
      return Err(AudioError::Format(
                 "Unable to find IFF header".to_string()));
    }
    return Err(e);
  }

  // Reset to read entire header in validate_container()
  try!(reader.seek(SeekFrom::Current(-8)));
  Ok(())
}

/// Reads the IFF header and returns the format variant and size of the data
/// within the IFF container.
fn validate_container<R: Read + Seek>(reader: &mut R) -> AudioResult<(FormatVariant, i32)> {
  let mut iff_header = [0u8; 12];
  try!(reader.read(&mut iff_header));
  if &iff_header[0..4] != FORM {
    println!("{:?}", &iff_header[0..4]);
    return Err(AudioError::Format("Not a valid IFF format".to_string()));
  }

  // Determine if container is AIFF or AIFF-C
  let variant =
    if &iff_header[8..12] == AIFF {
      FormatVariant::Aiff
    }
    else if &iff_header[8..12] == AIFC {
      FormatVariant::Aifc
    }
    else {
      return Err(AudioError::Format("Not a valid AIFF or AIFF-C".to_string()));
    };

  let file_size: i32 = BigEndian::read_i32(&iff_header[4..8]) - 4;

  Ok((variant, file_size))
}

// TODO: Verify that FVER must come immediately after the IFF header, else
//       rewrite function to seek for chunk. Once found, reset to position
//       the reader was at at the start of this function call.
/// Reads and checks for a malformed FVER chunk in an AIFF-C file.
fn find_aifc_fver<R: Read + Seek>(reader: &mut R) -> AudioResult<()> {
  let mut buffer = [0u8; 12];
  try!(reader.read(&mut buffer));
  if &buffer[0..4] == FVER
  && BigEndian::read_u32(&buffer[4..8]) == 4
  && BigEndian::read_u32(&buffer[8..12]) == AIFC_VERSION_1 {
     Ok(())
  }
  else if &buffer[0..4] != FVER {
    Err(AudioError::Format(
      "AIFF-C is missing format version chunk".to_string()))
  }
  else {
    Err(AudioError::Format(
      "AIFF-C has invalid format version chunk".to_string()))
  }
}

/// Searches the reader from the current position for a chunk with the given
/// identifier and returns the offset of the chunk from the previous position
/// of the reader and the read chunk_size. The chunk header is read, so the
/// next number of bytes equal to the returned chunk_size belong to the found
/// chunk.
///
/// The function starts its search by reading the next 8 bytes in the reader,
/// checking the first 4 bytes for the identifier. If the identifier doesn't
/// match, then the last 4 bytes are interpreted as the chunk size. A number
/// of bytes equal to the read chunk_size is skipped before reading another
/// 8 bytes to check for the chunk identifier. The funcion returns an 
/// `io::Error` if no more bytes are read into the chunk header buffer.
fn find_chunk<R: Read + Seek>(reader: &mut R,id: &[u8; 4]) -> AudioResult<(i64, usize)> {
  let mut buffer = [0u8; 8];
  let mut offset = 0i64; // offset from position reader was at when this function was called

  let mut chunk_size;
  loop {
    let num_bytes = try!(reader.read(&mut buffer));
    if num_bytes == 0 {
      // TODO: Simplify
      return Err(AudioError::from(
                 Error::new(ErrorKind::Other, "Chunk not found")));
    }

    chunk_size = BigEndian::read_i32(&buffer[4..8]) as i64;
    // IFF chunks must be of even size
    if chunk_size % 2 == 1 {
      chunk_size += 1;
    }

    if &buffer[0..4] == id {
      break;
    }

    try!(reader.seek(SeekFrom::Current(chunk_size)));
    offset += 8 + chunk_size;
  }

  // Move to start of found chunk header 
  // try!(reader.seek(SeekFrom::Current(-8)));
  Ok((offset, chunk_size as usize))
}

/// Reads the data within the COMM chunk from the given reader.
///
/// This function assumes the COMM chunk has already been found and its header
/// read using `find_chunk(reader, COMM)` and should use the chunk_size returned
/// by that function.
fn read_comm_chunk<R: Read + Seek>(reader: &mut R, chunk_size: usize) -> AudioResult<chunks::CommonChunk> {
  let mut chunk_buffer = vec![0u8; chunk_size as usize];
  try!(reader.read(&mut chunk_buffer));

  let compression_type =
    if chunk_buffer.len() > 18 {
      match &chunk_buffer[18..22] {
        tag if tag == NONE.0  => CompressionType::Pcm,
        tag if tag == RAW.0  => CompressionType::Raw,
        tag if tag == FL32.0
            || tag == b"FL32" => CompressionType::Float32,
        tag if tag == FL64.0
            || tag == b"FL64" => CompressionType::Float64,
        tag if tag == ALAW.0  => CompressionType::ALaw,
        tag if tag == ULAW.0  => CompressionType::MuLaw,
        _ => {
          return Err(AudioError::Unsupported(
            "Unknown compression type".to_string()
          ))
        }
      }
    }
    else {
      CompressionType::Pcm
    };
  Ok(
    chunks::CommonChunk {
      compression_type: compression_type,
      num_channels:     BigEndian::read_i16(&chunk_buffer[0..2]),
      num_frames:       BigEndian::read_u32(&chunk_buffer[2..6]),
      bit_depth:        BigEndian::read_i16(&chunk_buffer[6..8]),
      sample_rate:      chunks::convert_from_ieee_extended(&chunk_buffer[8..18])
    }
  )
}

/// Returns the `Codec` used by the read audio attributes.
fn determine_codec(compression_type: CompressionType, bit_depth: i16) -> AudioResult<Codec> {
  match (compression_type, bit_depth) {
    // AIFC supports:
    (CompressionType::Raw,     8 ) => Ok(LPCM_U8),
    (CompressionType::ALaw,    16) => Ok(G711_ALAW),
    (CompressionType::MuLaw,   16) => Ok(G711_ULAW),
    (CompressionType::Float32, 32) => Ok(LPCM_F32_BE),
    (CompressionType::Float64, 64) => Ok(LPCM_F64_BE),
    // AIFF supports:
    (CompressionType::Pcm, 8 ) => Ok(LPCM_I8),
    (CompressionType::Pcm, 16) => Ok(LPCM_I16_BE),
    (CompressionType::Pcm, 24) => Ok(LPCM_I24_BE),
    (CompressionType::Pcm, 32) => Ok(LPCM_I32_BE),
    ( _ , _ ) => return
      Err(AudioError::Unsupported(
        "Audio encoded with unsupported codec".to_string()
      ))
  }
}

/// Reads the data within the SSND chunk from the given reader.
///
/// This function assumes the SSND chunk has already been found and its header
/// read using `find_chunk(reader, SSND)` and should use the chunk_size returned
/// by that function. The audio data within the chunk is decoded using the give
/// codec, which can be determined using
/// `determine_codec(compression_type, bit_depth)`.
///
/// The current implementaiton skips the first 8 bytes of the data inside the
/// SSND chunk, which contains the sample offset and block_size, as that
/// information is not used in decoding.
fn read_ssnd_chunk<R: Read + Seek>(reader: &mut R, chunk_size: usize, codec: Codec) -> AudioResult<Vec<Sample>> {
  // let mut buffer = [0u8; 8];
  // try!(reader.read(&mut buffer));
  // let offset      : u32 = BigEndian::read_u32(&chunk_bytes[0..4]);
  // let block_size  : u32 = BigEndian::read_u32(&chunk_bytes[4..8]);
  try!(reader.seek(SeekFrom::Current(8)));

  let mut data_buffer = vec![0u8; chunk_size - 8];
  try!(reader.read(&mut data_buffer));
  read_codec(&data_buffer, codec)
}

/// Returns samples read using the given codec. If the container does not
/// support a codec, an error is returned.
fn read_codec(bytes: &[u8], codec: Codec) -> AudioResult<Vec<Sample>> {
  match is_supported(codec) {
    Ok(_)  => ::codecs::decode(bytes, codec),
    Err(e) => Err(e)
  }
}

/// Determines if codec is supported by the AIFF format.
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

/// Decodes the data within a valid AIFF format and returns an `AudioBuffer`.
///
/// This function searches for the IFF header and the chunks relevant to
/// decoding the stored audio data. All other chunks are ignored.
pub fn decode<R: Read + Seek>(reader: &mut R) -> AudioResult<AudioBuffer> {
  // TODO: Verify that IFF header may start later in file
  // 1.1 Find IFF header
  try!(find_iff_header(reader));

  // 1.2 Validate container
  let (variant, _) = try!(validate_container(reader));

  // 1.3 If Aifc, immediate read FVER chunk or return Err
  if variant == FormatVariant::Aifc {
    try!(find_aifc_fver(reader));
  }

  // 2.1 Find comm chunk, if not found return Err
  let (_, comm_chunk_size) = try!(find_chunk(reader, COMM));

  // 2.2 Read comm chunk
  let comm_chunk  = try!(read_comm_chunk(reader, comm_chunk_size));

  // 2.3 Determine data codec
  let codec = try!(determine_codec(
                   comm_chunk.compression_type,
                   comm_chunk.bit_depth));

  // 3.1 Seek ssnd chunk, if not found return Err
  let (_, ssnd_chunk_size) = try!(find_chunk(reader, SSND));

  // 3.2 Read ssnd chunk
  let samples = try!(read_ssnd_chunk(reader,ssnd_chunk_size, codec));

  Ok(AudioBuffer::from_samples(
    comm_chunk.sample_rate as u32,
    comm_chunk.num_channels as u32,
    samples
  ))
}

// ----------------------------------------------------------
// Encoding
// ----------------------------------------------------------

/// Returns samples as bytes created using the given codec. If the container
/// does not support a codec, an error is returned.
fn write_codec(audio: &AudioBuffer, codec: Codec) -> AudioResult<Vec<u8>> {
  match is_supported(codec) {
    Ok(_)  => ::codecs::encode(audio, codec),
    Err(e) => Err(e)
  }
}

/// Writes an `AudioBuffer` to the given writer in the AIFF format as 16-bit
/// LPCM audio.
///
/// 
pub fn encode<W: Write>(writer: &mut W, audio: &AudioBuffer) -> AudioResult<()> {
  encode_as(writer, audio, Codec::LPCM_I16_BE)
}

/// Writes an `AudioBuffer` to the given writer in the AIFF format using the
/// provided codec.
///
/// If the given codec is not supported by the AIFF format, then an `AudioError`
/// is returned.
pub fn encode_as<W: Write>(writer: &mut W, audio: &AudioBuffer, codec: Codec) -> AudioResult<()> {
  // Determine if codec is supported by container and if it's supported by
  // aiff or aiff-c.
  let aifc: bool    = try!(is_aifc(codec));
  // println!("{:?}", aifc);

  // Encode audio samples using codec.
  let data: Vec<u8> = try!(write_codec(audio, codec));

  // Calculate file_size
  let comm_chunk_size: u32 = try!(chunks::CommonChunk::calculate_size(codec)) as u32;
  let ssnd_chunk_size: u32 = 8 + data.len() as u32;
  // Ignoring 8 bytes to write FORM and file_size
  let mut file_size: u32  = 4 + (8 + comm_chunk_size)
                                 + (8 + ssnd_chunk_size);
  // AIFF-C files must include a format version chunk.
  if aifc {
    file_size += 12;
  }

  // Write the IFF header
  try!(writer.write(FORM));
  try!(writer.write_u32::<BigEndian>(file_size));
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

#[cfg(test)]
mod io {
  mod aiff {
    use std::fs::File;
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use ::audio;
    use ::codecs::Codec::*;

    #[test]
    fn i8_eq() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      let files = vec![
        "mono440-i8-44100.aiff",
        "stereo440-i8-44100.aiff"
      ];

      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i8.aiff");
        assert!(audio::save_as(&write_path, &audio, LPCM_I8).is_ok());

        let verify = audio::open(&write_path).unwrap();
        assert_eq!(audio.channels,      verify.channels);
        assert_eq!(audio.sample_rate,   verify.sample_rate);
        assert_eq!(audio.samples.len(), verify.samples.len());
        for (inital_sample, written_sample) in
            audio.samples.iter().zip(&verify.samples) {
          assert_eq!(inital_sample, written_sample);
        }

        // Assert every byte is the same between the two files.
        let read_file = File::open(path.as_path()).unwrap();
        let written_file = File::open(&write_path).unwrap();
        for (inital_byte, written_byte) in
            read_file.bytes().zip(written_file.bytes()) {
          assert_eq!(inital_byte.ok(), written_byte.ok());
        }
      }
    }

    #[test]
    fn i16_eq() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      let files = vec![
        "mono440-i16-44100.aiff",
        "stereo440-i16-44100.aiff"
      ];
      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i16.aiff");
        assert!(audio::save(&write_path, &audio).is_ok());

        let verify = audio::open(&write_path).unwrap();
        assert_eq!(audio.channels,      verify.channels);
        assert_eq!(audio.sample_rate,   verify.sample_rate);
        assert_eq!(audio.samples.len(), verify.samples.len());
        for (inital_sample, written_sample) in
            audio.samples.iter().zip(&verify.samples) {
          assert_eq!(inital_sample, written_sample);
        }

        // Assert every byte is the same between the two files.
        let read_file = File::open(path.as_path()).unwrap();
        let written_file = File::open(&write_path).unwrap();
        for (inital_byte, written_byte) in
            read_file.bytes().zip(written_file.bytes()) {
          assert_eq!(inital_byte.ok(), written_byte.ok());
        }
      }
    }

    #[test]
    fn i24_eq() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      let files = vec![
        "mono440-i24-44100.aiff",
        "stereo440-i24-44100.aiff"
      ];
      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i24.aiff");
        assert!(audio::save_as(&write_path, &audio, LPCM_I24_BE).is_ok());

        let verify = audio::open(&write_path).unwrap();
        assert_eq!(audio.channels,      verify.channels);
        assert_eq!(audio.sample_rate,   verify.sample_rate);
        assert_eq!(audio.samples.len(), verify.samples.len());
        for (inital_sample, written_sample) in
            audio.samples.iter().zip(&verify.samples) {
          assert_eq!(inital_sample, written_sample);
        }

        // Assert every byte is the same between the two files.
        let read_file = File::open(path.as_path()).unwrap();
        let written_file = File::open(&write_path).unwrap();
        for (inital_byte, written_byte) in
            read_file.bytes().zip(written_file.bytes()) {
          assert_eq!(inital_byte.ok(), written_byte.ok());
        }
      }
    }

    #[test]
    fn i32_eq() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      let files = vec![
        "mono440-i32-44100.aiff",
        "stereo440-i32-44100.aiff"
      ];
      for file in files.iter() {
        path.set_file_name(file);
        println!("{:?}", path.as_path());
        let audio = audio::open(path.as_path()).unwrap();

        let write_path = Path::new("tests/results/tmp_i32.aiff");
        assert!(audio::save_as(&write_path, &audio, LPCM_I32_BE).is_ok());

        let verify = audio::open(&write_path).unwrap();
        assert_eq!(audio.channels,      verify.channels);
        assert_eq!(audio.sample_rate,   verify.sample_rate);
        assert_eq!(audio.samples.len(), verify.samples.len());
        for (inital_sample, written_sample) in
            audio.samples.iter().zip(&verify.samples) {
          assert_eq!(inital_sample, written_sample);
        }

        // Assert every byte is the same between the two files.
        let read_file = File::open(path.as_path()).unwrap();
        let written_file = File::open(&write_path).unwrap();
        for (inital_byte, written_byte) in
            read_file.bytes().zip(written_file.bytes()) {
          assert_eq!(inital_byte.ok(), written_byte.ok());
        }
      }
    }
  }
  mod aifc {
    use std::fs::File;
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use ::audio;
    use ::codecs::Codec::*;

    #[test]
    fn read_aifc_format() {
      let aiff_path = Path::new("tests/aiff/M1F1-int16-AFsp.aif");
      let aifc_path = Path::new("tests/aiff/M1F1-int16C-AFsp.aif");
      // Compare aifc to aiff file with same data.
      let aiff = audio::open(&aiff_path).unwrap();
      let aifc = audio::open(&aifc_path).unwrap();
      assert_eq!(aiff.channels,      aifc.channels);
      assert_eq!(aiff.sample_rate,   aifc.sample_rate);
      assert_eq!(aiff.samples.len(), aifc.samples.len());
      for (aiff_sample, aifc_sample) in
          aiff.samples.iter().zip(&aifc.samples) {
        assert_eq!(aiff_sample, aifc_sample);
      }
    }

    #[test]
    fn u8_eq() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      path.set_file_name("mono440-u8-odd-bytes.aiff");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_u8.aiff");
      assert!(audio::save_as(&write_path, &audio, LPCM_U8).is_ok());

      let verify = audio::open(&write_path).unwrap();
      assert_eq!(audio.channels,      verify.channels);
      assert_eq!(audio.sample_rate,   verify.sample_rate);
      assert_eq!(audio.samples.len(), verify.samples.len());
      for (inital_sample, written_sample) in
          audio.samples.iter().zip(&verify.samples) {
        assert_eq!(inital_sample, written_sample);
      }

      // Assert every byte is the same between the two files.
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_path).unwrap();
      for (inital_byte, written_byte) in
          read_file.bytes().zip(written_file.bytes()) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }

    #[test]
    fn f32() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      path.set_file_name("M1F1-float32C-AFsp.aif");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_f32.aiff");
      assert!(audio::save_as(&write_path, &audio, LPCM_F32_BE).is_ok());

      let verify = audio::open(&write_path).unwrap();
      assert_eq!(audio.channels,      verify.channels);
      assert_eq!(audio.sample_rate,   verify.sample_rate);
      assert_eq!(audio.samples.len(), verify.samples.len());
      for (inital_sample, written_sample) in
          audio.samples.iter().zip(&verify.samples) {
        assert_eq!(inital_sample, written_sample);
      }

      // Assert every byte, excluding metadata, is the same between the two files.
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_path).unwrap();
      for (inital_byte, written_byte) in
          read_file.bytes().skip(154)
          .zip(written_file.bytes().skip(72)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }

    #[test]
    fn f64() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      path.set_file_name("M1F1-float64C-AFsp.aif");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_f64.aiff");
      assert!(audio::save_as(&write_path, &audio, LPCM_F64_BE).is_ok());

      let verify = audio::open(&write_path).unwrap();
      assert_eq!(audio.channels,      verify.channels);
      assert_eq!(audio.sample_rate,   verify.sample_rate);
      assert_eq!(audio.samples.len(), verify.samples.len());
      for (inital_sample, written_sample) in
          audio.samples.iter().zip(&verify.samples) {
        assert_eq!(inital_sample, written_sample);
      }

      // Assert every byte, excluding metadata, is the same between the two files.
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_path).unwrap();
      for (inital_byte, written_byte) in
          read_file.bytes().skip(154)
          .zip(written_file.bytes().skip(72)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }

    #[test]
    fn ulaw() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      path.set_file_name("M1F1-mulawC-AFsp.aif");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_ulaw.aiff");
      assert!(audio::save_as(&write_path, &audio, G711_ULAW).is_ok());

      let verify = audio::open(&write_path).unwrap();
      assert_eq!(audio.channels,      verify.channels);
      assert_eq!(audio.sample_rate,   verify.sample_rate);
      assert_eq!(audio.samples.len(), verify.samples.len());
      for (inital_sample, written_sample) in
          audio.samples.iter().zip(&verify.samples) {
        assert_eq!(inital_sample, written_sample);
      }

      // Assert every byte, excluding metadata, is the same between the two files.
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_path).unwrap();
      for (inital_byte, written_byte) in
          read_file.bytes().skip(146)
          .zip(written_file.bytes().skip(64)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }

    #[test]
    fn alaw() {
      let mut path = PathBuf::from("tests");
      path.push("aiff");
      path.push("empty.aiff");
      path.set_file_name("M1F1-AlawC-AFsp.aif");
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).unwrap();

      let write_path = Path::new("tests/results/tmp_alaw.aiff");
      assert!(audio::save_as(&write_path, &audio, G711_ALAW).is_ok());

      let verify = audio::open(&write_path).unwrap();
      assert_eq!(audio.channels,      verify.channels);
      assert_eq!(audio.sample_rate,   verify.sample_rate);
      assert_eq!(audio.samples.len(), verify.samples.len());
      for (inital_sample, written_sample) in
          audio.samples.iter().zip(&verify.samples) {
        assert_eq!(inital_sample, written_sample);
      }

      // Assert every byte, excluding metadata, is the same between the two files.
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_path).unwrap();
      for (inital_byte, written_byte) in
          read_file.bytes().skip(146)
          .zip(written_file.bytes().skip(64)) {
        assert_eq!(inital_byte.ok(), written_byte.ok());
      }
    }
  }
}
