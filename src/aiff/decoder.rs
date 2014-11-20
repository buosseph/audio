use std::io::{File};
use std::path::posix::{Path};

use super::chunk;
use super::{FORM, COMM, SSND};


pub fn read_file_data(file_path: &str) {
	let path = Path::new(file_path);
	let mut file = match File::open(&path) {
		Ok(f)	=> f,
		Err(e)	=> fail!("File error: {}", e),
	};


	let iff_header =  file.read_be_i32().unwrap();
	if iff_header != FORM {
		fail!("File is not valid AIFF.");
	}
	let header = chunk::IFFHeader::read_chunk(&mut file).unwrap();


	let comm_chunk_marker = file.read_be_i32().unwrap();
	if comm_chunk_marker != COMM {
		fail!("File is not valid AIFF. Does not contain required common chunk.");
	}
	let comm = chunk::CommonChunk::read_chunk(&mut file).unwrap();


	let ssnd_chunk_marker = file.read_be_i32().unwrap();
	if ssnd_chunk_marker != SSND {
		fail!("File is not valid AIFF. Does not contain required sound data chunk.");
	}
	let ssnd = chunk::SoundDataChunk::read_chunk(&mut file).unwrap();


	println!(
	"master_iff_chunk:
		(FORM) {}
		File size: {}
		File type: (AIFF) {}
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
		header.form_type,
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