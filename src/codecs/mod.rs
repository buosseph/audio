use std::fmt;

pub mod lpcm;
pub mod g711;

/// All supported audio codecs.
///
/// Any codec where endianess, sample ordering, or type influence coding is
/// represented with a separate variant. If not specifiec, the codec is for
/// interleaved sample data.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Codec {
  /// Unsigned 8-bit linear PCM
  LPCM_U8,
  /// Signed 8-bit linear PCM
  LPCM_I8,
  /// Signed 16-bit linear PCM in little endian format
  LPCM_I16_LE,
  /// Signed 16-bit linear PCM in big endian format
  LPCM_I16_BE,
  /// Signed 24-bit linear PCM in little endian format
  LPCM_I24_LE,
  /// Signed 24-bit linear PCM in big endian format
  LPCM_I24_BE,
  /// Signed 32-bit linear PCM in little endian format
  LPCM_I32_LE,
  /// Signed 32-bit linear PCM in big endian format
  LPCM_I32_BE,
  /// 32-bit floating-point linear PCM in little endian format
  LPCM_F32_LE,
  /// 32-bit floating-point linear PCM in big endian format
  LPCM_F32_BE,
  /// 64-bit floating-point linear PCM in little endian format
  LPCM_F64_LE,
  /// 64-bit floating-point linear PCM in big endian format
  LPCM_F64_BE,
  /// G.711 8-bit A-law
  G711_ALAW,
  /// G.711 8-bit µ-law
  G711_ULAW
}

impl Codec {
  /// Returns the bit depth of the encoded audio output by a codec.
  pub fn bit_depth(&self) -> usize {
    use Codec::*;

    match *self {
      G711_ALAW   |
      G711_ULAW   |
      LPCM_U8     |
      LPCM_I8     => 8,

      LPCM_I16_LE |
      LPCM_I16_BE => 16,

      LPCM_I24_LE |
      LPCM_I24_BE => 24,

      LPCM_I32_LE |
      LPCM_I32_BE |
      LPCM_F32_LE |
      LPCM_F32_BE => 32,

      LPCM_F64_LE |
      LPCM_F64_BE => 64
    }
  }
}

impl fmt::Display for Codec {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    use Codec::*;

    match self {
      &LPCM_U8     => fmt.write_str("Unsigned 8-bit PCM"),
      &LPCM_I8     => fmt.write_str("Signed 8-bit PCM"),
      &LPCM_I16_LE => fmt.write_str("Signed 16-bit little endian PCM"),
      &LPCM_I16_BE => fmt.write_str("Signed 16-bit big endian PCM"),
      &LPCM_I24_LE => fmt.write_str("Signed 24-bit little endian PCM"),
      &LPCM_I24_BE => fmt.write_str("Signed 24-bit big endian PCM"),
      &LPCM_I32_LE => fmt.write_str("Signed 32-bit little endian PCM"),
      &LPCM_I32_BE => fmt.write_str("Signed 32-bit big endian PCM"),
      &LPCM_F32_LE => fmt.write_str("32-bit little endian floating-point PCM"),
      &LPCM_F32_BE => fmt.write_str("32-bit big endian floating-point PCM"),
      &LPCM_F64_LE => fmt.write_str("64-bit little endian floating-point PCM"),
      &LPCM_F64_BE => fmt.write_str("64-bit big endian floating-point PCM"),
      &G711_ALAW   => fmt.write_str("G.711 8-bit A-law"),
      &G711_ULAW   => fmt.write_str("G.711 8-bit µ-law")
    }
  }
}

#[cfg(test)]
mod formatting {
  #[test]
  fn display() {
    use Codec::*;

    let formatted_strs =
      vec![
        "Unsigned 8-bit PCM",
        "Signed 8-bit PCM",
        "Signed 16-bit little endian PCM",
        "Signed 16-bit big endian PCM",
        "Signed 24-bit little endian PCM",
        "Signed 24-bit big endian PCM",
        "Signed 32-bit little endian PCM",
        "Signed 32-bit big endian PCM",
        "32-bit little endian floating-point PCM",
        "32-bit big endian floating-point PCM",
        "64-bit little endian floating-point PCM",
        "64-bit big endian floating-point PCM",
        "G.711 8-bit A-law",
        "G.711 8-bit µ-law"
      ];
    let codecs =
      vec![
        LPCM_U8,
        LPCM_I8,
        LPCM_I16_LE,
        LPCM_I16_BE,
        LPCM_I24_LE,
        LPCM_I24_BE,
        LPCM_I32_LE,
        LPCM_I32_BE,
        LPCM_F32_LE,
        LPCM_F32_BE,
        LPCM_F64_LE,
        LPCM_F64_BE,
        G711_ALAW,
        G711_ULAW
      ];
    for (expected_str, codec) in formatted_strs.iter().zip(codecs.iter()) {
      assert_eq!(*expected_str, format!("{}", codec));
    }
  }

  #[test]
  fn debug() {
    use Codec::*;

    let debug_strs =
      vec![
        "LPCM_U8",
        "LPCM_I8",
        "LPCM_I16_LE",
        "LPCM_I16_BE",
        "LPCM_I24_LE",
        "LPCM_I24_BE",
        "LPCM_I32_LE",
        "LPCM_I32_BE",
        "LPCM_F32_LE",
        "LPCM_F32_BE",
        "LPCM_F64_LE",
        "LPCM_F64_BE",
        "G711_ALAW",
        "G711_ULAW"
      ];
    let codecs =
      vec![
        LPCM_U8,
        LPCM_I8,
        LPCM_I16_LE,
        LPCM_I16_BE,
        LPCM_I24_LE,
        LPCM_I24_BE,
        LPCM_I32_LE,
        LPCM_I32_BE,
        LPCM_F32_LE,
        LPCM_F32_BE,
        LPCM_F64_LE,
        LPCM_F64_BE,
        G711_ALAW,
        G711_ULAW
      ];
    for (expected_str, codec) in debug_strs.iter().zip(codecs.iter()) {
      assert_eq!(*expected_str, format!("{:?}", codec));
    }
  }
}
