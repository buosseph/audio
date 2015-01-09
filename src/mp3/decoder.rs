/* References
 * -- For frame decoding --
 * - http://mpgedit.org/mpgedit/mpeg_format/mpeghdr.htm
 * 
 *
 *
 * - http://cutebugs.net/files/mpeg-drafts/11172-3.pdf (The specification)
 * - http://blog.bjrn.se/2008/10/lets-build-mp3-decoder.html (Some explaination for a Haskell implementation)
 * - https://bitbucket.org/portalfire/pymp3/src (pyMP3 source code)
 * - https://www.ee.columbia.edu/~dpwe/e6820/papers/Pan95-mpega.pdf (Another explaination of compression)
 * - http://www.cs.columbia.edu/~sedwards/classes/2010/4840/reports/KH.pdf (Explaination and C implementation of MPEG-1 Layer III files)
 */

// Approach:
// Start with MPEG-1 Layer III then MPEG-2 Layer III
// Lower layers can come later, as those to are the most common (I think)

use audio::{
	AudioResult,
	AudioError
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

#[derive(Show, PartialEq, Eq)]
enum Layer {
	LayerI 		= 0x3,
	LayerII 	= 0x2,
	LayerIII	= 0x1,
	Reserved 	= 0x0
}

// Frame and dependant structs
#[derive(Show)]
struct FrameHeader {			// Always 32 bits
	syncword: int,				// 12 bits 	-> always 0xfff (= 4095)
	version: int,				// 1 bit 	-> 1 = MPEG audio, 0 = reserved (or MPEG2?)
	layer: Layer,				// 2 bits 	-> 0x0 = reservec, 0x1 = Layer III, 0x2 = Layer II, 0x3 = Layer I (kinda silly)
	protection_bit: int,		// 1 bit 	-> 1 = no added redundancy, 0 - redundancy added to bitstream (CRC)
	kbps: int,					// 4 bits 	-> decode using huffman, see specification for table
	sample_rate: int,			// 2 bits 	-> 0x0 = 44.1kHz, 0x1 = 48kHz, 0x10 = 32kHz, 0x11 = reserved
	padding: bool,			// 1 bit 	-> see spec, kinda hard to put into comment
	private_bit: int,			// 1 bit 	-> not used in future
	mode: String,				// 2 bits 	-> 0x0 = Stereo, 0x1 = Joint Stereo, 0x2 = Dual Channel, 0x3 = Single Channel
	mode_extension: String,		// 2 bits 	-> Used if mode = Joint Stereo, see spec
	copyright: bool,			// 1 bit 	-> 0x1 = copyright protected
	original: bool,				// 1 bit 	-> 0x1 = is original, for lib purposes this will probably always be 0 (copy)
	emphasis: String,			// 2 bits 	-> type of de-emphasis used, see spec
}

// Every frame has n slots. In Layer I, one slot is 4 bytes long.
// In Layer II and III, one slot is one byte long. These are the equations.
//
// Layer I Frame Length (in bytes) = (12 * kbps / sample_rate + padding) * 4
// Layer II or III Frame Length (in bytes) = 144 * kbps / sample_rate + padding
//
// Know that the bit rate (kbps) is expressed as at the same magnitude as bits.
//
// E.g. LayerIII, bit_rate = 128kbps = 128000bps, sample_rate = 441000, padding = 0
//		=> frame_length = 417 bytes (remember to truncate, this is an int value)
//
// Every frame, despite its length, guarantees some number of samples
// depending on the layers. Layer I frames store 384 samples, but 
// Layer II and III store 1152 samples per frame.
// 
#[derive(Show)]
struct Frame<'f> {
	length: uint,
	header: FrameHeader,
	// side_info: SideInfo,
	data: &'f [u8],
}
impl<'f> Frame<'f> {
	/// Decodes frame header information from first four bytes of frame, which are provided as a slice
	fn read_header(slice: &[u8]) -> AudioResult<FrameHeader> {
		let mp3_syncword = ( (slice[0] as u16) << 8 | (slice[1] as u16) ) >> 4;
		if mp3_syncword != 4095 as u16 {
			return Err(AudioError::FormatError("Not a valid MP3 frame".to_string()))
		}

		let version_id = slice[1] << 4 >> 7;
		let layer: Layer =
			match slice[1] << 5 >> 6 {
				1 => Layer::LayerIII,
				2 => Layer::LayerII,
				3 => Layer::LayerI,
				_ => Layer::Reserved 
			};
		let protection_bit = slice[1] << 7 >> 7;

		let kbps = BIT_RATE_HUFF_INDEX[(slice[2] >> 4) as uint];
		let frequency = SAMP_FREQ_INDEX[(slice[2] << 4 >> 7) as uint];
		let pad_bit: bool = match slice[2] << 6 >> 7 { 0 => false, _ => true};
		let priv_bit = slice[2] << 7 >> 7;

		let mode = match layer {
			Layer::LayerIII => MODE3_INDEX[(slice[3] >> 6) as uint],
			_ 				=> MODE_INDEX[(slice[3] >> 6) as uint]
		};
		let extension = match layer {
			Layer::LayerIII => EXT3_INDEX[(slice[3] << 2 >> 6) as uint],
			_ 				=> EXT_INDEX[(slice[3] << 2 >> 6) as uint]
		};

		let copy_bit: bool = match slice[3] << 4 >> 7 { 0 => false, _ => true};
		let home_bit: bool = match slice[3] << 5 >> 7 { 0 => false, _ => true};
		let emphasis = EMPH_INDEX[(slice[3] << 6 >> 7) as uint];

		Ok(FrameHeader {
			syncword: mp3_syncword as int,
			version: version_id as int,
			layer: layer,
			protection_bit: protection_bit as int,
			kbps: kbps as int,
			sample_rate: frequency as int,
			padding: pad_bit,
			private_bit: priv_bit as int,
			mode: mode.to_string(),
			mode_extension: extension.to_string(),
			copyright: copy_bit,
			original: home_bit,
			emphasis: emphasis.to_string(),
		})
	}
}

#[derive(Show)]
struct Bitstream {
	// stream: String,
	buffer: Vec<u8>,
}
impl Bitstream {
	fn read_frames(&mut self) -> AudioResult<Vec<Frame>> {
		let mut v: Vec<Frame> = Vec::new();
		let mut i = 0;
		while i < self.buffer.len() {
			let frame = try!(self.read_frame(i));
			i += frame.length;
			v.push(frame);
		}
		Ok(v)
	}

	fn read_frame(&self, buffer_pos: uint) -> AudioResult<Frame> {
		let header: FrameHeader = try!(Frame::read_header(self.buffer.slice(buffer_pos, buffer_pos + 4)));
		let padding_length: int = // in bytes (is it suppose to be bits? Doubt it)
			match (&header.layer, header.padding){
				(&Layer::LayerI, true) 	=> 4,
				(_, true)				=> 1,
				(_, false)				=> 0
			};

		let frame_length: uint =
			match header.layer {
				Layer::LayerI => unimplemented!(),
				_ => (144 * (header.kbps * 1000) / header.sample_rate + padding_length) as uint
			};

		let data_length : uint 	= frame_length - 4; // frame_length includes header bytes
		let buffer 		: &[u8] = self.buffer.slice(buffer_pos + 4, buffer_pos + 4 + data_length);

		Ok(Frame{
			length: frame_length,
			header: header,
			data: 	buffer
		})
	}
}


// only for debugging
pub fn read_file_meta(path: &Path) -> AudioResult<()> {
	let mut file = try!(File::open(path));

	let buffer = file.read_to_end().unwrap();
	let mut bitstream = Bitstream { buffer: buffer };

	bitstream.read_frames();

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





















