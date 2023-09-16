use std::{io, string::FromUtf8Error};

use lazy_regex::regex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("UTF8 ERROR : {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("IO ERROR : {0}")]
    IoError(#[from] io::Error),

    #[error("REGEX ERROR : {0}")]
    RegexError(#[from] regex::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
