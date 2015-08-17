// This benchmark is for testing implementation approaches,
// not the library itself.

#![feature(test)]
extern crate audio;
extern crate byteorder;
extern crate test;

#[cfg(test)]
mod read {
  use std::fs::File;
  use std::io::{BufReader, Read};
  use std::path::Path;
  use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
  use test::Bencher;

  // This is one approach to reading a track without the abstractions
  // and error checking provided in the library. This is for testing
  // general reading approaches for optimal performance.
  #[allow(unused_must_use, unused_variables)]
  #[bench]
  fn read_wave_track_with_vec(b: &mut Bencher) {
    let path = &Path::new("tests/wav/Warrior Concerto - no meta.wav");
    b.iter(|| {
      let mut file = File::open(&path).ok().expect("Couldn't open file");
      let mut riff_header = [0u8; 12];
      file.read(&mut riff_header);
      let file_size = LittleEndian::read_u32(&riff_header[4..8]);
      // let mut wave_buffer = vec![0u8; file_size as usize];
      // file.read(&mut wave_buffer);

      // fmt chunk
      let mut chunk_header = [0u8; 8];
      file.read(&mut chunk_header);
      let mut chunk_size = LittleEndian::read_i32(&chunk_header[4..8]);
      let mut chunk = vec![0u8; chunk_size as usize];
      file.read(&mut chunk);

      // data chunk
      file.read(&mut chunk_header);
      chunk_size = LittleEndian::read_i32(&chunk_header[4..8]);
      chunk = vec![0u8; chunk_size as usize];
      file.read(&mut chunk);

      // Convert data to f32 samples
      let num_samples = (chunk_size as usize) / 2;
      let mut samples = vec![0f32; num_samples];
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = (LittleEndian::read_i16(&chunk[2 * i .. 2 * i + 2])) as f32 / 32_768f32;
      }

      // REALLY SLOW
      // let num_samples = (chunk_size as usize) / 2;
      // let mut samples = vec![0f32; num_samples];
      // for (i, sample) in samples.iter_mut().enumerate() {
      //   let int_val = file.read_i16::<LittleEndian>().ok().unwrap();
      //   *sample = int_val as f32 / 32_768f32;
      // }

      // Incorrect, but very fast
      // let num_samples = (chunk_size as usize) / 2;
      // let mut samples = vec![0f32; num_samples];
      // samples.iter_mut().map(|s| {
      //   *s = (file.read_i16::<LittleEndian>().ok().unwrap() as f32 / 32_768f32)
      // });

      // Validation
      // println!("{:?}", samples[0]);                  // 0
      // println!("{:?}", samples[1]);                  // 0.000030517578125
      // println!("{:?}", samples[samples.len() - 1]);  // 0.000091552734375
    });
  }

  // This is one approach to reading a track without the abstractions
  // and error checking provided in the library. This is for testing
  // general reading approaches for optimal performance.
  #[allow(unused_must_use, unused_variables)]
  #[bench]
  fn read_wave_track_with_cursor_vec(b: &mut Bencher) {
    use std::io::{Cursor, Seek, SeekFrom};
    let path = &Path::new("tests/wav/Warrior Concerto - no meta.wav");
    b.iter(|| {
      let mut file = File::open(&path).ok().expect("Couldn't open file");
      let mut riff_header = [0u8; 12];
      file.read(&mut riff_header);
      let file_size = LittleEndian::read_u32(&riff_header[4..8]);
      let mut wave_buffer = Cursor::new(vec![0u8; file_size as usize]);
      file.read(wave_buffer.get_mut());

      // fmt chunk
      let mut chunk_header = [0u8; 8];
      wave_buffer.read(&mut chunk_header);
      let mut chunk_size = LittleEndian::read_i32(&chunk_header[4..8]);
      let mut pos: usize = wave_buffer.position() as usize;
      {
        let chunk: &[u8] = &(wave_buffer.get_ref()[pos .. pos + chunk_size as usize]);
      }
      wave_buffer.seek(SeekFrom::Current(chunk_size as i64));

      // data chunk
      wave_buffer.read(&mut chunk_header);
      chunk_size = LittleEndian::read_i32(&chunk_header[4..8]);
      pos = wave_buffer.position() as usize;
      let num_samples = (chunk_size as usize) / 2;
      let mut samples = vec![0f32; num_samples];
      {
        let chunk: &[u8] = &(wave_buffer.get_ref()[pos .. pos + chunk_size as usize]);
        // Convert data to f32 samples
        for (i, sample) in samples.iter_mut().enumerate() {
          *sample = (LittleEndian::read_i16(&chunk[2 * i .. 2 * i + 2])) as f32 / 32_768f32;
        }
      }
      wave_buffer.seek(SeekFrom::Current(chunk_size as i64));

      // REALLY SLOW
      // let num_samples = (chunk_size as usize) / 2;
      // let mut samples = vec![0f32; num_samples];
      // for (i, sample) in samples.iter_mut().enumerate() {
      //   let int_val = file.read_i16::<LittleEndian>().ok().unwrap();
      //   *sample = int_val as f32 / 32_768f32;
      // }

      // Incorrect, but very fast
      // let num_samples = (chunk_size as usize) / 2;
      // let mut samples = vec![0f32; num_samples];
      // samples.iter_mut().map(|s| {
      //   *s = (file.read_i16::<LittleEndian>().ok().unwrap() as f32 / 32_768f32)
      // });

      // Validation
      // println!("{:?}", samples[0]);                  // 0
      // println!("{:?}", samples[1]);                  // 0.000030517578125
      // println!("{:?}", samples[samples.len() - 1]);  // 0.000091552734375
    });
  }

  // This is one approach to reading a track without the abstractions
  // and error checking provided in the library. This is for testing
  // general reading approaches for optimal performance.
  #[allow(unused_must_use, unused_variables)]
  #[bench]
  fn read_wave_track_with_bufreader(b: &mut Bencher) {
    let path = &Path::new("tests/wav/Warrior Concerto - no meta.wav");
    b.iter(|| {
      let file = File::open(&path).ok().expect("Couldn't open file");
      let mut buf_reader = BufReader::new(file);
      // riff header
      let mut riff_header = [0u8; 12];
      buf_reader.read(&mut riff_header);
      // fmt chunk
      let mut chunk_header = [0u8; 8];
      buf_reader.read(&mut chunk_header);
      let mut chunk_size = LittleEndian::read_i32(&chunk_header[4..8]);
      let mut chunk = vec![0u8; chunk_size as usize];
      buf_reader.read(&mut chunk);
      // data chunk
      buf_reader.read(&mut chunk_header);
      chunk_size = LittleEndian::read_i32(&chunk_header[4..8]);
      chunk = vec![0u8; chunk_size as usize];

      // This approach is correct, but twice as slow since data is being read byte at a time
      // let mut i = 0;
      // let mut sample_bytes = [0u8; 2];
      // while i < (chunk_size as usize) / 2 {
      //   buf_reader.read(&mut sample_bytes);
      //   chunk[2 * i] = sample_bytes[0];
      //   chunk[2 * i + 1] = sample_bytes[1];
      //   i += 1;
      // }
      // let num_samples = i;

      // Could use read_to_end(), but reads to EOF, not end of buffer size
      // buf_reader.read_to_end(&mut chunk);
      // let num_samples = chunk.len() / 2;

      // This approach is incorrect, but is just as fast a vec impl
      let mut num_bytes = 0;
      // Bug, last buffer isn't read. End of chunk data is still all zeros
      while let Ok(bytes_read) = buf_reader.read(&mut chunk) {
        if bytes_read == 0 || num_bytes + bytes_read > chunk.len() { break; }
        num_bytes += bytes_read;
      }
      // Deal with remaining bytes... how?
      assert_eq!(chunk_size as usize, num_bytes);
      let num_samples = num_bytes / 2;

      // Convert data to f32 samples
      let mut samples = vec![0f32; num_samples];
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = (LittleEndian::read_i16(&chunk[2 * i .. 2 * i + 2])) as f32 / 32_768f32;
      }

      // Validation
      // println!("{:?}", samples[0]); // 0
      // println!("{:?}", samples[1]); // 0.000030517578125
      // println!("{:?}", samples[samples.len() - 1]); // 0.000091552734375
    });
  }
}

#[cfg(test)]
mod write {
  use std::fs::File;
  use std::io::{BufWriter, Write};
  use std::path::Path;
  use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
  use super::audio;
  use test::Bencher;

  // This is one approach to reading a track without the abstractions
  // and error checking provided in the library. This is for testing
  // general reading approaches for optimal performance.
  #[allow(unused_must_use, unused_variables)]
  #[bench]
  fn write_wave_track_with_vec_push(b: &mut Bencher) {
    let audio = audio::open(
      &Path::new("tests/wav/Warrior Concerto - no meta.wav")
    ).ok().expect("Couldn't read file");
    let path = &Path::new("tests/results/tmp_i16.wav");
    b.iter(|| {
      // Convert samples to bytes (~10,000,000ns)
      let num_bytes = audio.samples.len() * 2;
      let mut bytes = vec![0u8; num_bytes];
      for (i, sample) in audio.samples.iter().enumerate() {
        LittleEndian::write_i16(&mut bytes[2 * i .. 2 * i + 2], (sample * 32_768f32) as i16);
      }
      // println!("{:?}", bytes[bytes.len() - 2]); // 3
      // println!("{:?}", bytes[bytes.len() - 1]); // 0

      // Calculate values for writing
      let header_size     : u32     = 44;
      let fmt_chunk_size  : u32     = 16;
      let total_bytes     : u32     = 12 
                                    + (8 + fmt_chunk_size)
                                    + (8 + (audio.samples.len() as u32 * audio.bit_rate / 8));
      let mut file = File::create(&path).ok().expect("Couldn't create file");
      // Create riff header
      let mut riff_header = Vec::with_capacity(12);
      riff_header.write(b"RIFF");
      riff_header.write_u32::<LittleEndian>(total_bytes - 8);
      riff_header.write(b"WAVE");
      // Create fmt chunk
      let mut fmt_chunk = Vec::with_capacity((fmt_chunk_size as usize) + 8);
      fmt_chunk.write(b"fmt ");
      fmt_chunk.write_u32::<LittleEndian>(fmt_chunk_size);
      fmt_chunk.write_u16::<LittleEndian>(1u16); // Always LPCM
      fmt_chunk.write_u16::<LittleEndian>(audio.channels as u16);
      fmt_chunk.write_u32::<LittleEndian>(audio.sample_rate as u32);
      fmt_chunk.write_u32::<LittleEndian>(audio.sample_rate * audio.channels * audio.bit_rate / 8u32);
      fmt_chunk.write_u16::<LittleEndian>((audio.channels * audio.bit_rate / 8u32) as u16);
      fmt_chunk.write_u16::<LittleEndian>(audio.bit_rate as u16);
      // Create data chunk
      let mut data_chunk = Vec::with_capacity(num_bytes + 8);
      data_chunk.write(b"data");
      data_chunk.write_u32::<LittleEndian>(num_bytes as u32);
      data_chunk.write(&bytes);

      // // Write to file
      file.write(&riff_header);
      file.write(&fmt_chunk);
      file.write(&data_chunk);
      // file.flush();
    });
  }

  // This is one approach to reading a track without the abstractions
  // and error checking provided in the library. This is for testing
  // general reading approaches for optimal performance.
  #[allow(unused_must_use, unused_variables)]
  #[bench]
  fn write_wave_track_with_vec_slice(b: &mut Bencher) {
    let audio = audio::open(
      &Path::new("tests/wav/Warrior Concerto - no meta.wav")
    ).ok().expect("Couldn't read file");
    let path = &Path::new("tests/results/tmp_i16.wav");
    b.iter(|| {
      // Convert samples to bytes (~10,000,000ns)
      let num_bytes = audio.samples.len() * 2;
      let mut bytes = vec![0u8; num_bytes];
      for (i, sample) in audio.samples.iter().enumerate() {
        LittleEndian::write_i16(&mut bytes[2 * i .. 2 * i + 2], (sample * 32_768f32) as i16);
      }
      // println!("{:?}", bytes[bytes.len() - 2]); // 3
      // println!("{:?}", bytes[bytes.len() - 1]); // 0

      // Calculate values for writing
      let header_size     : u32     = 44;
      let fmt_chunk_size  : u32     = 16;
      let total_bytes     : u32     = 12 
                                    + (8 + fmt_chunk_size)
                                    + (8 + (audio.samples.len() as u32 * audio.bit_rate / 8));
      let mut file = File::create(&path).ok().expect("Couldn't create file");
      // Create riff header
      let mut riff_header = vec![0u8; 12];
      BigEndian::write_u32(&mut riff_header[0..4], 0x52494646);
      LittleEndian::write_u32(&mut riff_header[4..8], total_bytes - 8);
      BigEndian::write_u32(&mut riff_header[8..12], 0x57415645);
      // Create fmt chunk
      let mut fmt_chunk = vec![0u8; (fmt_chunk_size as usize) + 8];
      BigEndian::write_u32(&mut fmt_chunk[0..4], 0x666D7420);
      LittleEndian::write_u32(&mut fmt_chunk[4..8], fmt_chunk_size);
      LittleEndian::write_u16(&mut fmt_chunk[8..10], 1u16); // Always LPCM
      LittleEndian::write_u16(&mut fmt_chunk[10..12], audio.channels as u16);
      LittleEndian::write_u32(&mut fmt_chunk[12..16], audio.sample_rate as u32);
      LittleEndian::write_u32(&mut fmt_chunk[16..20], audio.sample_rate * audio.channels * audio.bit_rate / 8u32);
      LittleEndian::write_u16(&mut fmt_chunk[20..22], (audio.channels * audio.bit_rate / 8u32) as u16);
      LittleEndian::write_u16(&mut fmt_chunk[22..24], audio.bit_rate as u16);
      // Create data chunk
      let mut data_chunk = vec![0u8; num_bytes + 8];
      BigEndian::write_u32(&mut data_chunk[0..4], 0x64617461);
      LittleEndian::write_u32(&mut data_chunk[4..8], num_bytes as u32);
      for (i, byte) in data_chunk.iter_mut().skip(8).enumerate() {
        *byte = bytes[i];
      }
      
      // Write to file
      file.write(&riff_header);
      file.write(&fmt_chunk);
      file.write(&data_chunk);  // MAJOR performance drain here
      // file.flush();
    });
  }

  // This is one approach to reading a track without the abstractions
  // and error checking provided in the library. This is for testing
  // general reading approaches for optimal performance.
  #[allow(unused_must_use, unused_variables)]
  #[bench]
  fn write_wave_track_with_bufwriter(b: &mut Bencher) {
    let audio = audio::open(
      &Path::new("tests/wav/Warrior Concerto - no meta.wav")
    ).ok().expect("Couldn't read file");
    let path = &Path::new("tests/results/tmp_i16.wav");
    b.iter(|| {
      // Convert samples to bytes (~10,000,000ns)
      let num_bytes = audio.samples.len() * 2;
      let mut bytes = vec![0u8; num_bytes];
      for (i, sample) in audio.samples.iter().enumerate() {
        LittleEndian::write_i16(&mut bytes[2 * i .. 2 * i + 2], (sample * 32_768f32) as i16);
      }
      // println!("{:?}", bytes[bytes.len() - 2]); // 3
      // println!("{:?}", bytes[bytes.len() - 1]); // 0

      // Calculate values for writing
      let header_size     : u32     = 44;
      let fmt_chunk_size  : u32     = 16;
      let total_bytes     : u32     = 12 
                                    + (8 + fmt_chunk_size)
                                    + (8 + (audio.samples.len() as u32 * audio.bit_rate / 8));

      // Create BufWriter and write data
      let file = File::create(&path).ok().expect("Couldn't create file");
      let mut buf_writer = BufWriter::new(file);
      // Write riff header
      buf_writer.write(b"RIFF");
      buf_writer.write_u32::<LittleEndian>(total_bytes - 8);
      buf_writer.write(b"WAVE");
      // Write fmt chunk
      buf_writer.write(b"fmt ");
      buf_writer.write_u32::<LittleEndian>(fmt_chunk_size);
      buf_writer.write_u16::<LittleEndian>(1u16); // Always LPCM
      buf_writer.write_u16::<LittleEndian>(audio.channels as u16);
      buf_writer.write_u32::<LittleEndian>(audio.sample_rate as u32);
      buf_writer.write_u32::<LittleEndian>(audio.sample_rate * audio.channels * audio.bit_rate / 8u32);
      buf_writer.write_u16::<LittleEndian>((audio.channels * audio.bit_rate / 8u32) as u16);
      buf_writer.write_u16::<LittleEndian>(audio.bit_rate as u16);
      // Write data chunk
      buf_writer.write(b"data");
      buf_writer.write_u32::<LittleEndian>(num_bytes as u32);
      buf_writer.write_all(&bytes);
    });
  }
}
