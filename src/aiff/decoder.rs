use audio::{
	AudioResult,
	AudioError,
	RawAudio,
	SampleOrder
};
use std::io::{File};
use super::chunk;
use super::{FORM, COMM, SSND};

pub fn read_file_meta(file_path: &str) -> AudioResult<()> {
	let path = Path::new(file_path);
	let mut file = try!(File::open(&path));

	let iff_header =  file.read_be_i32().unwrap();
	if iff_header != FORM {
		return Err(AudioError::FormatError(
			"File is not valid AIFF".to_string()
		))
	}
	let header = chunk::IFFHeader::read_chunk(&mut file).unwrap();


	let comm_chunk_marker = file.read_be_i32().unwrap();
	if comm_chunk_marker != COMM {
		return Err(AudioError::FormatError(
			"File is not valid AIFF. Does not contain required common chunk.".to_string()
		))
	}
	let comm = chunk::CommonChunk::read_chunk(&mut file).unwrap();

	let ssnd_chunk_marker = file.read_be_i32().unwrap();
	if ssnd_chunk_marker != SSND {
		return Err(AudioError::FormatError(
			"File is not valid AIFF. Does not contain required sound data chunk.".to_string()
		))
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
		comm.sample_rate,
		ssnd.size,
		ssnd.offset,
		ssnd.block_size
		);

	Ok(())
}

/* Most recent benchmark:
 * - 152916993 ns/iter (+/- 60141351)
 */
pub fn read_file(path: &Path) -> AudioResult<RawAudio> {
	let mut file = try!(File::open(path));

	let iff_header =  file.read_be_i32().unwrap();
	if iff_header != FORM {
		return Err(AudioError::FormatError(
			"File is not valid AIFF".to_string()
		))
	}
	let header = chunk::IFFHeader::read_chunk(&mut file).unwrap();

	let comm_chunk_marker = file.read_be_i32().unwrap();
	if comm_chunk_marker != COMM {
		return Err(AudioError::FormatError(
			"File is not valid AIFF. Does not contain required common chunk.".to_string()
		))
	}
	let comm = chunk::CommonChunk::read_chunk(&mut file).unwrap();

	let ssnd_chunk_marker = file.read_be_i32().unwrap();
	if ssnd_chunk_marker != SSND {
		return Err(AudioError::FormatError(
			"File is not valid AIFF. Does not contain required sound data chunk.".to_string()
		))
	}
	let ssnd = chunk::SoundDataChunk::read_chunk(&mut file).unwrap();


	let num_of_frames: uint = comm.num_of_frames as uint;
	let mut samples: Vec<f64> = Vec::with_capacity(num_of_frames * comm.num_of_channels as uint);

	let mut frame: &[u8];
	match comm.bit_rate {
		16 	=> {
			match comm.num_of_channels {
				2 	=> {
					for i in range(0, num_of_frames) {
						frame = ssnd.data.slice(i * 4 as uint, i * 4 as uint + 4 as uint);

						let left_sample	: i16 	= (frame[0] as i16) << 8 | frame[1] as i16;
						let right_sample: i16 	= (frame[2] as i16) << 8 | frame[3] as i16;
					
						let float_left	: f64 	= left_sample as f64 / 32768f64;
						let float_right	: f64 	= right_sample as f64 / 32768f64;

						samples.push(float_left);
						samples.push(float_right);
					}
				},

				1 	=> {
					for i in range(0, num_of_frames) {
						frame = ssnd.data.slice(i * 2 as uint, i * 2 as uint + 2 as uint);
						let sample : i16 = (frame[0] as i16) << 8 | frame[1] as i16;
						let float_sample : f64 = sample as f64 / 32768f64;
						samples.push(float_sample);
					}
				},

				_	=> {
					return Err(AudioError::UnsupportedError(
						format!(
						"Cannot read {}-channel .aiff files.",
						comm.num_of_channels)
					))
				}
			}
		},

		_	=> {
			return Err(AudioError::UnsupportedError(
				format!(
				"Cannot read {}-bit .aiff files.",
				comm.bit_rate)
			))
		}
	}

	Ok(
		RawAudio {
			bit_rate: 		comm.bit_rate as uint,
			sample_rate: 	comm.sample_rate as uint,
			channels: 		comm.num_of_channels as uint,
			order: 			SampleOrder::INTERLEAVED,
			samples: 		samples,
		}
	)
}

#[cfg(test)]
mod tests {
	extern crate test;

	use super::*;

	#[bench]
	fn bench_read_file(b: &mut test::Bencher) {
		b.iter(|| {
			let _ = read_file(&Path::new("test/aiff/Warrior Concerto - no meta.aiff"));
		});
	}
}