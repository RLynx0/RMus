use clap::Parser;

use crate::{
    get::get_files,
    opt::{Opt, Output},
};

pub use crate::err::{Error, Result};

mod err;
mod get;
mod opt;
mod out;

fn main() -> Result<()> {
    let opt = Opt::parse();
    let files = get_files(&opt)?;

    match opt.output {
        Output::List => out::list(files),
        Output::Tree => out::tree(files),
    }

    Ok(())
}
