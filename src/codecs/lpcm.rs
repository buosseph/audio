use std::mem;
use buffer::*;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use codecs::{AudioCodec, Codec};
use codecs::Codec::*;
use codecs::g711::*;
use error::AudioResult;

// TODO: Create and use macros for lpcm encoding and decoding based on these
// C++ macros from [FFmpeg](https://github.com/FFmpeg/FFmpeg/blob/master/libavcodec/pcm.c#L65):
//
// /**
//  * Write PCM samples macro
//  * @param type   Datatype of native machine format
//  * @param endian bytestream_put_xxx() suffix
//  * @param src    Source pointer (variable name)
//  * @param dst    Destination pointer (variable name)
//  * @param n      Total number of samples (variable name)
//  * @param shift  Bitshift (bits)
//  * @param offset Sample value offset
//  */
// #define ENCODE(type, endian, src, dst, n, shift, offset)                \
//     samples_ ## type = (const type *) src;                              \
//     for (; n > 0; n--) {                                                \
//         register type v = (*samples_ ## type++ >> shift) + offset;      \
//         bytestream_put_ ## endian(&dst, v);                             \
//     }
//
// /**
//  * Read PCM samples macro
//  * @param size   Data size of native machine format
//  * @param endian bytestream_get_xxx() endian suffix
//  * @param src    Source pointer (variable name)
//  * @param dst    Destination pointer (variable name)
//  * @param n      Total number of samples (variable name)
//  * @param shift  Bitshift (bits)
//  * @param offset Sample value offset
//  */
// #define DECODE(size, endian, src, dst, n, shift, offset)                \
//     for (; n > 0; n--) {                                                \
//         uint ## size ## _t v = bytestream_get_ ## endian(&src);         \
//         AV_WN ## size ## A(dst, (v - offset) << shift);                 \
//         dst += size / 8;                                                \
//     }

// TODO: Write macro for clipping samples based on number of bits

#[allow(dead_code)]
pub struct LPCM;

impl AudioCodec for LPCM {
  fn read(bytes: &[u8], codec: Codec) -> AudioResult<Vec<Sample>> {
    let bit_rate    : usize     =
      match codec {
        LPCM_U8     |
        LPCM_I8     |
        // ALaw and ULaw are decompressed to 16 bits per sample
        LPCM_ALAW   |
        LPCM_ULAW   => 8,
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
      };
    let num_samples : usize     = bytes.len() / (bit_rate / 8);
    let mut samples : Vec<Sample>  = vec![0f32; num_samples];
    if num_samples != 0 {
      match codec {
        LPCM_U8     => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = bytes[i].to_sample();
          }
        },
        LPCM_I8     => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = (bytes[i] as i8).to_sample();
          }
        },
        LPCM_ALAW   => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = alaw_to_linear(bytes[i]).to_sample();
          }
        },
        LPCM_ULAW   => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = ulaw_to_linear(bytes[i]).to_sample();
          }
        },
        LPCM_I16_LE => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = LittleEndian::read_i16(&bytes[2 * i .. 2 * i + 2]).to_sample();
          }
        },
        LPCM_I16_BE => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = BigEndian::read_i16(&bytes[2 * i .. 2 * i + 2]).to_sample();
          }
        },
        LPCM_I24_LE => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample =
              (((bytes[3 * i + 2] as i32) << 16) | ((bytes[3 * i + 1] as i32) << 8) | (bytes[3 * i] as i32)) as Sample
              / 16_777_216f32;
          }
        },
        LPCM_I24_BE => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample =
              (((bytes[3 * i] as i32) << 16) | ((bytes[3 * i + 1] as i32) << 8) | (bytes[3 * i + 2] as i32)) as Sample
              / 16_777_216f32;
          }
        },
        LPCM_I32_LE => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = LittleEndian::read_i32(&bytes[4 * i .. 4 * i + 4]).to_sample();
          }
        },
        LPCM_I32_BE => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = BigEndian::read_i32(&bytes[4 * i .. 4 * i + 4]).to_sample();
          }
        },
        LPCM_F32_LE => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = LittleEndian::read_f32(&bytes[4 * i .. 4 * i + 4]).to_sample();
          }
        },
        LPCM_F32_BE => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = BigEndian::read_f32(&bytes[4 * i .. 4 * i + 4]).to_sample();
          }
        },
        LPCM_F64_LE => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = LittleEndian::read_f64(&bytes[8 * i .. 8 * i + 8]).to_sample();
          }
        },
        LPCM_F64_BE => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = BigEndian::read_f64(&bytes[8 * i .. 8 * i + 8]).to_sample();
          }
        }
      }
    }
    Ok(samples)
  }
  fn create(audio: &AudioBuffer, codec: Codec) -> AudioResult<Vec<u8>> {
    let num_bits_per_sample =
      match codec {
        LPCM_U8     |
        LPCM_I8     |
        LPCM_ALAW   |
        LPCM_ULAW   => 8,
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
      };
    let num_bytes = audio.samples.len() * (num_bits_per_sample / 8);
    let mut bytes = vec![0u8; num_bytes];
    if num_bytes != 0 {
      match codec {
        LPCM_U8     => {
          for (i, sample) in audio.samples.iter().enumerate() {
            bytes[i] = u8::from_sample(*sample);
          }
        },
        LPCM_I8     => {
          for (i, sample) in audio.samples.iter().enumerate() {
            bytes[i] = unsafe { mem::transmute_copy(&(i8::from_sample(*sample))) };
          }
        },
        LPCM_ALAW   => {
          for (i, sample) in audio.samples.iter().enumerate() {
            bytes[i] = linear_to_alaw(i16::from_sample(*sample));
          }
        },
        LPCM_ULAW   => {
          for (i, sample) in audio.samples.iter().enumerate() {
            bytes[i] = linear_to_ulaw(i16::from_sample(*sample));
          }
        },
        LPCM_I16_LE => {
          for (i, sample) in audio.samples.iter().enumerate() {
            LittleEndian::write_i16(&mut bytes[2 * i .. 2 * i + 2], i16::from_sample(*sample));
          }
        },
        LPCM_I16_BE => {
          for (i, sample) in audio.samples.iter().enumerate() {
            BigEndian::write_i16(&mut bytes[2 * i .. 2 * i + 2], i16::from_sample(*sample));
          }
        },
        LPCM_I24_LE => {
          for (i, sample) in audio.samples.iter().enumerate() {
            let tmp = (sample * 16_777_216f32) as i32;
            bytes[3 * i + 2] = (tmp >> 16) as u8;
            bytes[3 * i + 1] = (tmp >> 8)  as u8;
            bytes[3 * i]     =  tmp        as u8;
          }
        },
        LPCM_I24_BE => {
          for (i, sample) in audio.samples.iter().enumerate() {
            let tmp = (sample * 16_777_216f32) as i32;
            bytes[3 * i]     = (tmp >> 16) as u8;
            bytes[3 * i + 1] = (tmp >> 8)  as u8;
            bytes[3 * i + 2] =  tmp        as u8;
          }
        },
        LPCM_I32_LE => {
          for (i, sample) in audio.samples.iter().enumerate() {
            LittleEndian::write_i32(&mut bytes[4 * i .. 4 * i + 4], i32::from_sample(*sample));
          }
        },
        LPCM_I32_BE => {
          for (i, sample) in audio.samples.iter().enumerate() {
            BigEndian::write_i32(&mut bytes[4 * i .. 4 * i + 4], i32::from_sample(*sample));
          }
        },
        LPCM_F32_LE => {
          for (i, sample) in audio.samples.iter().enumerate() {
            LittleEndian::write_f32(&mut bytes[4 * i .. 4 * i + 4], f32::from_sample(*sample));
          }
        },
        LPCM_F32_BE => {
          for (i, sample) in audio.samples.iter().enumerate() {
            BigEndian::write_f32(&mut bytes[4 * i .. 4 * i + 4], f32::from_sample(*sample));
          }
        },
        LPCM_F64_LE => {
          for (i, sample) in audio.samples.iter().enumerate() {
            LittleEndian::write_f64(&mut bytes[8 * i .. 8 * i + 8], f64::from_sample(*sample));
          }
        },
        LPCM_F64_BE => {
          for (i, sample) in audio.samples.iter().enumerate() {
            BigEndian::write_f64(&mut bytes[8 * i .. 8 * i + 8], f64::from_sample(*sample));
          }
        }
      }
    }
    Ok(bytes)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn bytes_to_sample() {
    let bytes1: &[u8] = &[0x51, 0xB0];
    let mut sample1: i16;
    sample1 = (bytes1[1] as i16) << 8 |  bytes1[0] as i16;
    assert_eq!(-20399i16, sample1);
    assert_eq!(20912i16, sample1.swap_bytes());
    sample1 = sample1 ^ sample1;
    assert_eq!(0i16, sample1);

    println!("");
    let mut i = 0;
    for byte in bytes1.iter() {
      sample1 = sample1 | (*byte as i16) << ((bytes1.len() - i - 1) * 8);
      i += 1;
      println!("{:?} ({:x})", sample1, sample1);
    }
    assert_eq!(20912i16, sample1);
    assert_eq!(-20399i16, sample1.swap_bytes());
    sample1 = sample1 ^ sample1;
    assert_eq!(0i16, sample1);

    let bytes2: &[u8] = &[0xE5, 0xA8, 0x6D];
    let mut sample2: i32 = 0i32;
    i = 0;
    println!("");
    for byte in bytes2.iter() {
      sample2 = sample2 | (*byte as i32) << ((bytes2.len() - i - 1) * 8);
      i += 1;
      println!("{:?} ({:x})", sample2, sample2);
    }
    assert_eq!(15050861i32, sample2);
    assert_eq!(1839785216i32, sample2.swap_bytes());
    sample2 = sample2 ^ sample2;
    assert_eq!(0i32, sample2);

    let bytes3: &[u8] = &[0x9D, 0x25, 0x81, 0x2B];
    let mut sample3: i32 = 0i32;
    i = 0;
    println!("");
    for byte in bytes3.iter() {
      sample3 = sample3 | (*byte as i32) << ((bytes3.len() - i - 1) * 8);
      i += 1;
      println!("{:?} ({:x})", sample3, sample3);
    }
    assert_eq!(-1658486485i32, sample3);
    assert_eq!(729884061i32, sample3.swap_bytes());
    sample3 = sample3 ^ sample3;
    assert_eq!(0i32, sample3);
  }
}