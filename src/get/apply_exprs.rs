use std::rc::Rc;

use lazy_regex::Regex;

pub fn apply_exprs_conjunctive<I>(mut files: Vec<Rc<str>>, exprs: I) -> Vec<Rc<str>>
where
    I: IntoIterator<Item = Regex>,
{
    for expr in exprs {
        files = files
            .into_iter()
            .filter(|file| expr.is_match(file))
            .collect()
    }

    files
}

pub fn apply_exprs_disjunctive<I>(files: Vec<Rc<str>>, exprs: I) -> Vec<Rc<str>>
where
    I: IntoIterator<Item = Regex>,
{
    let exprs: Vec<Regex> = exprs.into_iter().collect();
    files
        .into_iter()
        .filter(|file| matches_any(file, exprs.clone()))
        .collect()
}

fn matches_any<I>(file: &str, exprs: I) -> bool
where
    I: IntoIterator<Item = Regex>,
{
    exprs
        .into_iter()
        .map(|expr| expr.is_match(file))
        .reduce(|a, b| a || b)
        .unwrap_or(true)
}
