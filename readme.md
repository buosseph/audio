# audio
[![Build Status](https://travis-ci.org/brianuosseph/audio.svg?branch=master)](https://travis-ci.org/brianuosseph/audio)
[![Coverage Status](https://coveralls.io/repos/brianuosseph/audio/badge.svg?branch=master&service=github)](https://coveralls.io/github/brianuosseph/audio?branch=master)

A Rust audio coding library.

[Documentation](http://brianuosseph.github.io/audio)

## Decoding

| Audio Format | Variant | Codec | Data formats |
| ---- | ---------- | ----- | ------------------------------- |
| WAVE |            | PCM   | u8, i16, i24, i32, f32, f64     |
|      |            | G.711 | alaw, ulaw                      |
| AIFF | Aiff, Aifc | PCM   | u8, i8, i16, i24, i32, f32, f64 |
|      |            | G.711 | alaw, ulaw                      |

## Encoding

| Audio Format | Variant | Codec | Bit Rates |
| ---- | ---------- | ----- | ------------------------------- |
| WAVE |            | PCM   | u8, i16, i24, i32, f32, f64     |
|      |            | G.711 | alaw, ulaw                      |
| AIFF | Aiff, Aifc | PCM   | u8, i8, i16, i24, i32, f32, f64 |
|      |            | G.711 | alaw, ulaw                      |

## TODO
- Improved support for alternative WAVE formats
  - Clear up ambiguity on use cases of `WAVE_FORMAT_EXTENSIBLE`
  - Should the user specify when to use format variants, as done in Audacity?
    - This would also apply to AIFF-C
- Improved error messages
  - Revise messages throughout code
- Improved testing
  - Improve integration tests
  - Add unit tests for better coverage
- Improved examples and documentation
  - Inspect generated docs and apply revisions where needed
  - Add examples in documentation
  - Create more example programs
- Improved multichannel support?
  - Represent channel layout
- Add metadata support?
  - Requires additional support for RIFF and IFF textual chunks
  - Requires handling of possible ID3 metadata
    - Using `crate rust-id3`
- Miscellaneous
  - Add a "from_buffer" function?
  - Add `load_as` for reading data as a different audio format?
  - Come up with a name!
  - Explore other audio formats
