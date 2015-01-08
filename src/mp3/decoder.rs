/* References
 * - http://cutebugs.net/files/mpeg-drafts/11172-3.pdf (The specification)
 * - http://blog.bjrn.se/2008/10/lets-build-mp3-decoder.html (Some explaination for a Haskell implementation)
 * - https://bitbucket.org/portalfire/pymp3/src (pyMP3 source code)
 * - https://www.ee.columbia.edu/~dpwe/e6820/papers/Pan95-mpega.pdf (Another explaination of compression)
 * - http://www.cs.columbia.edu/~sedwards/classes/2010/4840/reports/KH.pdf (Explaination and C implementation of MPEG-1 Layer III files)
 */

// Approach:
// Start with MPEG-1 Layer III then MPEG-2 Layer III
// Lower layers can come later, as those to are the most common

use audio::{
	AudioResult,
	AudioError,
	RawAudio,
	SampleOrder
};
use std::io::{File};
use super::{
	BIT_RATE_HUFF_INDEX,
	SAMP_FREQ_INDEX,
	MODE_INDEX,
	MODE3_INDEX,
	EXT_INDEX,
	EXT3_INDEX,
	EMPH_INDEX
};

// Frame and dependant structs
#[derive(Show)]
struct FrameHeader {			// Always 32 bits
	syncword: int,				// 12 bits 	-> always 0xfff (= 4095)
	version: int,				// 1 bit 	-> 1 = MPEG audio, 0 = reserved
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

	fn read_header(slice: &[u8]) -> AudioResult<FrameHeader> {
		// println!("{}", slice);

		let mp3_syncword = ( (slice[0] as u16) << 8 | (slice[1] as u16) ) >> 4;
		if mp3_syncword != 4095 as u16 {
			return Err(AudioError::FormatError("Not a valid MP3 frame".to_string()))
		}

		let version_id = slice[1] << 4 >> 7;
		let layer_id = slice[1] << 5 >> 6;
		let protection_bit = slice[1] << 7 >> 7;

		let kbps = BIT_RATE_HUFF_INDEX[(slice[2] >> 4) as uint];
		let frequency = SAMP_FREQ_INDEX[(slice[2] << 4 >> 7) as uint];
		let pad_bit: bool = match slice[2] << 6 >> 7 { 0 => false, _ => true};
		let priv_bit = slice[2] << 7 >> 7;

		// Probably put layer_id as enum to make more sense!
		let mode = match layer_id as int {
			1 => MODE3_INDEX[(slice[3] >> 6) as uint],
			_ => MODE_INDEX[(slice[3] >> 6) as uint]
		};
		let extension = match layer_id as int {
			1 => EXT3_INDEX[(slice[3] << 2 >> 6) as uint],
			_ => EXT_INDEX[(slice[3] << 2 >> 6) as uint]
		};

		let copy_bit: bool = match slice[3] << 4 >> 7 { 0 => false, _ => true};
		let home_bit: bool = match slice[3] << 5 >> 7 { 0 => false, _ => true};
		let emphasis = EMPH_INDEX[(slice[3] << 6 >> 7) as uint];

		Ok(FrameHeader {
			syncword: mp3_syncword as int,
			version: version_id as int,
			layer: layer_id as int,
			protection_bit: protection_bit as int,
			bit_rate_index: kbps as int,
			sampling_frequency: frequency as int,
			padding_bit: pad_bit,
			private_bit: priv_bit as int,
			mode: mode.to_string(),
			mode_extension: extension.to_string(),
			copyright: copy_bit,
			original: home_bit,
			emphasis: emphasis.to_string(),
		})
	}

	fn error_check(&mut self) {
		if self.header.protection_bit == 0 {
			// crc_check
		}
		unimplemented!();
	}
}

// Maybe unnecessary
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



pub fn read_file_meta(file_path: &str) -> AudioResult<()> {
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





















