//! LPCM
//!
//! Linear pulse code modulation

use std::mem;
use byteorder::*;
use codecs::Codec;
use codecs::Codec::*;
use error::*;
use sample::*;

pub fn write(samples: &[Sample], codec: Codec) -> AudioResult<Vec<u8>> {
  let num_bytes = samples.len() * (codec.bit_depth() / 8);
  let mut bytes = vec![0u8; num_bytes];

  if num_bytes != 0 {
    match codec {
      LPCM_U8     => {
        for (i, sample) in samples.iter().enumerate() {
          bytes[i] = u8::from_sample(*sample);
        }
      },

      LPCM_I8     => {
        for (i, sample) in samples.iter().enumerate() {
          bytes[i] = unsafe {
            mem::transmute_copy(&(i8::from_sample(*sample)))
          };
        }
      },

      LPCM_I16_LE => {
        for (i, sample) in samples.iter().enumerate() {
          LittleEndian::write_i16(&mut bytes[2 * i .. 2 * i + 2],
                                  i16::from_sample(*sample));
        }
      },

      LPCM_I16_BE => {
        for (i, sample) in samples.iter().enumerate() {
          BigEndian::write_i16(&mut bytes[2 * i .. 2 * i + 2],
                               i16::from_sample(*sample));
        }
      },

      LPCM_I24_LE => {
        for (i, sample) in samples.iter().enumerate() {
          let tmp_f32     = sample * 8_388_608f32;
          let mut integer = tmp_f32 as i32;

          if tmp_f32 > 8_388_607f32 {
            integer = 8_388_607i32
          }
          else if tmp_f32 < -8_388_608f32 {
            integer = -8_388_608i32
          }

          // Handle for sign
          if integer & 0x800000 != 0 {
            integer |= !0xffffff;
          }

          bytes[3 * i + 2] = (integer >> 16) as u8;
          bytes[3 * i + 1] = (integer >> 8)  as u8;
          bytes[3 * i]     =  integer        as u8;
        }
      },

      LPCM_I24_BE => {
        for (i, sample) in samples.iter().enumerate() {
          let tmp_f32     = sample * 8_388_608f32;
          let mut integer = tmp_f32 as i32;

          if tmp_f32 > 8_388_607f32 {
            integer = 8_388_607i32
          }
          else if tmp_f32 < -8_388_608f32 {
            integer = -8_388_608i32
          }

          // Handle for sign
          if (integer & 0x800000) >> 23 == 1 {
            integer |= !0xffffff;
          }

          bytes[3 * i]     = (integer >> 16) as u8;
          bytes[3 * i + 1] = (integer >> 8)  as u8;
          bytes[3 * i + 2] =  integer        as u8;
        }
      },

      LPCM_I32_LE => {
        for (i, sample) in samples.iter().enumerate() {
          LittleEndian::write_i32(&mut bytes[4 * i .. 4 * i + 4],
                                  i32::from_sample(*sample));
        }
      },

      LPCM_I32_BE => {
        for (i, sample) in samples.iter().enumerate() {
          BigEndian::write_i32(&mut bytes[4 * i .. 4 * i + 4],
                               i32::from_sample(*sample));
        }
      },

      LPCM_F32_LE => {
        for (i, sample) in samples.iter().enumerate() {
          LittleEndian::write_f32(&mut bytes[4 * i .. 4 * i + 4],
                                  f32::from_sample(*sample));
        }
      },

      LPCM_F32_BE => {
        for (i, sample) in samples.iter().enumerate() {
          BigEndian::write_f32(&mut bytes[4 * i .. 4 * i + 4],
                               f32::from_sample(*sample));
        }
      },

      LPCM_F64_LE => {
        for (i, sample) in samples.iter().enumerate() {
          LittleEndian::write_f64(&mut bytes[8 * i .. 8 * i + 8],
                                  f64::from_sample(*sample));
        }
      },

      LPCM_F64_BE => {
        for (i, sample) in samples.iter().enumerate() {
          BigEndian::write_f64(&mut bytes[8 * i .. 8 * i + 8],
                               f64::from_sample(*sample));
        }
      },

      c => {
        return Err(AudioError::Unsupported(
          format!("Unsupported codec {} was passed into the LPCM decoder", c)
        ))
      }
    }
  }

  Ok(bytes)
}

pub fn read_sample(bytes: &[u8], codec: Codec) -> AudioResult<Sample> {
  let required_num_bytes = codec.bit_depth() / 8;

  if bytes.len() != required_num_bytes {
    return Err(AudioError::Unsupported(
      "Missing some bytes for sample decode".to_string()))
  }

  let sample =
    match codec {
      LPCM_U8 => {
        bytes[0].to_sample()
      },

      LPCM_I8 => {
        (bytes[0] as i8).to_sample()
      },

      LPCM_I16_LE => {
        LittleEndian::read_i16(&bytes[0..2]).to_sample()
      },

      LPCM_I16_BE => {
        BigEndian::read_i16(&bytes[0..2]).to_sample()
      },

      LPCM_I24_LE => {
        let mut tmp_i32 = 0;
        tmp_i32 |= (bytes[2] as i32) << 16;
        tmp_i32 |= (bytes[1] as i32) << 8;
        tmp_i32 |=  bytes[0] as i32;

        // Handle for sign
        if (tmp_i32 & 0x800000) >> 23 == 1 {
          tmp_i32 |= !0xffffff;
        }

        tmp_i32 as Sample / 8_388_608f32
      },

      LPCM_I24_BE => {
        let mut tmp_i32 = 0;
        tmp_i32 |= (bytes[0] as i32) << 16;
        tmp_i32 |= (bytes[1] as i32) << 8;
        tmp_i32 |=  bytes[2] as i32;

        // Handle for sign
        if (tmp_i32 & 0x800000) >> 23 == 1 {
          tmp_i32 |= !0xffffff;
        }

        tmp_i32 as Sample / 8_388_608f32
      },

      LPCM_I32_LE => {
        LittleEndian::read_i32(&bytes[0..4]).to_sample()
      },

      LPCM_I32_BE => {
        BigEndian::read_i32(&bytes[0..4]).to_sample()
      },

      LPCM_F32_LE => {
        LittleEndian::read_f32(&bytes[0..4]).to_sample()
      },

      LPCM_F32_BE => {
        BigEndian::read_f32(&bytes[0..4]).to_sample()
      },

      LPCM_F64_LE => {
        LittleEndian::read_f64(&bytes[0..8]).to_sample()
      },

      LPCM_F64_BE => {
        BigEndian::read_f64(&bytes[0..8]).to_sample()
      },

      _ => unimplemented!()
    };

  Ok(sample)
}

#[cfg(test)]
mod coding {
  mod encode {
    use ::buffer::*;
    use byteorder::*;
    use ::codecs::Codec::*;
    use ::codecs::lpcm;

    #[test]
    fn unsupported_codec() {
      let audio = AudioBuffer::from_samples(44100, 1, vec![0f32; 4]);
      let codecs =
        vec![
          G711_ALAW,
          G711_ULAW
        ];
      for unsupported_codec in codecs.iter() {
        assert!(lpcm::write(&audio.samples, *unsupported_codec).is_err());
      }
    }

    #[test]
    fn to_u8() {
      let samples = vec![0f32, 1f32, -1f32];
      let audio = AudioBuffer::from_samples(44100, 1, samples);
      if let Ok(bytes) = lpcm::write(&audio.samples, LPCM_U8) {
        assert_eq!(128, bytes[0]);
        // 1.0 is mapped to 254
        assert_eq!(u8::max_value(), bytes[1]);
        assert_eq!(u8::min_value(), bytes[2]);
      }
    }

    #[test]
    fn to_i8() {
      let samples = vec![0f32, 1f32, -1f32];
      let audio = AudioBuffer::from_samples(44100, 1, samples);
      if let Ok(bytes) = lpcm::write(&audio.samples, LPCM_I8) {
        assert_eq!(0, bytes[0] as i8);
        assert_eq!(i8::max_value(), bytes[1] as i8);
        assert_eq!(i8::min_value(), bytes[2] as i8);
      }
    }

    #[test]
    fn to_i16_le() {
      let samples = vec![0f32, 1f32, -1f32];
      let audio = AudioBuffer::from_samples(44100, 1, samples);
      if let Ok(bytes) = lpcm::write(&audio.samples, LPCM_I16_LE) {
        assert_eq!(0, LittleEndian::read_i16(&bytes[0..2]));
        assert_eq!(i16::max_value(), LittleEndian::read_i16(&bytes[2..4]));
        assert_eq!(i16::min_value(), LittleEndian::read_i16(&bytes[4..6]));
      }
    }

    #[test]
    fn to_i16_be() {
      let samples = vec![0f32, 1f32, -1f32];
      let audio = AudioBuffer::from_samples(44100, 1, samples);
      if let Ok(bytes) = lpcm::write(&audio.samples, LPCM_I16_BE) {
        assert_eq!(0, BigEndian::read_i16(&bytes[0..2]));
        assert_eq!(i16::max_value(), BigEndian::read_i16(&bytes[2..4]));
        assert_eq!(i16::min_value(), BigEndian::read_i16(&bytes[4..6]));
      }
    }

    #[test]
    fn to_i24_le() {
      let samples = vec![0f32, 1f32, -1f32];
      let audio = AudioBuffer::from_samples(44100, 1, samples);
      if let Ok(bytes) = lpcm::write(&audio.samples, LPCM_I24_LE) {
        assert_eq!(0, bytes[0]);
        assert_eq!(0, bytes[1]);
        assert_eq!(0, bytes[2]);
        assert_eq!(0xff, bytes[3]);
        assert_eq!(0xff, bytes[4]);
        assert_eq!(0x7f, bytes[5]);
        assert_eq!(0x00, bytes[6]);
        assert_eq!(0x00, bytes[7]);
        assert_eq!(0x80, bytes[8]);
      }
    }

    #[test]
    fn to_i24_be() {
      let samples = vec![0f32, 1f32, -1f32];
      let audio = AudioBuffer::from_samples(44100, 1, samples);
      if let Ok(bytes) = lpcm::write(&audio.samples, LPCM_I24_BE) {
        assert_eq!(0, bytes[0]);
        assert_eq!(0, bytes[1]);
        assert_eq!(0, bytes[2]);
        assert_eq!(0x7f, bytes[3]);
        assert_eq!(0xff, bytes[4]);
        assert_eq!(0xff, bytes[5]);
        assert_eq!(0x80, bytes[6]);
        assert_eq!(0x00, bytes[7]);
        assert_eq!(0x00, bytes[8]);
      }
    }

    // Test i32 values are different because f32 can't represent the extereme values.
    #[test]
    fn to_i32_le() {
      let samples = vec![0f32, 1f32, -1f32];
      let audio = AudioBuffer::from_samples(44100, 1, samples);
      if let Ok(bytes) = lpcm::write(&audio.samples, LPCM_I32_LE) {
        assert_eq!(0, LittleEndian::read_i32(&bytes[0..4]));
        assert_eq!((i32::min_value() + 128).abs(), LittleEndian::read_i32(&bytes[4..8]) );
        assert_eq!(i32::min_value() + 128, LittleEndian::read_i32(&bytes[8..12]) );
      }
    }

    #[test]
    fn to_i32_be() {
      println!("i32::max {:?}", i32::max_value());
      println!("i32::max with headroom {:?}", 2_147_483_647i32 - 128);
      let samples = vec![0f32, 1f32, -1f32];
      let audio = AudioBuffer::from_samples(44100, 1, samples);
      if let Ok(bytes) = lpcm::write(&audio.samples, LPCM_I32_BE) {
        assert_eq!(0, BigEndian::read_i32(&bytes[0..4]));
        assert_eq!((i32::min_value() + 128).abs(), BigEndian::read_i32(&bytes[4..8]) );
        assert_eq!(i32::min_value() + 128, BigEndian::read_i32(&bytes[8..12]) );
      }
    }
  }

  mod decode {
    use ::codecs::Codec::*;
    use ::codecs::lpcm;

    #[test]
    fn unsupported_codec() {
      let bytes = vec![0u8; 4];
      let codecs =
        vec![
          G711_ALAW,
          G711_ULAW
        ];

      for unsupported_codec in codecs.iter() {
        assert!(lpcm::read_sample(&bytes, *unsupported_codec).is_err());
      }
    }

    #[test]
    fn from_u8() {
      let codec   = LPCM_U8;
      let read    = lpcm::read_sample;
      let samples =
        vec![
          [128u8],
          [u8::max_value()],
          [u8::min_value()]
        ];

      assert_eq!(0f32, read(&samples[0], codec).unwrap());
      assert!(( 1f32 - read(&samples[1], codec).unwrap()).abs() < 1e-2f32);
      assert!((-1f32 - read(&samples[2], codec).unwrap()).abs() < 1e-2f32);
    }

    #[test]
    fn from_i8() {
      let codec   = LPCM_I8;
      let read    = lpcm::read_sample;
      let samples =
        vec![
          [0u8],
          [i8::max_value() as u8],
          [i8::min_value() as u8]
        ];

      assert_eq!(0f32, read(&samples[0], codec).unwrap());
      assert!(( 1f32 - read(&samples[1], codec).unwrap()).abs() < 1e-2f32);
      assert!((-1f32 - read(&samples[2], codec).unwrap()).abs() < 1e-2f32);
    }

    #[test]
    fn from_i16_le() {
      let codec   = LPCM_I16_LE;
      let read    = lpcm::read_sample;
      let samples =
        vec![
          [0u8, 0x00],
          [0xff, 0x7f],
          [0x00, 0x80]
        ];

      assert_eq!(0f32, read(&samples[0], codec).unwrap());
      assert!(( 1f32 - read(&samples[1], codec).unwrap()).abs() < 1e-2f32);
      assert!((-1f32 - read(&samples[2], codec).unwrap()).abs() < 1e-2f32);
    }

    #[test]
    fn from_i16_be() {
      let codec   = LPCM_I16_BE;
      let read    = lpcm::read_sample;
      let samples =
        vec![
          [0u8, 0x00],
          [0x7f, 0xff],
          [0x80, 0x00]
        ];

      assert_eq!(0f32, read(&samples[0], codec).unwrap());
      assert!(( 1f32 - read(&samples[1], codec).unwrap()).abs() < 1e-2f32);
      assert!((-1f32 - read(&samples[2], codec).unwrap()).abs() < 1e-2f32);
    }

    #[test]
    fn from_i24_le() {
      let codec   = LPCM_I24_LE;
      let read    = lpcm::read_sample;
      let samples =
        vec![
          [0u8, 0x00, 0x00],
          [0xff, 0xff, 0x7f],
          [0x00, 0x00, 0x80]
        ];

      assert_eq!(0f32, read(&samples[0], codec).unwrap());
      assert!(( 1f32 - read(&samples[1], codec).unwrap()).abs() < 1e-2f32);
      assert!((-1f32 - read(&samples[2], codec).unwrap()).abs() < 1e-2f32);
    }

    #[test]
    fn from_i24_be() {
      let codec   = LPCM_I24_BE;
      let read    = lpcm::read_sample;
      let samples =
        vec![
          [0u8, 0x00, 0x00],
          [0x7f, 0xff, 0xff],
          [0x80, 0x00, 0x00]
        ];

      assert_eq!(0f32, read(&samples[0], codec).unwrap());
      assert!(( 1f32 - read(&samples[1], codec).unwrap()).abs() < 1e-2f32);
      assert!((-1f32 - read(&samples[2], codec).unwrap()).abs() < 1e-2f32);
    }

    #[test]
    fn from_i32_le() {
      let codec   = LPCM_I32_LE;
      let read    = lpcm::read_sample;
      let samples =
        vec![
          [0u8, 0x00, 0x00, 0x00],
          [0xff, 0xff, 0xff, 0x7f],
          [0x00, 0x00, 0x00, 0x80]
        ];

      assert_eq!(0f32, read(&samples[0], codec).unwrap());
      assert!(( 1f32 - read(&samples[1], codec).unwrap()).abs() < 1e-2f32);
      assert!((-1f32 - read(&samples[2], codec).unwrap()).abs() < 1e-2f32);
    }

    #[test]
    fn from_i32_be() {
      let codec   = LPCM_I32_BE;
      let read    = lpcm::read_sample;
      let samples =
        vec![
          [0u8, 0x00, 0x00, 0x00],
          [0x7f, 0xff, 0xff, 0xff],
          [0x80, 0x00, 0x00, 0x00]
        ];

      assert_eq!(0f32, read(&samples[0], codec).unwrap());
      assert!(( 1f32 - read(&samples[1], codec).unwrap()).abs() < 1e-2f32);
      assert!((-1f32 - read(&samples[2], codec).unwrap()).abs() < 1e-2f32);
    }
  }
}
