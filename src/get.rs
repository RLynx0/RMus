use std::{process::Command, rc::Rc, str::FromStr};

use lazy_regex::Regex;

use self::apply_exprs::*;
use crate::{
    err::Error,
    opt::{MatchMode, Opt},
};

mod apply_exprs;

pub fn get_files(opt: &Opt) -> Result<Vec<Rc<str>>, Error> {
    let mut exprs = opt
        .expressions
        .iter()
        .map(|exp| Regex::from_str(&exp))
        .collect::<Result<Vec<_>, _>>()?;
    if !opt.all {
        exprs.insert(0, Regex::from_str(r"\.(mp3|ogg|wav)$")?);
    }
    let mut exprs = exprs.into_iter();

    let base_expr = exprs.next().unwrap_or(Regex::from_str(".*")?);
    let output = locate(base_expr)?;

    let files: Vec<Rc<str>> = output.lines().map(|l| Rc::from(l)).collect();
    Ok(match opt.matchmode {
        MatchMode::All => apply_exprs_conjunctive(files, exprs),
        MatchMode::Any => apply_exprs_disjunctive(files, exprs),
    })
}

fn locate(regex: Regex) -> Result<String, Error> {
    let raw_output = Command::new("locate")
        .args(["--regex", &regex.to_string()])
        .output()?
        .stdout;
    let output = String::from_utf8(raw_output)?;
    Ok(output)
}
