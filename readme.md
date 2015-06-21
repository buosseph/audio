# audio [![Build Status](https://travis-ci.org/brianuosseph/audio.svg?branch=master)](https://travis-ci.org/brianuosseph/audio)
A Rust audio decoding library

## Status

`AIFF` sample rates are stored as an 80 bit floating point number, so in order to read and write the bytes into a supported numeric type the conversion process uses an unstable float operation. Hopefully the operation is stablized by the next release.

## TODO
- Add suppor for other AIFF bit rates
- Optimize, optimize, optimize
- Explore other audio formats (don't hold your breath on this one)

## Decoding
Supports:

| Format | Codec | Bit Rates |
| ------ | ----- | --------- |
| WAVE | PCM | 8, 16, 24, 32 |
| AIFF | PCM | 16 |

## Encoding
Support: 

| Format | Codec | Bit Rates |
| ------ | ----- | --------- |
| WAVE | PCM | 8, 16, 24, 32 |
| AIFF | PCM | 16 |
