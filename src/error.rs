pub mod image;
use crate::consts::msg;
use std::ffi::OsString;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}: {:?}", msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8, 0)]
    ArgNotConvertibleToUtf8(std::ffi::OsString),
    #[error(transparent)]
    ImageError(#[from] image::Error),
}

impl From<std::ffi::OsString> for Error {
    fn from(err: OsString) -> Self {
        Self::ArgNotConvertibleToUtf8(err)
    }
}
