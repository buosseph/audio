# audio [![Build Status](https://travis-ci.org/brianuosseph/audio.svg?branch=master)](https://travis-ci.org/brianuosseph/audio)
A Rust audio coding library.

## Decoding

| Audio Format | Codec | Data formats |
| ------ | ----- | --------- |
| WAVE | PCM   | u8, i16, i24, i32, f32, f64 |
|      | G.711 | alaw, ulaw |
| AIFF | PCM   | u8, i8, i16, i24, i32, f32, f64 |
|      | G.711 | alaw, ulaw |

## Encoding

| Audio Format | Codec | Bit Rates |
| ------ | ----- | --------- |
| WAVE | PCM   | u8, i16, i24, i32, f32, f64 |
|      | G.711 | alaw, ulaw |
| AIFF | PCM   | u8, i8, i16, i24, i32, f32, f64 |
|      | G.711 | alaw, ulaw |

## TODO
- Improved multichannel support
  - Represent channel layout
- Improved support for alternative WAVE formats
  - Clear up ambiguity on use cases of `WAVE_FORMAT_EXTENSIBLE`
  - Should the user specify when to use format variants, as done in Audacity?
    - This would also apply to AIFF-C
- Add metadata support?
  - Should
  - Requires additional support for RIFF and IFF textual chunks
  - Requires handling of possible ID3 metadata
    - Using `crate rust-id3`
- Improved error messages
  - Refactor `AudioError` and revise messages throughout code
- Improved testing
  - Improve integration tests
  - Add unit tests for better coverage
- Improved examples and documentation
  - Inspect generated docs and apply revisions where needed
  - Add examples in documents
  - Create more example programs
- Miscellaneous
  - Add a "from_buffer" function?
  - Add `load_as` for reading data as a different audio format?
  - Expand API?
    - Export `Container::open` and `Container::create`
  - Come up with a name!
  - Explore other audio formats
