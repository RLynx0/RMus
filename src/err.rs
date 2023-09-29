use std::{io, string::FromUtf8Error};

use rodio::{decoder::DecoderError, PlayError, StreamError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("UTF8 ERROR : {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("IO ERROR : {0}")]
    IoError(#[from] io::Error),

    #[error("REGEX ERROR : {0}")]
    RegexError(#[from] regex::Error),

    #[error("STREAM ERROR : {0}")]
    StreamError(#[from] StreamError),

    #[error("DECODER ERROR : {0}")]
    DecoderError(#[from] DecoderError),

    #[error("PLAY ERROR : {0}")]
    PlayError(#[from] PlayError),
}

pub type Result<T> = std::result::Result<T, Error>;
