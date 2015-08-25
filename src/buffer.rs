use sample::{Sample, SampleOrder};

/// A container for audio samples and important attributes.
#[derive(Clone, Debug)]
pub struct AudioBuffer {
  /// Number of quantization levels
  pub bit_depth:   u32,
  /// Number of samples per second
  pub sample_rate: u32,
  /// Number of channels
  pub channels:    u32,
  /// Organization of samples
  pub order:       SampleOrder,
  /// Decoded audio samples
  pub samples:     Vec<Sample>
}
