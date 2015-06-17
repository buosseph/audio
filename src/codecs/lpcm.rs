use buffer::*;
use byteorder::{ByteOrder, BigEndian};
use codecs::AudioCodec;
use error::{AudioResult, AudioError};

#[allow(dead_code)]
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
  #[allow(unused_assignments)]
  fn read(bytes: &mut Vec<u8>, bit_rate: &u32, channels: &u32) -> AudioResult<Vec<Sample>> {
    // All bytes passed to this codec must be in big-endian format
    let block_size = (bit_rate / 8u32 * channels) as usize;
    let sample_size = (bit_rate / 8u32) as usize;
    let num_of_frames: usize = bytes.len() / block_size;
    let num_of_samples: usize = bytes.len() / sample_size;
    debug_assert_eq!(num_of_samples, num_of_frames * *channels as usize);
    let mut samples: Vec<f64> = Vec::with_capacity(num_of_samples);
    let mut i;
    match *bit_rate as usize {
      8   => {
        for sample in bytes.iter() {
           samples.push((*sample as f64 - 128f64) / 128f64);
        }
      },
      16  => {
        let mut sample: i16 = 0i16;
        let range: f64 = i16::max_value() as f64 + 1f64;
        for sample_bytes in bytes.chunks(sample_size) {
          sample = BigEndian::read_i16(sample_bytes);
          samples.push(sample as f64 / range);
          sample = sample ^ sample; // clear sample value
        }
      },
      24  => {
        let mut sample: i32 = 0i32;
        let range: f64 = 8388608f64;
        for sample_bytes in bytes.chunks(sample_size) {
          i = 0;
          for byte in sample_bytes.iter() {
            sample = sample | (*byte as i32) << ((sample_bytes.len() - i - 1) * 8);
            i += 1;
          }
          samples.push(sample as f64 / range);
          sample = sample ^ sample; // clear sample value
        }
      },
      32  => {
        let mut sample: i32 = 0i32;
        let range: f64 = i32::max_value() as f64 + 1f64;
        for sample_bytes in bytes.chunks(sample_size) {
          sample = BigEndian::read_i32(sample_bytes);
          samples.push(sample as f64 / range);
          sample = sample ^ sample; // clear sample value
        }
      },
      _   => return Err(AudioError::UnsupportedError(format!("Cannot read {}-bit LPCM", bit_rate)))
    }
    debug_assert_eq!(num_of_samples, samples.len());
    debug_assert_eq!(samples.capacity(), num_of_samples);
    debug_assert!( if num_of_frames != 0 { samples.len() != 0 } else { samples.len() == 0 });
    Ok(samples)
  }
  fn create(audio: &AudioBuffer) -> AudioResult<Vec<u8>> {
    /*
     *  TODO: Dealing with bit rates not supported by format.
     *
     *  Audio can be manipulated to be of any bit rate, but codecs don't
     *  support that. It's preferred that the fields in AudioBuffer
     *  reflect the samples stored, so values need to be checked when
     *  encoding. For bit rate, just round up to nearest multiple of 8
     *  that's less than the highest supported bit rate and use that
     *  value for the encoding bit rate.
     */

    // Only encoding as 16-bit PCM for now
    let bit_rate = 16;
    let sample_size = bit_rate / 8;
    let num_of_bytes = audio.samples.len() * sample_size;
    let mut buffer: Vec<u8> = Vec::with_capacity(num_of_bytes);
    for _ in 0..buffer.capacity() { buffer.push(0u8); }
    let mut sample: f64;
    let mut i = 0;
    // if sample_size == 2 (16-bit)
    for sample_bytes in buffer.chunks_mut(sample_size) {
      sample = audio.samples[i] * 32768f64;
      if sample > 32768f64 {
        sample = 32768f64;
      }
      else if sample < -32768f64 {
        sample = -32768f64;
      }
      BigEndian::write_i16(sample_bytes, sample as i16);
      i += 1;
    }
    debug_assert_eq!(num_of_bytes, i * sample_size);
    debug_assert_eq!(num_of_bytes / sample_size, audio.samples.len());
    debug_assert_eq!(num_of_bytes, buffer.len());
    Ok(buffer)
  }
}

#[test]
fn test_bytes_to_sample() {
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