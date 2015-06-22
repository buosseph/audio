use std::io::Write;
use audio::AudioEncoder;
use buffer::*;
use codecs::Codec;
use traits::Container;
use aiff::container::AiffContainer;
use error::AudioResult;

pub struct Encoder<'w, W: 'w> {
  writer: &'w mut W,
}

impl<'w, W> Encoder<'w, W> where W: Write {
  pub fn new(writer: &'w mut W) -> Encoder<'w, W> {
    Encoder {
      writer: writer
    }
  }
}

impl<'w, W> AudioEncoder for Encoder<'w, W> where W: Write {
  fn encode(&mut self, audio: &AudioBuffer) -> AudioResult<()> {
    // Codec must be passed to container to determine if it's supported
    let buffer: Vec<u8> = try!(AiffContainer::create(Codec::LPCM, audio));
    try!(self.writer.write_all(&buffer));
    Ok(())
  }
}
