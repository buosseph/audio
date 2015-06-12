pub type Sample = f64;

/// An enumeration for keeping track of how samples are organized in the loaded audio.
/// Multichannel samples are usually interleaved, but other orderings are included if they
/// are needed in the furutre.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SampleOrder { MONO, INTERLEAVED, REVERSED, PLANAR }

/// Holds all samples and necessary audio data for processing and encoding.
#[derive(Clone, Debug)]
pub struct AudioBuffer {
  pub bit_rate: u32,
  pub sample_rate: u32,
  pub channels: u32,
  pub order: SampleOrder,
  pub samples: Vec<Sample>,
}
