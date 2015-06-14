use std::io::{Write};
use audio::{AudioEncoder};
use buffer::*;
use containers::*;
use error::AudioResult;

pub struct Encoder<'w, W: 'w> {
  writer: &'w mut W,
}

impl<'w, W> Encoder<'w, W> where W: Write {
  pub fn new(w: &'w mut W, audio: &AudioBuffer) -> Encoder<'w, W> {
    Encoder {
      writer: w,
    }
  }
}

impl<'w, W> AudioEncoder for Encoder<'w, W> where W: Write {
  fn encode(self) -> AudioResult<bool> {
    Ok(false)
  }
}

/*
use audio::{
	AudioResult,
	AudioError,
	RawAudio
};
use audio::SampleOrder::{MONO, INTERLEAVED};
use std::old_io::{File};
use super::{RIFF, WAVE, FMT, DATA};

#[allow(deprecated)]
pub fn write_file(raw_audio: &RawAudio, path: &Path) -> AudioResult<bool> {
	match raw_audio.order {
		MONO		=> {},
		INTERLEAVED => {},
		_			=> return Err(AudioError::UnsupportedError("Multi-channel audio must be interleaved for encoding".to_string()))
	}

	let mut file 	= File::create(path);

	let num_of_channels	: u16 		= raw_audio.channels as u16;
	let sampling_rate	: u32 		= raw_audio.sample_rate as u32;
	let data_rate		: u32 		= (raw_audio.sample_rate * raw_audio.channels * raw_audio.bit_rate / 8) as u32;
	let bit_rate		: u16 		= 16; //= raw_audio.bit_rate as u16;
	let block_size		: uint 		= raw_audio.channels * raw_audio.bit_rate / 8;
	let fmt_chunk_size	: u32 		= 16;

	let num_of_frames	: uint		= raw_audio.samples.len() / raw_audio.channels;
	let data_size		: u32 		= (num_of_frames * block_size) as u32;

	let total_bytes					= 44 + data_size;	// Always write 44 byte header
	let file_size		: u32 		=  (total_bytes - 8) as u32;
		// = 4 + (8 + fmt_chunk size) + (8 + (data_chunk size * block_size)) (NOTE: 8 bytes are purposely missing for riff_header and file_size)
		// = 4 + (WAVE chunks) = total_bytes - 8 (exclude first 8 bytes)
	let mut buffer 			: Vec<u8> 	= Vec::with_capacity(total_bytes as uint);


	buffer.push_all(&u32_to_le_slice(RIFF));
	buffer.push_all(&u32_to_le_slice(file_size));
	buffer.push_all(&u32_to_le_slice(WAVE));

	buffer.push_all(&u32_to_le_slice(FMT));
	buffer.push_all(&u32_to_le_slice(fmt_chunk_size));
	buffer.push_all(&u16_to_le_slice(1 as u16));		// Always encode as PCM
	buffer.push_all(&u16_to_le_slice(num_of_channels));
	buffer.push_all(&u32_to_le_slice(sampling_rate));
	buffer.push_all(&u32_to_le_slice(data_rate));
	buffer.push_all(&u16_to_le_slice(block_size as u16));
	buffer.push_all(&u16_to_le_slice(bit_rate));

	buffer.push_all(&u32_to_le_slice(DATA));
	buffer.push_all(&u32_to_le_slice(data_size));

	for sample in raw_audio.samples.iter() {
		let mut pcm_sample = *sample * 32768f64;

		if pcm_sample > 32768f64 {
			pcm_sample = 32768f64;
		}
		if pcm_sample < -32768f64 {
			pcm_sample = -32768f64;
		}

		buffer.push_all(&i16_to_le_slice(pcm_sample as i16));
	}

	try!(file.write_all(buffer.as_slice()));

	Ok(true)
}

fn u32_to_le_slice(num: u32) -> [u8; 4] {
	[ num as u8, (num >> 8) as u8, (num >> 16) as u8, (num >> 24) as u8 ]
}

fn u16_to_le_slice(num: u16) -> [u8; 2] {
	[ num as u8, (num >> 8) as u8 ]
}

fn i16_to_le_slice(num: i16) -> [u8; 2] {
	[ num as u8, (num >> 8) as u8 ]
}
*/