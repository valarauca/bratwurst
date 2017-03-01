extern crate clap;
extern crate brotli2;

pub use brotli2::stream::CompressMode;

mod cli;
pub use self::cli::{
  Way,FileOpt
};

mod inout;

fn main() {

  let args = cli::fetch();
  let fops = cli::read_write(&args);
  let (cmpopts, buff) = cli::get_params(&args);
  let way = cli::way(&args);
  match way {
    Way::Decompress => match inout::decomp(buff,&fops.0,&fops.1) {
      Ok(_) => { },
      Err(e) => {
        println!("Error occured while decompressing {:?}", e);
        ::std::process::exit(1);
      }
    },
    Way::Compress => match inout::comp(buff,&fops.0,&fops.1,&cmpopts) {
      Ok(_) => { },
      Err(e) => {
        println!("Error occured while compressing {:?}", e);
        ::std::process::exit(1);
      }
    },
  };
}
