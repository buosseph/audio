//! The Audio Interchange File Format
//!
//! AIFF files use the Interchange File Format (IFF), a generic file container
//! format that uses chunks to store data. All bytes are stored in big-endian
//! format.
//!
//! References
//! - [McGill University](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/AIFF/AIFF.html)
//! - [AIFF Spec](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/AIFF/Docs/AIFF-1.3.pdf)
//! - [AIFF/AIFFC Spec from Apple](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/AIFF/Docs/MacOS_Sound-extract.pdf)

mod container;
mod chunks;
pub mod encoder;

pub use aiff::encoder::Encoder as Encoder;

/// AIFF/AIFC chunk identifiers.
const FORM: &'static [u8; 4] = b"FORM";
const AIFF: &'static [u8; 4] = b"AIFF";
const AIFC: &'static [u8; 4] = b"AIFC";
const FVER: &'static [u8; 4] = b"FVER";
const COMM: &'static [u8; 4] = b"COMM";
const SSND: &'static [u8; 4] = b"SSND";

/// AIFF-C Version 1 timestamp for the FVER chunk.
const AIFC_VERSION_1: u32 = 0xA2805140;
