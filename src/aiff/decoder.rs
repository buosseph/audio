use audio::AudioError;
use audio::AudioResult;

use std::io::{File};
use std::path::posix::{Path};

use super::chunk;
use super::{FORM, COMM, SSND};


pub fn read_file_data(file_path: &str) -> AudioResult<()> {
	let path = Path::new(file_path);
	let mut file = try!(File::open(&path));

	let iff_header =  file.read_be_i32().unwrap();
	if iff_header != FORM {
		return Err(AudioError::FormatError("File is not valid AIFF.".to_string()))
	}
	let header = chunk::IFFHeader::read_chunk(&mut file).unwrap();


	let comm_chunk_marker = file.read_be_i32().unwrap();
	if comm_chunk_marker != COMM {
		return Err(AudioError::FormatError("File is not valid AIFF. Does not contain required common chunk.".to_string()))
	}
	let comm = chunk::CommonChunk::read_chunk(&mut file).unwrap();


	let ssnd_chunk_marker = file.read_be_i32().unwrap();
	if ssnd_chunk_marker != SSND {
		return Err(AudioError::FormatError("File is not valid AIFF. Does not contain required sound data chunk.".to_string()))
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

	Ok(())
}