pub mod decoder;

// Symbol keys are array incides (0-15, 0-4, etc.)
const BIT_RATE_HUFF_INDEX: [u16; 16] = [ 0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0 ];
const SAMP_FREQ_INDEX: [f64; 4] = [ 44100f64, 48000f64, 32000f64, -1f64 ];
const MODE_INDEX: [&'static str; 4] = [ "stereo", "joint_stereo", "dual_channel", "single_channel" ];
const MODE3_INDEX: [&'static str; 4] = [ "stereo", "intensity stereo and/or ms stereo", "dual_channel", "single_channel" ];

// In a rush fix up later
const EXT_INDEX: [&'static str; 4] = ["bound==4","bound==8","bound==12","bound==16"];
const EXT3_INDEX: [&'static str; 4] = ["i: off, ms: off","i: on, ms: off","i: off, ms: on","i: on, ms: on"];
const EMPH_INDEX: [&'static str; 4] = ["None","50/15 ms","reserved","CCITT J.17"];