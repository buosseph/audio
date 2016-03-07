//! The Waveform Audio File Format
//!
//! WAVE files use the Resource Interchange File Format (RIFF), a generic
//! file container format that uses chunks to store data. All integers are stored
//! in little-endian format, but identifier bytes are in ASCII, big-endian.
//!
//! References
//! - [McGill University](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html)
//! - [WAVE Spec](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/Docs/riffmci.pdf)
//! - [ksmedia.h](http://www-mmsp.ece.mcgill.ca/documents/audioformats/wave/Docs/ksmedia.h)

mod container;
mod chunks;
pub mod encoder;

pub use wave::encoder::Encoder as Encoder;

/// WAVE chunk identifiers.
const RIFF: &'static [u8; 4] = b"RIFF";
const WAVE: &'static [u8; 4] = b"WAVE";
const FMT:  &'static [u8; 4] = b"fmt ";
const DATA: &'static [u8; 4] = b"data";
const FACT: &'static [u8; 4] = b"fact";
