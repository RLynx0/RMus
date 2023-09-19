use clap::Parser;
use nametree::Tree;

use crate::{get::get_files, opt::Opt};

pub use crate::err::{Error, Result};

mod err;
mod get;
mod nametree;
mod opt;

fn main() -> Result<()> {
    let opt = Opt::parse();
    let files = get_files(&opt)?;

    let mut names = Tree::new();
    for file in files {
        names.insert(&file);
    }

    println!("{}", names);

    Ok(())
}
