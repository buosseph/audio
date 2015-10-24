//! The Audio Interchange File Format
//!
//! AIFF files use the Interchange File Format (IFF), a generic file container
//! format that uses chunks to store data. All bytes are stored in big-endian
//! format.

use std::io::{
  Error,
  ErrorKind,
  Read,
  Seek,
  SeekFrom,
  Write
};

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
use self::chunks::*;

mod chunks;

// AIFF/AIFC chunk identifiers.

/// IFF form group identifier.
const FORM: &'static [u8; 4] = b"FORM";

/// AIFF form type identifier.
const AIFF: &'static [u8; 4] = b"AIFF";

/// AIFF-C form type identifier.
const AIFC: &'static [u8; 4] = b"AIFC";

/// AIFF-C Format Version Chunk identifier.
const FVER: &'static [u8; 4] = b"FVER";

/// AIFF-C Version 1 timestamp for the FVER chunk.
const AIFC_VERSION_1: u32 = 0xA2805140;

/// AIFF Common Cchunk identifier.
const COMM: &'static [u8; 4] = b"COMM";

/// AIFF Sound Data Chunk identifier.
const SSND: &'static [u8; 4] = b"SSND";

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
      return Err(AudioError::from(
                 Error::new(ErrorKind::Other,
                            "Chunk not found")));
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
  try!(find_iff_header(reader));
  let (variant, _) = try!(validate_container(reader));
  if variant == FormatVariant::Aifc {
    try!(find_aifc_fver(reader));
  }

  let (_, comm_chunk_size) = try!(find_chunk(reader, COMM));
  let comm_chunk  = try!(CommonChunk::read_chunk_data(reader,
                                                      comm_chunk_size));

  let codec = try!(determine_codec(
                   comm_chunk.compression_type,
                   comm_chunk.bit_depth));

  let (_, ssnd_chunk_size) = try!(find_chunk(reader, SSND));
  let samples = try!(SoundDataChunk::read_chunk_data(reader, 
                                                     ssnd_chunk_size,
                                                     codec));

  Ok(AudioBuffer::from_samples(
    comm_chunk.sample_rate  as u32,
    comm_chunk.num_channels as u32,
    samples
  ))
}

// ----------------------------------------------------------
// Encoding
// ----------------------------------------------------------

/// Determines if the `Codec` given requires the audio to be encoded as AIFF-C.
///
/// This function also doubles as a check for codec support by this format. If
/// the codec is not supported by this format, or library, then an `AudioError`
/// is returned.
#[inline]
pub fn is_aifc(codec: Codec) -> AudioResult<bool> {
  match codec {
    G711_ALAW   |
    G711_ULAW   |
    LPCM_U8     |
    LPCM_F32_BE |
    LPCM_F64_BE => Ok(true),
    LPCM_I8     |
    LPCM_I16_BE |
    LPCM_I24_BE |
    LPCM_I32_BE => Ok(false),
    c @ _       =>
      return Err(AudioError::Unsupported(
        format!("Aiff does not support {:?} codec", c)
      ))
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
  // Determine if codec is supported by format variants.
  let aifc = try!(is_aifc(codec));
  let data = try!(write_codec(audio, codec));

  // Calculate file_size
  let comm_chk_size: u32 = try!(CommonChunk::calculate_size(codec)) as u32;
  let ssnd_chk_size: u32 = 8 + data.len() as u32;
  // Ignoring 8 bytes to write FORM and file_size
  let mut file_size: u32 = 4 + (8 + comm_chk_size)
                             + (8 + ssnd_chk_size);

  // AIFF-C files must include a format version chunk.
  if aifc {
    file_size += 12;
  }

  // Write the IFF header.
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

  // Write comm chunk.
  try!(CommonChunk::write(writer, audio, codec));

  // Write ssnd chunk.
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
