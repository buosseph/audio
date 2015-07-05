use buffer::*;
use codecs::{Endian, AudioCodec};
use error::{AudioResult, AudioError};

#[allow(dead_code)]
pub struct LPCM;

impl AudioCodec for LPCM {
  fn read(bytes: &mut Vec<u8>, endian: Endian, bit_rate: &u32, channels: &u32) -> AudioResult<Vec<Sample>> {
    let le =
      match endian {
        Endian::LittleEndian => true,
        Endian::BigEndian    => false
      };
    let block_size  : usize     = (bit_rate / 8u32 * channels) as usize;
    let sample_size : usize     = (bit_rate / 8u32) as usize;
    let num_frames  : usize     = bytes.len() / block_size;
    let num_samples : usize     = bytes.len() / sample_size;
    let mut samples : Vec<f64>  = Vec::with_capacity(num_samples);
    match *bit_rate as usize {
      8   => {
        for sample in bytes.iter() {
           samples.push((*sample as f64 - 128f64) / 128f64);
        }
      },
      16  => {
        let mut sample: i16 = 0i16;
        let offset: isize =
          if le {
            0
          } else {
            (sample_size - 1) as isize
          };
        for i in 0..num_samples {
          for j in 0..sample_size {
            sample |=
              (bytes[i * sample_size + j] as i16)
              << (8 * (-1 * j as isize + offset).abs() as usize);
          }
          samples.push(sample as f64 / 32_768f64);
          sample = 0;
        }
      },
      24  => {
        let mut sample: i32 = 0i32;
        let offset: isize =
          if le {
            0
          } else {
            (sample_size - 1) as isize
          };
        for i in 0..num_samples {
          for j in 0..sample_size {
            sample |=
              (bytes[i * sample_size + j] as i32)
              << (8 * (-1 * j as isize + offset).abs() as usize);
          }
          samples.push(sample as f64 / 16_777_216f64);
          sample = 0;
        }
      },
      32  => {
        let mut sample: i32 = 0i32;
        let offset: isize =
          if le {
            0
          } else {
            (sample_size - 1) as isize
          };
        for i in 0..num_samples {
          for j in 0..sample_size {
            sample |=
              (bytes[i * sample_size + j] as i32)
              << (8 * (-1 * j as isize + offset).abs() as usize);
          }
          samples.push(sample as f64 / 2_147_483_648f64);
          sample = 0;
        }
      },
      _   => 
        return Err(AudioError::UnsupportedError(
          format!("Cannot read {}-bit LPCM", bit_rate)
        ))
    }
    debug_assert_eq!(num_samples, samples.len());
    debug_assert_eq!(samples.capacity(), num_samples);
    debug_assert!( if num_frames != 0 { samples.len() != 0 } else { samples.len() == 0 });
    Ok(samples)
  }

  fn create(audio: &AudioBuffer, endian: Endian) -> AudioResult<Vec<u8>> {
    // Only support 8, 16, 24, 32 bit endcoding
    let le =
      match endian {
        Endian::LittleEndian => true,
        Endian::BigEndian    => false
      };
    let bit_rate = audio.bit_rate as usize;
    let sample_size = bit_rate / 8;
    let num_samples = audio.samples.len();
    let num_bytes = num_samples * sample_size;
    let mut buffer: Vec<u8> = Vec::with_capacity(num_bytes);
    let mut sample: f64;
    match bit_rate {
      8   => {
        for i in 0..num_samples {
          buffer.push((audio.samples[i] * 128f64 + 128f64) as u8);
        }
      },
      16  => {
        let mut write_sample: i16;
        for i in 0..num_samples {
          sample = audio.samples[i] * 32768f64;
          if sample > 32768f64 {
            sample = 32768f64;
          }
          else if sample < -32768f64 {
            sample = -32768f64;
          }
          write_sample = 
            if le {
              (sample as i16).swap_bytes()
            } else {
              sample as i16
            };
          for i in 0..sample_size {
            buffer.push(
              (write_sample
              >> (8 * (sample_size - 1 - i))) as u8
            );
          }
        }
      },
      24  => {
        let mut write_sample: i32;
        for i in 0..num_samples {
          sample = audio.samples[i] * 16_777_216f64;
          if sample > 16_777_216f64 {
            sample = 16_777_216f64;
          }
          else if sample < -16_777_216f64 {
            sample = -16_777_216f64;
          }
          write_sample = 
            if le {
              (sample as i32)
                .swap_bytes()
                .rotate_right(8u32)
            } else {
              sample as i32
            };
          for i in 0..sample_size {
            buffer.push(
              (write_sample
              >> (8 * (sample_size - 1 - i))) as u8
            );
          }
        }
      },
      32  => {
        let mut write_sample: i32;
        for i in 0..num_samples {
          sample = audio.samples[i] * 2_147_483_648f64;
          if sample > 2_147_483_648f64 {
            sample = 2_147_483_648f64;
          }
          else if sample < -2_147_483_648f64 {
            sample = -2_147_483_648f64;
          }
          write_sample = 
            if le {
              (sample as i32).swap_bytes()
            } else {
              sample as i32
            };
          for i in 0..sample_size {
            buffer.push(
              (write_sample
              >> (8 * (sample_size - 1 - i))) as u8
            );
          }
        }
      },
      b @ _ => return Err(AudioError::UnsupportedError(
        format!("Can't encode {}-bit LPCM", b)
      ))
    }
    debug_assert_eq!(num_bytes, buffer.len());
    Ok(buffer)
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