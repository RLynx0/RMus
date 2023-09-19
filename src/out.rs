use std::rc::Rc;

use crate::{opt::Opt, Result};

use self::nametree::Tree;

mod nametree;

pub fn output<I>(files: I, opt: &Opt) -> Result<()>
where
    I: IntoIterator<Item = Rc<str>>,
{
    match opt.output {
        crate::opt::Output::List => list(files),
        crate::opt::Output::Tree => print_tree(files),
    }
    Ok(())
}

fn print_tree<I>(files: I)
where
    I: IntoIterator<Item = Rc<str>>,
{
    let mut nametree = Tree::new();
    for file in files {
        nametree.insert(&file);
    }

    println!("{}", nametree);
}

fn list<I>(files: I)
where
    I: IntoIterator<Item = Rc<str>>,
{
    for file in files {
        println!("{}", file);
    }
}
