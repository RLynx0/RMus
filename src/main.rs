use clap::Parser;
use get::get_files;

use crate::opt::Opt;

pub use crate::err::{Error, Result};

mod err;
mod get;
mod opt;

fn main() -> Result<()> {
    let opt = Opt::parse();
    let files = get_files(&opt)?;
    for file in files {
        println!("{file}");
    }
    Ok(())
}
