pub mod chunk;
pub mod decoder;
pub mod encoder;

// Hex constants are stored, read, and written as little endian
const RIFF: u32 = 0x46464952;
const WAVE: u32 = 0x45564157;
const FMT:	u32 = 0x20746D66;
const DATA: u32 = 0x61746164;


// Sample = singular f64 value (independent of channel)
// Clip = Group of samples along time domain (Should always include all channels)
// Separate channels into separate tracks for processing