use clap::Parser;

use crate::{get::get_files, opt::Opt, out::output};

pub use crate::err::{Error, Result};

mod err;
mod get;
mod opt;
mod out;

fn main() -> Result<()> {
    let opt = Opt::parse();
    let files = get_files(&opt)?;
    output(files, &opt)
}
