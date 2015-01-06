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

use super::{
	BIT_RATE_HUFF_INDEX,
	SAMP_FREQ_INDEX,
	MODE_INDEX,
	Mode
};

// Frame and dependant structs
#[derive(Show)]
struct FrameHeader {			// Always 32 bits
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
}

#[derive(Show)]
struct SideInfo {		// if mono 17 bytes, if stereo 32 bytes
    data_begin: int,
    private_bits: int,
    // Matrices...
}

#[derive(Show)]
struct FrameData {		// Every frame has 1152 samples
    scalefac_s: int, 	// More Matrices...
}

#[derive(Show)]
struct DecodedData {
    dq_vals: int,		// More Matrices...
}

#[derive(Show)]
struct Frame {
	header: FrameHeader,
	side_info: SideInfo,
	data: FrameData,
}
impl Frame {
	fn read_frame() -> Frame {
		// Read data differently based on FrameHeader.layer
		unimplemented!();
	}

	fn read_header(slice: &[u8]) -> FrameHeader {
		println!("{}", slice);
		FrameHeader {
			syncword: 0,
			id: 0,
			layer: 0,
			protection_bit: 0,
			bit_rate_index: 0,
			sampling_frequency: 0,
			padding_bit: false,
			private_bit: 0,
			mode: "Mono".to_string(),
			mode_extension: "None".to_string(),
			copyright: false,
			original: false,
			emphasis: "None".to_string(),
		}
	}

	fn error_check(&mut self) {
		if self.header.protection_bit == 0 {
			// crc_check
		}
		unimplemented!();
	}
}


#[derive(Show)]
struct Bitstream {
	// stream: String,
	buffer: Vec<u8>
}
impl Bitstream {
	pub fn read_first_frame_header(&mut self) {
		let header = Frame::read_header(self.buffer.slice(0, 4));
		println!("{}", header);
	} 
}



pub fn read_file_meta(file_path: &str) -> IoResult<()> {
	println!("{}", file_path);
	let path = Path::new(file_path);
	let mut file = try!(File::open(&path));

	let mut buffer = file.read_to_end().unwrap();
	let mut bitstream = Bitstream { buffer: buffer };

	bitstream.read_first_frame_header();

	// for symbol in range(0, 16) {
	// 	let value = BIT_RATE_HUFF_INDEX[symbol];
	// 	println!("{}: {}", symbol, value);
	// }

	// for symbol in range(0, 4) {
	// 	let value = SAMP_FREQ_INDEX[symbol];
	// 	println!("{}: {}", symbol, value);
	// }

	// for symbol in range(0, 4) {
	// 	let value = MODE_INDEX[symbol];
	// 	println!("{}: {}", symbol, value);
	// }

	Ok(())
}





















