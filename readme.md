# audio [![Build Status](https://travis-ci.org/brianuosseph/audio.svg?branch=master)](https://travis-ci.org/brianuosseph/audio)
A Rust audio coding library.

## TODO
- Better support for alternative WAVE sample formats
  - When exactly should the encoder write using `WAVE_FORMAT_EXTENSIBLE`? Should the user be able to specify this using a separate codec (like in Audacity)?
- Add support for RIFF and IFF metadata tags
- Integrate `crate rust-id3` for handling ID3 metadata
- Improve integration tests
- Write examples
- Look into using `Container::open` and `Container::create` as part of the public API
- Possibly add a "from_buffer" function
- Possibly add `open_as` and `load_as` for reading data as a different audio format
- Come up with a name!
- Explore other audio formats

## Decoding

| Audio Format | Codec | Data formats |
| ------ | ----- | --------- |
| WAVE | PCM | u8, alaw, ulaw, i16, i24, i32, f32, f64 |
| AIFF | PCM | u8, i8, alaw, ulaw, i16, i24, i32, f32, f64 |

## Encoding

| Audio Format | Codec | Bit Rates |
| ------ | ----- | --------- |
| WAVE | PCM | u8, alaw, ulaw, i16, i24, i32, f32, f64 |
| AIFF | PCM | u8, i8, alaw, ulaw, i16, i24, i32, f32, f64 |
