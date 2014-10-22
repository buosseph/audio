//use audio::AudioDecoder;
//use audio::{RawAudio, SampleOrder};

use std::str::from_utf8;
use std::io::{File};
use std::path::posix::{Path};

pub mod chunk;


pub fn read_file_data(file_path: &str) {
	let path = Path::new(file_path);
	let mut file = match File::open(&path) {
		Ok(f)	=> f,
		Err(e)	=> fail!("File error: {}", e),
	};

	let double_word = file.read_exact(4).unwrap();
	let iff_header = from_utf8(double_word.as_slice()).unwrap();
	// If FORM => read IffHeader, else fail
	let header = chunk::IFFHeader::read_chunk(&mut file).unwrap();


	let double_word = file.read_exact(4).unwrap();
	let comm_chunk_marker = from_utf8(double_word.as_slice()).unwrap();
	// If COMM => read CommonChunk, else fail
	let comm = chunk::CommonChunk::read_chunk(&mut file).unwrap();


	let double_word = file.read_exact(4).unwrap();
	let ssnd_chunk_marker = from_utf8(double_word.as_slice()).unwrap();
	// If SSND => read SoundDataChunk, else fail
	let ssnd = chunk::SoundDataChunk::read_chunk(&mut file).unwrap();


	println!(
		"master_iff_chunk:
			{}
			File size: {}
			File type: AIFF
		COMM_chunk:
			Chunk size: {},
			Channels: {},
			Frames: {},
			Bit rate: {},
			Sample rate (extended): {},
		SSND_chunk:
			Chunk size: {},
			Offset: {},
			Block size: {},
		",
		iff_header,
		header.size,
		comm.size,
		comm.num_of_channels,
		comm.num_of_frames,
		comm.bit_rate,
		comm.sampling_rate,
		ssnd.size,
		ssnd.offset,
		ssnd.block_size
		);


}