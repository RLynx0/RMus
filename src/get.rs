use std::{process::Command, rc::Rc};

use regex::{Regex, RegexBuilder};

use crate::{
    err::Result as RmusResult,
    opt::{MatchMode, Opt},
};

pub fn get_files(opt: &Opt) -> RmusResult<Vec<Rc<str>>> {
    let exprs = opt
        .expressions
        .iter()
        .map(|exp| {
            RegexBuilder::new(exp)
                .case_insensitive(opt.case_insensitive)
                .build()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let files = locate(opt)?
        .lines()
        .map(Rc::from)
        .filter(|file| matches_in_opt(&exprs, file, opt))
        .collect();

    Ok(files)
}

fn matches_in_opt(exprs: &[Regex], file: &Rc<str>, opt: &Opt) -> bool {
    exprs
        .iter()
        .map(|expr| expr.is_match(file))
        .reduce(|a, b| match opt.matchmode {
            MatchMode::All => a && b,
            MatchMode::Any => a || b,
        })
        .unwrap_or(true)
}

fn locate(opt: &Opt) -> RmusResult<String> {
    let regex = Regex::new(&opt.pool)?.to_string();

    let mut args = vec![&regex, "--regex"];
    if opt.case_insensitive {
        args.push("-i")
    }

    let raw_output = Command::new("locate").args(args).output()?.stdout;
    Ok(String::from_utf8(raw_output)?)
}
