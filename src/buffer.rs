/// An enumeration for keeping track of how samples are organized in the loaded audio.
/// Multichannel samples are usually interleaved, but other orderings are included if they
/// are needed in the future.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SampleOrder { MONO, INTERLEAVED, REVERSED, PLANAR }

/// Holds all samples and necessary audio data for processing and encoding.
#[derive(Clone, Debug)]
pub struct AudioBuffer {
  pub bit_rate: u32,
  pub sample_rate: u32,
  pub channels: u32,
  pub order: SampleOrder,
  pub samples: Vec<Sample>
}

pub type Sample = f64;

/// Converts object to a `Sample` value.
///
/// For integer types, the maximum value will be mapped to a value less than 1.
pub trait ToSample {
  fn to_sample(self) -> Sample;
}

impl ToSample for u8 {
  #[inline]
  fn to_sample(self) -> Sample {
    let result = self as Sample;
    (result - 128f64) / 128f64
  }
}

impl ToSample for i8 {
  #[inline]
  fn to_sample(self) -> Sample {
    let result = self as Sample;
    result / 128f64
  }
}

impl ToSample for i16 {
  #[inline]
  fn to_sample(self) -> Sample {
    let result = self as Sample;
    result / 32_768f64
  }
}

impl ToSample for i32 {
  #[inline]
  fn to_sample(self) -> Sample {
    let result = self as Sample;
    result / 2_147_483_648f64
  }
}

impl ToSample for f32 {
  #[inline]
  fn to_sample(self) -> Sample {
    self as Sample
  }
}

// To be consistent
impl ToSample for f64 {
  #[inline]
  fn to_sample(self) -> Sample {
    self
  }
}

/// Converts a `Sample` to an object.
///
/// For integer types, mapping the maximum value of a `Sample` are clipped to
/// prevent arithmetic overflow in the result.
pub trait FromSample {
  fn from_sample(sample: Sample) -> Self;
}

impl FromSample for u8 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    let result = sample * 128f64 + 128f64;
    if result > 255f64 {
      u8::max_value()
    }
    else {
      result as u8
    }
  }
}

impl FromSample for i8 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    let result = sample * 128f64;
    if result > 128f64 - 1f64 {
      i8::max_value()
    }
    else {
      result as i8
    }
  }
}

impl FromSample for i16 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    let result = sample * 32_768f64;
    if result > 32_768f64 - 1f64 {
      i16::max_value()
    }
    else {
      result as i16
    }
  }
}

impl FromSample for i32 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    let result = sample * 2_147_483_648f64;
    if result > 2_147_483_648f64 - 1f64 {
      i32::max_value()
    }
    else {
      result as i32
    }
  }
}

impl FromSample for f32 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    sample as f32
  }
}

// To be consistent
impl FromSample for f64 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    sample
  }
}
