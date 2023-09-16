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

    let names = files
        .iter()
        // SAFETY: there cannot be empty lines
        .map(|f| f.split('/').last().unwrap())
        .collect::<Vec<_>>();

    for name in names {
        println!("{}", name);
    }

    Ok(())
}
