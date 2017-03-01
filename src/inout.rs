

use std::io::prelude::*;
use std::io::{
  self,
  stdin,
  stdout,
  Stdin,
  Stdout,
  BufReader
};
use std::fs::{
  OpenOptions,
  File
};
use std::path::Path;
use super::FileOpt;
use std::mem;

use super::brotli2::stream::{
  CompressParams,
  Compress,
};
use super::brotli2::bufread::{
  BrotliDecoder,
  BrotliEncoder
};

/*
 * Reader:
 *
 *    Handles reading data into the system
 *
 *
 *
 *
 *
 */
pub enum Reader {
  Ile(File),
  IO(Stdin)
}
impl Reader {

  /// Open an item for reading
  pub fn from(x: &FileOpt) -> io::Result<Reader> {
    match x {
      &FileOpt::File(ref p) => Ok(Reader::Ile(OpenOptions::new()
        .read(true).write(false).create(false).open(p)?)),
      &FileOpt::Magic => Ok(Reader::IO(stdin()))
    }
  }
}
impl Read for Reader {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    match self {
      &mut Reader::Ile(ref mut f) => f.read(buf),
      &mut Reader::IO(ref mut i) => i.read(buf)
    }
  }
}

/*
 * Writer:
 *
 *    Handled writing data out of the system
 *
 *
 *
 *
 *
 */
pub enum Writer {
  Ile(File),
  IO(Stdout)
}
impl Writer {
  
  pub fn from(x: &FileOpt) -> io::Result<Writer> {
    match x {
      &FileOpt::File(ref f) => Ok(Writer::Ile(OpenOptions::new()
        .read(false).write(true).create(true).open(f)?)),
      &FileOpt::Magic => Ok(Writer::IO(stdout()))
    }
  }
  
  pub fn write_data(&mut self, x: WriteBuffer) -> io::Result<()> {
    let len = x.data.len();
    let mut r = x.data.len();
    loop {
      let start = len - r;
      match self.write( &x.data[start..len]) {
        Ok(val) => r -= val,
        Err(e) => return Err(e)
      };
      if r == 0 {
        return Ok(());
      }
    }
  }
}
impl Write for Writer {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    match self {
      &mut Writer::Ile(ref mut f) => f.write(buf),
      &mut Writer::IO(ref mut i) => i.write(buf)
    }
  }
  fn flush(&mut self) -> io::Result<()> {
    match self {
      &mut Writer::Ile(ref mut f) => f.flush(),
      &mut Writer::IO(ref mut i) => i.flush()
    }
  }
}


/*
 * WriteBuffer:
 *
 *    Hold data to be written. Hold `rank` field
 *    to keep them in order.
 *
 *
 *
 *
 */
/// Buffer to be written
pub struct WriteBuffer{
  pub rank: usize,
  pub data: Vec<u8>
}
#[inline(always)]
fn wb_sort(x: &WriteBuffer) -> usize {
  x.rank.clone()
}

/*
 * WriteItems
 *
 *    Holds a collection of write buffers
 *    it handles maintaining a priority
 *    queue of the items
 *
 *
 *
 */
/// Buffer of items to write
pub struct WriteItems {
  next: usize,
  data: Vec<WriteBuffer>
}
impl WriteItems {

  /// create
  pub fn new(size: usize) -> WriteItems {
    WriteItems {
      next: ::std::usize::MAX,
      data: Vec::with_capacity(16)
    }
  }

  /// insert an item to write
  pub fn write_item(&mut self, item: WriteBuffer) {
    let rank = wb_sort(&item);
    let x = match self.data.binary_search_by_key(&rank,wb_sort) {
      Ok(_) => panic!("This really can't happen the concurrency is broke"),
      Err(i) => i
    };
    self.data.insert(x, item);
    self.data.sort_by_key(wb_sort);
  }

  /// Check if there is an item to read
  pub fn read_item(&mut self) -> Option<WriteBuffer> {
    let rank = self.next.clone();
    let x = match self.data.binary_search_by_key(&rank,wb_sort) {
      Ok(x) => x,
      Err(_) => return None
    };
    self.next -= 1;
    let val = self.data.remove(x);
    self.data.sort_by_key(wb_sort);
    Some(val)
  }
}

/// Bad
pub enum Fault {
  OS(io::Error),
  Other(String)
}

/*
 * Simple Compressor
 *
 *    Single thread compression jobs only
 *
 *
 *
 *
 *
 */
pub fn comp(
  buf: usize,
  i: &FileOpt,
  o: &FileOpt,
  opts: &CompressParams)
-> io::Result<()> {
  
  //set up phase
  let mut v = Vec::with_capacity(buf);
  unsafe{ v.set_len(buf) };
  
  //build reader/writer
  let mut r = Reader::from(i)?;
  let mut w = Writer::from(o)?;
  
  //configure encoder
  let mut r = BufReader::with_capacity(buf,r);
  let mut encoder = BrotliEncoder::from_params(r, opts);

  loop {
  
    let dist = encoder.read(v.as_mut_slice())?;
    if dist == 0 {
      w.flush()?;
      return Ok(());
    }

    let _ = w.write_all(&v.as_slice()[0..dist])?;
  }
}

/*
 * Simple Decompressor
 *
 *    Single thread decompression jobs only
 *
 *
 *
 *
 *
 */
pub fn decomp(
  buf: usize,
  i: &FileOpt,
  o: &FileOpt)
-> io::Result<()> {
  
  //set up phase
  let mut v = Vec::with_capacity(buf);
  unsafe{ v.set_len(buf) };
  
  //build reader/writer
  let mut r = Reader::from(i)?;
  let mut w = Writer::from(o)?;
  
  //configure encoder
  let mut r = BufReader::with_capacity(buf,r);
  let mut decoder = BrotliDecoder::new(r);

  loop {
  
    let dist = decoder.read(v.as_mut_slice())?;
    if dist == 0 {
      w.flush()?;
      return Ok(());
    }

    let _ = w.write_all(&v.as_slice()[0..dist])?;
  }
}

