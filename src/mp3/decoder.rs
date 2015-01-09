/* References
 * -- For frame decoding --
 * - http://mpgedit.org/mpgedit/mpeg_format/mpeghdr.htm
 * 
 * -- For side_info decoding --
 * - http://sea-mist.se/fou/cuppsats.nsf/all/857e49b9bfa2d753c125722700157b97/$file/Thesis%20report-%20MP3%20Decoder.pdf
 *
 * - http://cutebugs.net/files/mpeg-drafts/11172-3.pdf (The specification)
 * - http://blog.bjrn.se/2008/10/lets-build-mp3-decoder.html (Some explaination for a Haskell implementation)
 * - https://bitbucket.org/portalfire/pymp3/src (pyMP3 source code)
 * - https://www.ee.columbia.edu/~dpwe/e6820/papers/Pan95-mpega.pdf (Another explaination of compression)
 * - http://www.cs.columbia.edu/~sedwards/classes/2010/4840/reports/KH.pdf (Explaination and C implementation of MPEG-1 Layer III files)
 */

// Approach:
// Start with MPEG-1 Layer III then MPEG-2 Layer III
// Lower layers are considered .mp2 and .mp2, which
// could be implmented in the same module if I figure out the decoding

use audio::{
	AudioResult,
	AudioError
};
use std::io::{File};
use super::{
	KBPS_INDEX,
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

#[derive(Show)]
struct FrameHeader {			// Always 32 bits
	syncword: int,				// 12 bits 	-> always 0xfff (= 4095)
	version: int,				// 1 bit 	-> 1 = MPEG audio, 0 = reserved (or MPEG2?)
	layer: Layer,				// 2 bits 	-> 0x0 = reservec, 0x1 = Layer III, 0x2 = Layer II, 0x3 = Layer I (kinda silly)
	has_crc: bool,				// 1 bit 	-> 1 = no added redundancy, 0 - redundancy added to bitstream (CRC)
	kbps: int,					// 4 bits 	-> decode using huffman, see specification for table
	sample_rate: int,			// 2 bits 	-> 0x0 = 44.1kHz, 0x1 = 48kHz, 0x10 = 32kHz, 0x11 = reserved
	padding: bool,				// 1 bit 	-> padding determined by layer
	private_bit: int,			// 1 bit 	-> not used in future
	mode: String,				// 2 bits 	-> 0x0 = Stereo, 0x1 = Joint Stereo, 0x2 = Dual Channel, 0x3 = Single Channel
	mode_extension: String,		// 2 bits 	-> Used if mode = Joint Stereo, see spec
	copyright: bool,			// 1 bit 	-> 0x1 = copyright protected
	original: bool,				// 1 bit 	-> 0x1 = is original, for lib purposes this will probably always be 0 (copy)
	emphasis: String,			// 2 bits 	-> type of de-emphasis used, see spec
}

#[derive(Show)]
struct SideInfo {
	main_data_begin: int,	// location of start of main data relative from frame header, if 0 then starts after side info, else it's n bytes prior to current frame header in bitstream
	share: int,				// ???
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
	physical_length: uint,
	original_header: &'f [u8],	// for debugging
	header: FrameHeader,
	side_info: SideInfo,
	// data: &'f [u8],	// audio data for frame isn't contained in 'physical frame', thus can be in two frames
}
impl<'f> Frame<'f> {
	/// Decodes frame header information from first four bytes of frame, which are provided as a slice
	fn read_header(slice: &[u8]) -> AudioResult<FrameHeader> {
		let mp3_syncword = ( (slice[0] as u16) << 8 | (slice[1] as u16) ) >> 4;
		if mp3_syncword != 4095 as u16 {
			return Err(AudioError::FormatError("Not a valid MP3 frame".to_string()))
		}

		let version_id = slice[1] << 4 >> 7;
		if version_id != 1 {
			return Err(AudioError::UnsupportedError("This is not MPEG-1 audio".to_string()))
		}
		let layer: Layer =
			match slice[1] << 5 >> 6 {
				1 => Layer::LayerIII,	// mp3
				2 => Layer::LayerII,	// mp2
				3 => Layer::LayerI,		// mp1
				_ => Layer::Reserved 
			};
		if layer != Layer::LayerIII {
			return Err(AudioError::UnsupportedError("This is not Layer III audio (not MP3)".to_string()))
		}
		let protection_bit: bool = match slice[1] << 7 >> 7 { 0 => true, _ => false };

		let kbps = KBPS_INDEX[(slice[2] >> 4) as uint];
		let frequency = SAMP_FREQ_INDEX[(slice[2] << 4 >> 7) as uint];
		let pad_bit: bool = match slice[2] << 6 >> 7 { 0 => false, _ => true };
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
			has_crc: protection_bit,
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

	fn read_side_info(slice: &[u8]) -> AudioResult<SideInfo> {
		match slice.len() {
			32 => Frame::read_dual_side_info(slice),
			17 => Frame::read_single_side_info(slice),
			_  => return Err(AudioError::FormatError(
				"Incorrect slice length provided for decoding frame side info".to_string()))
		}
	}

	fn read_single_side_info(slice: &[u8]) -> AudioResult<SideInfo> {
		let start_ptr = (((slice[0] as u16) << 8 | ((slice[1] >> 7 << 7) as u16)) >> 7) as int;	// remember this is a negative offset
		// let priv_bits = (slice[1] << 1 >> 3);	// 5 bits
		let share = ((slice[1] << 6 >> 4) | (slice[2] >> 6)) as int;
		Ok(SideInfo{
			main_data_begin: start_ptr,
			share: share,
		})
	}

	fn read_dual_side_info(slice: &[u8]) -> AudioResult<SideInfo> {
		let start_ptr = (((slice[0] as u16) << 8 | ((slice[1] >> 7 << 7) as u16)) >> 7) as int;	// remember this is a negative offset
		// let priv_bits = (slice[1] << 1 >> 5);	// 3 bits
		// println!("{}", start_ptr);
		Ok(SideInfo{
			main_data_begin: start_ptr,
			share: 0
		})
	}
}

// Need seek_frame to find first valid frame (MP3s usually have a preceeding ID3 tag).
// Since there can be a lot of data that can contain false syncwords, will need to decode
// the candidate header and the next frame header for validation.
#[derive(Show)]
struct Bitstream {
	// stream: String,
	buffer: Vec<u8>,
}
impl Bitstream {
	fn read_frames(&mut self) -> AudioResult<Vec<Frame>> {
		let mut v: Vec<Frame> = Vec::new();
		let mut i = 0;
		let mut data_ptr: uint = 0;
		while i < self.buffer.len() {
			let frame = try!(self.read_frame(i, /*data_ptr*/));
			i += frame.physical_length;
			v.push(frame);
		}
		println!("{}\n{}", v[2], v[2].side_info.main_data_begin);
		Ok(v)
	}

	fn read_frame(&self, header_pos: uint, /*prev_data_ptr: &mut uint*/) -> AudioResult<Frame> {
		let mut start_pos: uint = header_pos;
		let mut end_pos: uint = header_pos + 4;
		let orig = self.buffer.slice(start_pos, end_pos); // Only for debugging
		let header: FrameHeader = try!(Frame::read_header(self.buffer.slice(start_pos, end_pos)));
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

		// Skip CRC for now
		let crc_length: uint = match header.has_crc { true => 2, false => 0 };
		start_pos = end_pos;
		end_pos += crc_length;

		let side_info_length: uint =	// I think this is the same between all layers. Not important right now
			match header.mode.as_slice() {
				"single_channel" 	=> 17,
				_					=> 32
			};
		start_pos 				= end_pos;
		end_pos 				+= side_info_length;
		let side_info: SideInfo = try!(Frame::read_side_info(self.buffer.slice(start_pos, end_pos)));

		// let data_length : uint 	= frame_length - 4 - side_info_length;
		// start_pos				= end_pos;
		// end_pos 				+= data_length;
		// let buffer 		: &[u8] = self.buffer.slice(start_pos, end_pos);

		// let data_start: uint = header_pos - side_info.main_data_begin;	// This still includes frame headers and side info

		Ok(Frame{
			physical_length: frame_length,
			original_header: orig,	// for debugging
			header: header,
			side_info: side_info,
			// data: 	buffer
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





















