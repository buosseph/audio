use std::fmt;
use buffer::*;
use codecs::{Codec, AudioCodec};
use error::{AudioResult, AudioError};

pub struct LPCM;

/*
enum PcmSample {
  8Bit(u8),       // range = 128
  16Bit(i16),     // range = 32768
  24Bit(i32),     // range = 8388608
  32Bit(i32)      // range = 2147483648
}
*/

impl AudioCodec for LPCM{
  fn read(bytes: &mut Vec<u8>, bit_rate: &u32, channels: &u32) -> AudioResult<Vec<Sample>> {
    // Assuming bytes is in little-endian format from WAV

    /*
    16 (2, 4)=>
    for sample_bytes in bytes.chunks(block_size / *channels as usize) {
      let sample: i16 = 0i16;
      for byte in sample_bytes {
        
      }
      let sample = sample_bytes.swap_bytes();
      samples.push((sample as f64 - 128f64) / 128f64);
    }
    */
    let block_size = (bit_rate / 8u32 * channels) as usize;
    let num_of_frames: usize = bytes.len() / block_size;
    let mut samples: Vec<f64> = Vec::with_capacity(num_of_frames * *channels as usize);
    match *bit_rate as usize {
      8   => {
        for sample in bytes.iter() {
           samples.push((*sample as f64 - 128f64) / 128f64);
        }
      },
      16  => {
        let mut sample: i16 = 0i16;
        let range: f64 = i16::max_value() as f64 + 1f64;
        for sample_bytes in bytes.chunks(block_size / *channels as usize) {
          for byte in sample_bytes.iter() {
            sample << 8;
            sample = sample | *byte as i16;
          }
          sample.swap_bytes();  // convert to big endian
          samples.push(sample as f64 / range);
          sample = sample ^ sample; // clear sample value
        }
      },
      24  => {
        let mut sample: i32 = 0i32;
        let range: f64 = 8388608f64;
        for sample_bytes in bytes.chunks(block_size / *channels as usize) {
          for byte in sample_bytes.iter() {
            sample << 8;
            sample = sample | *byte as i32;
          }
          sample.swap_bytes();  // convert to big endian
          samples.push(sample as f64 / range);
          sample = sample ^ sample; // clear sample value
        }
      },
      32  => {
        let mut sample: i32 = 0i32;
        let range: f64 = i32::max_value() as f64 + 1f64;
        for sample_bytes in bytes.chunks(block_size / *channels as usize) {
          for byte in sample_bytes.iter() {
            sample << 8;
            sample = sample | *byte as i32;
          }
          sample.swap_bytes();  // convert to big endian
          samples.push(sample as f64 / range);
          sample = sample ^ sample; // clear sample value
        }
      },
      _   => return Err(AudioError::UnsupportedError(format!("Cannot read {}-bit LPCM", bit_rate)))
    }
    Ok(samples)
  }
}