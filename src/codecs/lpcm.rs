use std::mem;
use buffer::*;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use codecs::{AudioCodec, Endian};
use error::AudioResult;
use super::SampleFormat;
use super::SampleFormat::*;

#[allow(dead_code)]
pub struct LPCM;

impl AudioCodec for LPCM {
  fn read(bytes: &[u8], sample_format: SampleFormat, endian: Endian) -> AudioResult<Vec<Sample>> {
    let le =
      match endian {
        Endian::LittleEndian => true,
        Endian::BigEndian    => false
      };
    let bit_rate    : usize     =
      match sample_format {
        SampleFormat::Unsigned8 => 8,
        SampleFormat::Signed8   => 8,
        SampleFormat::Signed16  => 16,
        SampleFormat::Signed24  => 24,
        SampleFormat::Signed32  => 32
      };
    let num_samples : usize     = bytes.len() / (bit_rate / 8);
    let mut samples : Vec<f64>  = vec![0f64; num_samples];
    if num_samples != 0 {
      match sample_format {
        SampleFormat::Unsigned8 => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = (bytes[i] as f64 - 128f64) / 128f64;
          }
        },
        SampleFormat::Signed8 => {
          for (i, sample) in samples.iter_mut().enumerate() {
            *sample = bytes[i] as f64 / 128f64;
          }
        },
        SampleFormat::Signed16 => {
          if le {
            for (i, sample) in samples.iter_mut().enumerate() {
              *sample = (LittleEndian::read_i16(&bytes[2 * i .. 2 * i + 2])) as f64 / 32_768f64;
            }
          }
          else {
            for (i, sample) in samples.iter_mut().enumerate() {
              *sample = (BigEndian::read_i16(&bytes[2 * i .. 2 * i + 2])) as f64 / 32_768f64;
            }
          }
        },
        SampleFormat::Signed24 => {
          if le {
            for (i, sample) in samples.iter_mut().enumerate() {
              *sample =
                (((bytes[3 * i + 2] as i32) << 16) | ((bytes[3 * i + 1] as i32) << 8) | (bytes[3 * i] as i32)) as f64
                / 16_777_216f64;
            }
          }
          else {
            for (i, sample) in samples.iter_mut().enumerate() {
              *sample =
                (((bytes[3 * i] as i32) << 16) | ((bytes[3 * i + 1] as i32) << 8) | (bytes[3 * i + 2] as i32)) as f64
                / 16_777_216f64;
            }
          }
        },
        SampleFormat::Signed32 => {
          if le {
            for (i, sample) in samples.iter_mut().enumerate() {
              *sample = (LittleEndian::read_i32(&bytes[4 * i .. 4 * i + 4])) as f64 / 2_147_483_648f64;
            }
          }
          else {
            for (i, sample) in samples.iter_mut().enumerate() {
              *sample = (BigEndian::read_i32(&bytes[4 * i .. 4 * i + 4])) as f64 / 2_147_483_648f64;
            }
          }
        },
      }
    }
    Ok(samples)
  }

  fn create(audio: &AudioBuffer, sample_format: SampleFormat, endian: Endian) -> AudioResult<Vec<u8>> {
    let le =
      match endian {
        Endian::LittleEndian => true,
        Endian::BigEndian    => false
      };    
    let num_bits_per_sample = 
      match sample_format {
        Unsigned8| Signed8  => 8,
        Signed16            => 16,
        Signed24            => 24,
        Signed32            => 32
      };
    let num_bytes = audio.samples.len() * (num_bits_per_sample / 8);
    let mut bytes = vec![0u8; num_bytes];
    if num_bytes != 0 {
      match sample_format {
        Unsigned8 => {
          for (i, sample) in audio.samples.iter().enumerate() {
            bytes[i] = (sample * 128f64 + 128f64) as u8;
          }
        },
        Signed8   => {
          for (i, sample) in audio.samples.iter().enumerate() {
            bytes[i] = unsafe { mem::transmute_copy(&((sample * 128f64) as i8)) };
          }
        },
        Signed16  => {
          if le {
            for (i, sample) in audio.samples.iter().enumerate() {
              LittleEndian::write_i16(&mut bytes[2 * i .. 2 * i + 2], (sample * 32_768f64) as i16);
            }
          }
          else {
            for (i, sample) in audio.samples.iter().enumerate() {
              BigEndian::write_i16(&mut bytes[2 * i .. 2 * i + 2], (sample * 32_768f64) as i16);
            }
          }
        },
        Signed24  => {
          if le {
            for (i, sample) in audio.samples.iter().enumerate() {
              let tmp = (sample * 16_777_216f64) as i32;
              bytes[3 * i + 2] = (tmp >> 16) as u8;
              bytes[3 * i + 1] = (tmp >> 8)  as u8;
              bytes[3 * i]     =  tmp        as u8;
            }
          }
          else {
            for (i, sample) in audio.samples.iter().enumerate() {
              let tmp = (sample * 16_777_216f64) as i32;
              bytes[3 * i]     = (tmp >> 16) as u8;
              bytes[3 * i + 1] = (tmp >> 8)  as u8;
              bytes[3 * i + 2] =  tmp        as u8;
            }
          }
        },
        Signed32  => {
          if le {
            for (i, sample) in audio.samples.iter().enumerate() {
              LittleEndian::write_i32(&mut bytes[4 * i .. 4 * i + 4], (sample * 2_147_483_648f64) as i32);
            }
          }
          else {
            for (i, sample) in audio.samples.iter().enumerate() {
              BigEndian::write_i32(&mut bytes[4 * i .. 4 * i + 4], (sample * 2_147_483_648f64) as i32);
            }
          }
        },
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