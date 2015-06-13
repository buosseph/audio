use buffer::*;
use codecs::{Codec, AudioCodec};
use error::{AudioResult, AudioError};

pub struct LPCM;

impl AudioCodec for LPCM{
  fn read(bytes: &mut Vec<u8>) -> AudioResult<Vec<Sample>> {
    // Assuming bytes is in little-endian format from WAV

    //let block_size = (bit_rate / 8) * channels;
    //let num_of_frames: usize = bytes.len() / block_size;

    /*
    let num_of_frames: uint = data.size as uint / fmt.block_size as uint ;
    let mut samples: Vec<f64> = Vec::with_capacity(num_of_frames * fmt.num_of_channels as uint);
    let mut sample_order: SampleOrder;

    let mut frame: &[u8];
    match fmt.compression_code {
      PCM => {

        // This is the main code we're intereseted in

        match fmt.bit_rate {
          8 => {
            match (fmt.num_of_channels, fmt.block_size) {
              (2, 2) => {
                sample_order = INTERLEAVED;
                for i in range(0, num_of_frames) {
                  frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];

                  let left_sample : u8  = frame[0];
                  let right_sample: u8  = frame[1];
                
                  let float_left  : f64   = (left_sample as f64 - 128f64) / 128f64;
                  let float_right : f64   = (right_sample as f64 - 128f64) / 128f64;

                  samples.push(float_left);
                  samples.push(float_right);
                }
              },

              (1, 1) => {
                sample_order = MONO;
                for i in range(0, num_of_frames) {
                  frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];
                  let sample : u8     = frame[0];
                  let float_sample : f64  = (sample as f64 - 128f64) / 128f64;
                  samples.push(float_sample);
                }
              },

              (_, _) => {
                return Err(AudioError::UnsupportedError(
                  format!(
                  "Cannot read {}-channel .wav files.",
                  fmt.num_of_channels)
                ))
              }
            }
          },

          16 => {
            match (fmt.num_of_channels, fmt.block_size) {
              (2, 4) => {
                sample_order = INTERLEAVED;
                for i in range(0, num_of_frames) {
                  frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];

                  let left_sample : i16   = (frame[1] as i16) << 8 | frame[0] as i16;
                  let right_sample: i16   = (frame[3] as i16) << 8 | frame[2] as i16;
                
                  let float_left  : f64   = left_sample as f64 / 32768f64;
                  let float_right : f64   = right_sample as f64 / 32768f64;

                  samples.push(float_left);
                  samples.push(float_right);
                }
              },

              (1, 2) => {
                sample_order = MONO;
                for i in range(0, num_of_frames) {
                  frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];
                  let sample : i16    = (frame[1] as i16) << 8 | frame[0] as i16;
                  let float_sample : f64  = sample as f64 / 32768f64;
                  samples.push(float_sample);
                }
              },

              (_, _) => {
                return Err(AudioError::UnsupportedError(
                  format!(
                  "Cannot read {}-channel .wav files.",
                  fmt.num_of_channels)
                ))
              }
            }
          },

          24 => {
            match (fmt.num_of_channels, fmt.block_size) {
              (2, 6) => {
                sample_order = INTERLEAVED;
                for i in range(0, num_of_frames) {
                  frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];

                  let left_sample : i32   = (frame[2] as i32) << 16 | (frame[1] as i32) << 8 | frame[0] as i32;
                  let right_sample: i32   = (frame[5] as i32) << 16 | (frame[4] as i32) << 8 | frame[3] as i32;
                
                  let float_left  : f64   = left_sample as f64 / 8388608f64;
                  let float_right : f64   = right_sample as f64 / 8388608f64;

                  samples.push(float_left);
                  samples.push(float_right);
                }
              },

              (1, 3) => {
                sample_order = MONO;
                for i in range(0, num_of_frames) {
                  frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];
                  let sample : i32    = (frame[2] as i32) << 16 | (frame[1] as i32) << 8 | frame[0] as i32;
                  let float_sample : f64  = sample as f64 / 8388608f64;
                  samples.push(float_sample);
                }
              },

              (_, _) => {
                return Err(AudioError::UnsupportedError(
                  format!(
                  "Cannot read {}-channel .wav files.",
                  fmt.num_of_channels)
                ))
              }
            }
          },

          32 => {
            match (fmt.num_of_channels, fmt.block_size) {
              (2, 8) => {
                sample_order = INTERLEAVED;
                for i in range(0, num_of_frames) {
                  frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];

                  let left_sample : i32   = (frame[3] as i32) << 24 | (frame[2] as i32) << 16 | (frame[1] as i32) << 8 | frame[0] as i32;
                  let right_sample: i32   = (frame[7] as i32) << 24 | (frame[6] as i32) << 16 | (frame[5] as i32) << 8 | frame[4] as i32;
                
                  let float_left  : f64   = left_sample as f64 / 2147483648f64;
                  let float_right : f64   = right_sample as f64 / 2147483648f64;

                  samples.push(float_left);
                  samples.push(float_right);
                }
              },

              (1, 4) => {
                sample_order = MONO;
                for i in range(0, num_of_frames) {
                  frame = &data.data[i * fmt.block_size as uint .. i * fmt.block_size as uint + fmt.block_size as uint];
                  let sample : i32    = (frame[3] as i32) << 24 | (frame[2] as i32) << 16 | (frame[1] as i32) << 8 | frame[0] as i32;
                  let float_sample : f64  = sample as f64 / 2147483648f64;
                  samples.push(float_sample);
                }
              },

              (_, _) => {
                return Err(AudioError::UnsupportedError(
                  format!(
                  "Cannot read {}-channel .wav files.",
                  fmt.num_of_channels)
                ))
              }
            }
          },

          _ => {
            return Err(AudioError::UnsupportedError(
              format!(
              "Cannot read {}-bit .wav files.",
              fmt.bit_rate)
            ))
          }
        }



      },
      _ => {
        return Err(AudioError::UnsupportedError(
          "Can only read PCM encoded .wav files.".to_string()
        ))
      }
    }
    */


    Err(AudioError::UnsupportedError("This codec is not yet supported".to_string()))
  }
}