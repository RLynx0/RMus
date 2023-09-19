use std::rc::Rc;

use self::nametree::Tree;

mod nametree;

pub fn tree<I>(files: I)
where
    I: IntoIterator<Item = Rc<str>>,
{
    let mut nametree = Tree::new();
    for file in files {
        nametree.insert(&file);
    }

    println!("{}", nametree);
}

pub fn list<I>(files: I)
where
    I: IntoIterator<Item = Rc<str>>,
{
    for file in files {
        println!("{}", file);
    }
}
