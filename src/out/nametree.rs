use std::{collections::HashMap, fmt::Display, rc::Rc};

#[derive(Clone, Debug)]
pub struct Tree {
    branches: HashMap<Rc<str>, Tree>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            branches: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: &str) {
        let path = path.split('/').map(Rc::from);
        self.insert_iter(path)
    }

    fn insert_iter<I>(&mut self, path: I)
    where
        I: IntoIterator<Item = Rc<str>>,
    {
        let mut iter = path.into_iter();
        let name = match iter.next() {
            Some(name) => name,
            None => return,
        };

        match self.branches.get_mut(&name) {
            Some(tree) => tree.insert_iter(iter),
            None => {
                self.branches.insert(name, Tree::from_iter(iter));
            }
        }
    }
}

impl FromIterator<Rc<str>> for Tree {
    fn from_iter<T: IntoIterator<Item = Rc<str>>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let branches = match iter.next() {
            Some(name) => HashMap::from([(name, Tree::from_iter(iter))]),
            None => HashMap::new(),
        };
        Tree { branches }
    }
}

enum DisplayPlace {
    Item,
    Last,
    Appended,
}

const PRE_LINE: &str = "│  ";
const PRE_ITEM: &str = "├╴ ";
const PRE_LAST: &str = "╰╴ ";
const PRE_EMPT: &str = "   ";

impl Tree {
    fn display_root(&self, preprint: &str) -> String {
        let root_length = self.branches.len();
        self.branches
            .iter()
            .enumerate()
            .map(|(index, (name, branch))| {
                branch.display_branch(
                    // FMT: -
                    root_length,
                    index,
                    preprint,
                    name,
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn display_branch(
        &self,
        root_len: usize,
        index: usize,
        preprint: &str,
        name: &Rc<str>,
    ) -> String {
        let len = self.branches.len();
        let placement = compute_placement(root_len, index);
        let child_preprint = compute_child_preprint(preprint, &placement);
        let preprint = compute_preprint(preprint, &placement);
        let name_separator = compute_name_separator(len);

        format!(
            "{}{}{}{}",
            preprint,
            name,
            name_separator,
            self.display_root(&child_preprint)
        )
    }
}

fn compute_placement(root_len: usize, index: usize) -> DisplayPlace {
    use DisplayPlace::*;
    let last_index = root_len.saturating_sub(1);
    match (root_len, index) {
        (0 | 1, _) => Appended,
        (_, i) if i == last_index => Last,
        _ => Item,
    }
}

fn compute_child_preprint(preprint: &str, placement: &DisplayPlace) -> String {
    use DisplayPlace::*;
    match placement {
        Item => format!("{}{}", preprint, PRE_LINE),
        Last => format!("{}{}", preprint, PRE_EMPT),
        Appended => preprint.to_string(),
    }
}

fn compute_preprint(preprint: &str, placement: &DisplayPlace) -> String {
    use DisplayPlace::*;
    match placement {
        Item => format!("{}{}", preprint, PRE_ITEM),
        Last => format!("{}{}", preprint, PRE_LAST),
        Appended => String::new(),
    }
}

fn compute_name_separator(len: usize) -> &'static str {
    match len {
        0 => "",
        1 => "/",
        _ => "/\n",
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_root(""))
    }
}
