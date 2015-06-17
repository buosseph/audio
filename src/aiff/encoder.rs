use std::io::Write;
use audio::AudioEncoder;
use buffer::*;
use codecs::Codec;
use traits::Container;
use aiff::chunks::AiffContainer;
use error::{AudioResult, AudioError};

pub struct Encoder<'w, W: 'w> {
  writer: &'w mut W,
}

impl<'w, W> Encoder<'w, W> where W: Write {
  pub fn new(writer: &'w mut W) -> Encoder<'w, W> {
    Encoder {
      writer: writer
    }
  }
}

impl<'w, W> AudioEncoder for Encoder<'w, W> where W: Write {
  fn encode(&mut self, audio: &AudioBuffer) -> AudioResult<()> {
    // Codec must be passed to container to determine if it's supported
    //let buffer: Vec<u8> = try!(AiffContainer::create(Codec::LPCM, audio));
    //try!(self.writer.write_all(&buffer));
    Err(AudioError::UnsupportedError("This feature is not yet complete".to_string()))
  }
}

/*
#[allow(deprecated)]
pub fn write_file(raw_audio: &RawAudio, path: &Path) -> AudioResult<bool> {
	match raw_audio.order {
		MONO		=> {},
		INTERLEAVED => {},
		_			=> return Err(AudioError::FormatError("AIFF requires multi-channel audio to be interleaved".to_string()))
	}

	let mut file = File::create(path);
	let block_size: uint = raw_audio.channels * raw_audio.bit_rate / 8;
		// This is not the block_size written to file, needed to determine correct data_size

	let num_of_channels		: u16 		= raw_audio.channels as u16;
	let sampling_rate		: uint 		= raw_audio.sample_rate;
	let bit_rate			: u16 		= raw_audio.bit_rate as u16;
	let num_of_frames		: u32 		= (raw_audio.samples.len() / raw_audio.channels) as u32;
	let sample_rate_buffer	: Vec<u8> 	= convert_to_ieee_extended(sampling_rate);
	let comm_chunk_size 	: u32 		= 18;	// COMM chunk always 18 since we're not adding padding

	let offset			: u32 	= 0;
	let aiff_block_size	: u32 	= 0;
	let data_size		: u32 	= num_of_frames * block_size as u32;
	let ssnd_chunk_size	: u32 	= 8 + data_size;

	let total_bytes 					= 12 + (comm_chunk_size + 8) + (ssnd_chunk_size + 8);	// = [FORM: 12] + [COMM: 26] + [SSND: 8 + chunk_size]
	let file_size			: u32 		= total_bytes - 8;
	let mut buffer 			: Vec<u8> 	= Vec::with_capacity(total_bytes as uint);


	buffer.push_all(&i32_to_be_slice(FORM));
	buffer.push_all(&u32_to_be_slice(file_size));
	buffer.push_all(&i32_to_be_slice(AIFF));

	buffer.push_all(&i32_to_be_slice(COMM));
	buffer.push_all(&u32_to_be_slice(comm_chunk_size));
	buffer.push_all(&u16_to_be_slice(num_of_channels));
	buffer.push_all(&u32_to_be_slice(num_of_frames));
	buffer.push_all(&u16_to_be_slice(bit_rate));
	buffer.push_all(sample_rate_buffer.as_slice());

	buffer.push_all(&i32_to_be_slice(SSND));
	buffer.push_all(&u32_to_be_slice(ssnd_chunk_size));
	buffer.push_all(&u32_to_be_slice(offset));
	buffer.push_all(&u32_to_be_slice(aiff_block_size));

	for sample in raw_audio.samples.iter() {
		let mut pcm_sample = *sample * 32768f64;

		if pcm_sample > 32768f64 {
			pcm_sample = 32768f64;
		}
		if pcm_sample < -32768f64 {
			pcm_sample = -32768f64;
		}

		buffer.push_all(&i16_to_be_slice(pcm_sample as i16));
	}

	try!(file.write_all(buffer.as_slice()));

	Ok(true)
}
*/