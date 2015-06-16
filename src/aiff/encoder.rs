use std::io::Write;
use audio::AudioEncoder;
use buffer::*;
use codecs::Codec;
use traits::Container;
use aiff::chunks::AiffContainer;
use error::AudioResult;

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
    Ok(())
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

fn convert_to_ieee_extended(sample_rate: uint) -> Vec<u8>{
	if sample_rate == 0 {
		let vec: Vec<u8> = vec![0,0,0,0,0,0,0,0,0,0];
		return vec;
	}

	let mut num 	: f64 = sample_rate as f64;
	let mut exponent: int;
	let mut f_mant 	: f64;
	let mut fs_mant	: f64;
	let mut hi_mant	: u32;
	let mut low_mant: u32;


	let sign: int = match num < 0f64 {
		true => { num *= -1f64; 0x8000 },
		false => { 0x0000 }
	};

	let tuple = Float::frexp(num);
	f_mant = tuple.0;
	exponent = tuple.1;

	if exponent > 16384 || !(f_mant < 1f64) {
		exponent 	= (sign|0x7fff) as int;
		hi_mant 	= 0;
		low_mant 	= 0;
	}
	else {
		exponent += 16382;
		if exponent < 0 {
			f_mant 		= Float::ldexp(f_mant, exponent);
			exponent 	= 0;
		}

		exponent 	|= sign as int;
		f_mant 		= Float::ldexp(f_mant, 32);
		fs_mant 	= Float::floor(f_mant);
		hi_mant 	= fs_mant as u32;
		f_mant 		= Float::ldexp(f_mant - fs_mant, 32);
		fs_mant 	= Float::floor(f_mant);
		low_mant 	= fs_mant as u32;
	}

	let vec: Vec<u8> = vec![
		(	exponent 		>> 8	)	as u8,
		(	exponent				)	as u8,
		(	hi_mant 		>> 24 	) 	as u8,
		(	hi_mant 		>> 16 	) 	as u8,
		(	hi_mant 		>> 8 	) 	as u8,
			hi_mant 					as u8,
		(	low_mant 		>> 24 	) 	as u8,
		(	low_mant 		>> 16 	) 	as u8,
		(	low_mant 		>> 8 	) 	as u8,
			low_mant 					as u8
	];

	return vec;
}


fn u32_to_be_slice(num: u32) -> [u8; 4] {
	[ (num >> 24) as u8, (num >> 16) as u8, (num >> 8) as u8, num as u8 ]
}

fn u16_to_be_slice(num: u16) -> [u8; 2] {
	[ (num >> 8) as u8, num as u8 ]
}

fn i32_to_be_slice(num: i32) -> [u8; 4] {
	[ (num >> 24) as u8, (num >> 16) as u8, (num >> 8) as u8, num as u8 ]
}

fn i16_to_be_slice(num: i16) -> [u8; 2] {
	[ (num >> 8) as u8, num as u8 ]
}

#[cfg(test)]
mod tests {
	use super::convert_to_ieee_extended;

	#[test]
	fn test_convert_to_ieee_extended() {
		let sample_rate_in_bytes = vec![0x40, 0x0E, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
		let sample_rate = 44100;
		let result = convert_to_ieee_extended(sample_rate);
		assert_eq!(sample_rate_in_bytes, result);
	}
}
*/