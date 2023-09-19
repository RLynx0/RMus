use std::rc::Rc;

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

    match opt.output {
        opt::Output::List => list(&files),
        opt::Output::Tree => display_tree(&files),
    }

    Ok(())
}

fn list(files: &Vec<Rc<str>>) {
    for file in files {
        println!("{}", file);
    }
}

fn display_tree(files: &Vec<Rc<str>>) {
    let mut nametree = Tree::new();
    for file in files {
        nametree.insert(file);
    }

    println!("{}", nametree);
}
