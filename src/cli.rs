use super::clap::{
  Arg,
  App,
  ArgMatches
};
use std::path::PathBuf;
use super::brotli2::stream::{
  CompressParams,
  CompressMode
};

/// Build the input params into a compression mode object
pub fn get_params(x: &ArgMatches) -> (CompressParams,usize) {
  let mut c = CompressParams::new();
  let bs: usize;
  match x.value_of("mode") {
    Option::Some("generic") |
    Option::None => c.mode(CompressMode::Generic),
    Option::Some("text") => c.mode(CompressMode::Text),
    Option::Some("font") => c.mode(CompressMode::Font),
    Option::Some(s) => {
      println!("Illegal value for `-m, --mode` recieved: {}", s);
      ::std::process::exit(1);
    }
  };
  match x.value_of("level") {
    Option::Some(s) => match u32::from_str_radix(s,10) {
      Ok(val) => c.quality(val),
      Err(_) => {
        println!("Illegal value for '-l, --level` received: {}",s);
        ::std::process::exit(1);
      },
    },
    Option::None => unreachable!()
  };
  match x.value_of("window") {
    Option::Some("1k") => c.lgwin(10),
    Option::Some("2k") => c.lgwin(11),
    Option::Some("4k") => c.lgwin(12),
    Option::Some("8k") => c.lgwin(13),
    Option::Some("16k") => c.lgwin(14),
    Option::Some("32k") => c.lgwin(15),
    Option::Some("64k") => c.lgwin(16),
    Option::Some("128k") => c.lgwin(17),
    Option::Some("256k") => c.lgwin(18),
    Option::Some("512k") => c.lgwin(19),
    Option::Some("1m") => c.lgwin(20),
    Option::Some("2m") => c.lgwin(21),
    Option::Some("4m") => c.lgwin(22),
    Option::Some("8m") => c.lgwin(23),
    Option::Some("16m") => c.lgwin(24),
    _ => unreachable!()
  };
  match x.value_of("block") {
    Option::Some("64k") => { bs = 1 << 16; c.lgblock(16); },
    Option::Some("128k") => { bs = 1 << 17; c.lgblock(17); },
    Option::Some("256k") => { bs = 1 << 18; c.lgblock(18); },
    Option::Some("512k") => { bs = 1 << 19; c.lgblock(19); },
    Option::Some("1m") => { bs = 1 << 20; c.lgblock(20); },
    Option::Some("2m") => { bs = 1 << 21; c.lgblock(21); },
    Option::Some("4m") => { bs = 1 << 22; c.lgblock(22); },
    Option::Some("8m") => { bs = 1 << 23; c.lgblock(23); },
    Option::Some("16m") => { bs = 1 << 24; c.lgblock(24); },
    _ => unreachable!()
  };
  (c,bs)
}

/// Are we compressing or decompressing
pub enum Way {
  Compress,
  Decompress
}
pub fn way(x: &ArgMatches) -> Way {
  if x.is_present("decompress") {
    return Way::Decompress;
  }
  if x.is_present("compress") {
    return Way::Compress;
  }
  println!("Neither compress nor decompress were passed.");
  ::std::process::exit(1);
}

/// Reading/Writing
pub enum FileOpt {
  File(PathBuf),
  Magic
}
pub fn read_write(x: &ArgMatches) -> (FileOpt,FileOpt) {
  let i_p = x.is_present("input");
  let o_p = x.is_present("output");
  let sop = x.is_present("stdout");
  if i_p && o_p {
    match x.value_of("input") {
      Option::None => unreachable!(),
      Option::Some(i) => match x.value_of("output") {
        Option::None => unreachable!(),
        Option::Some(o) => {
          return (FileOpt::File(PathBuf::from(i)), FileOpt::File(PathBuf::from(o)));
        }
      }
    };
  }
  if i_p && !sop {
    let pb = PathBuf::from(x.value_of("input").unwrap());
    let opb = match pb.extension() {
      Option::None => {
        let mut opb = pb.clone();
        opb.set_extension(".br");
        opb
      },
      Option::Some(ext) => match ext.to_str() {
        Option::None => {
          println!("Input file {:?} has an invalid extension", &pb);
          ::std::process::exit(1);
        },
        Option::Some(ext) => {
          let mut s = String::with_capacity(ext.len() + 3);
          s.push_str(ext);
          s.push_str(".br");
          let mut opb = pb.clone();
          opb.set_extension(&s);
          opb
        }
      }
    };
    return (FileOpt::File(pb), FileOpt::File(opb));
  }
  if sop && !i_p {
    return (FileOpt::Magic,FileOpt::Magic);
  }
  println!("I'm confused what am I reading and what am I outputing?");
  ::std::process::exit(1);
}


/// Read CLI options
///
/// Creates a CLAP App object which handles reading
/// and parsing the CLI arguments for me
pub fn fetch<'a>() -> ArgMatches<'a> {
  App::new("Bratwurst")
    .version("1.0")
    .author("Cody Laeder, <codylaeder@gmail.com>")
    .about("Brotli based file compression program")
    .arg(Arg::with_name("decompress")
      .short("d")
      .long("decompress")
      .takes_value(false)
      .next_line_help(true)
      .help("Decompress data"))
    .arg(Arg::with_name("compress")
      .short("z")
      .long("compress")
      .takes_value(false)
      .next_line_help(true)
      .help("Compress Data"))
    .arg(Arg::with_name("keep")
      .short("k")
      .long("keep")
      .takes_value(false)
      .next_line_help(true)
      .help("Keep (don't delete) the input file"))
    .arg(Arg::with_name("stdout")
      .short("c")
      .long("stdout")
      .takes_value(false)
      .next_line_help(true)
      .help("Compatibility flag for interacting with GNU utils"))
    .arg(Arg::with_name("force")
      .short("f")
      .long("force")
      .takes_value(false)
      .next_line_help(true)
      .help("Compatibility flag for interacting with GNU utils"))
    .arg(Arg::with_name("threads")
      .short("t")
      .long("threads")
      .takes_value(true)
      .multiple(false)
      .value_name("NUM")
      .next_line_help(true)
      .help("When compressing do so with multiple threads"))
    .arg(Arg::with_name("level")
      .short("l")
      .long("level")
      .takes_value(true)
      .multiple(false)
      .default_value("3")
      .next_line_help(true)
      .possible_values(&[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"])
      .help("Compression Quality, speed vs ratio trade off"))
    .arg(Arg::with_name("mode")
      .short("m")
      .long("mode")
      .takes_value(true)
      .multiple(false)
      .default_value("generic")
      .next_line_help(true)
      .possible_values(&[
        "generic", "text", "font"])
      .help("Compression mode. WOFF and UTF8 text get special treatment"))
    .arg(Arg::with_name("window")
      .short("w")
      .long("window")
      .takes_value(true)
      .multiple(false)
      .default_value("8k")
      .next_line_help(true)
      .possible_values(&[
        "1k", "2k", "4k", "8k", "16k", "32k", "64k", "128k", "256k",
        "512k", "1m", "2m", "4m"])
      .help("Sliding window to find matches on"))
    .arg(Arg::with_name("block")
      .short("b")
      .long("block")
      .takes_value(true)
      .multiple(false)
      .default_value("128k")
      .next_line_help(true)
      .possible_values(&[
        "64k", "128k", "256k", "512k", "1m", "2m", "4m"])
      .help("How large each block is"))
    .arg(Arg::with_name("input")
      .short("i")
      .long("input")
      .value_name("FILE")
      .takes_value(true)
      .multiple(false)
      .next_line_help(true)
      .help("File to compress"))
    .arg(Arg::with_name("output")
      .short("o")
      .long("output")
      .value_name("FILE")
      .takes_value(true)
      .multiple(false)
      .next_line_help(true)
      .help("File that we compressed"))
    .get_matches()
}
