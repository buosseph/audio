/* References
 * - http://cutebugs.net/files/mpeg-drafts/11172-3.pdf (The specification)
 * - http://blog.bjrn.se/2008/10/lets-build-mp3-decoder.html (Some explaination for a Haskell implementation)
 * - https://bitbucket.org/portalfire/pymp3/src (pyMP3 source code)
 * - https://www.ee.columbia.edu/~dpwe/e6820/papers/Pan95-mpega.pdf (Another explaination of compression)
 * - http://www.cs.columbia.edu/~sedwards/classes/2010/4840/reports/KH.pdf (Explaination and C implementation of MPEG-1 Layer III files)
 */

use audio::RawAudio;
use audio::SampleOrder;
use std::io::{File, IoResult};
use std::path::posix::Path;

#[derive(Show)]
struct Bitstream {
	// stream: String,
	buffer: Vec<u8>
}


// Frame and dependant structs
struct FrameHeader {
	// mpeg_version: int,
	syncword: int,				// 12 bits 	-> always 0xfff
	id: int,					// 1 bit 	-> 1 = MPEG audio, 0 = reserved
	layer: int,					// 2 bits 	-> 0x0 = reservec, 0x1 = Layer III, 0x2 = Layer II, 0x3 = Layer I (kinda silly)
	protection_bit: int,		// 1 bit 	-> 1 = no added redundancy, 0 - redundancy added to bitstream
	bit_rate_index: int,		// 4 bits 	-> decode using huffman, see specification for table
	sampling_frequency: int,	// 2 bits 	-> 0x0 = 44.1kHz, 0x1 = 48kHz, 0x10 = 32kHz, 0x11 = reserved
	padding_bit: bool,			// 1 bit 	-> see spec, kinda hard to put into comment
	private_bit: int,			// 1 bit 	-> not used in future
	mode: String,				// 2 bits 	-> 0x0 = Stereo, 0x1 = Joint Stereo, 0x2 = Dual Channel, 0x3 = Single Channel
	mode_extension: String,		// 2 bits 	-> Used if mode = Joint Stereo, see spec
	copyright: bool,			// 1 bit 	-> 0x1 = copyright protected
	original: bool,				// 1 bit 	-> 0x1 = is original, for lib purposes this will probably always be 0 (copy)
	emphasis: String,			// 2 bits 	-> type of de-emphasis used, see spec
	// has_crc: bool,
}

struct SideInfo {
    data_begin: int,
    private_bits: int,
    // Matrices...
}

struct FrameData {
    scalefac_s: int, // More Matrices...
}

struct DecodedData {
    dq_vals: int,	// More Matrices...
}

struct Frame {
	header: FrameHeader,
	side_info: SideInfo,		// if mono 17 bytes, if stereo 32 bytes
	data: FrameData,
	// decoded_data: DecodedData 	// Every frame has 1152 samples
}
impl Frame {
	fn read_frame() -> Frame {
		// Read data differently based on FrameHeader.layer
		unimplemented!();
	}

	fn read_header() -> FrameHeader {
		// Take in slice or [u8; 4]
		unimplemented!();
	}

	fn error_check(&mut self) {
		if self.header.protection_bit == 0 {
			// crc_check
		}
		unimplemented!();
	}
}





pub fn read_file_meta(file_path: &str) -> IoResult<()> {
	let path = Path::new(file_path);
	let mut file = try!(File::open(&path));

	let mut buffer = file.read_to_end().unwrap();
	let mut bitstream = Bitstream { buffer: buffer };

	// whisle (true) {
	// 	Frame::read_frame()
	// }

	Ok(())
}





















