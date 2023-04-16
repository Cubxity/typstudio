mod fs;
mod typst;

pub use self::typst::*;
pub use fs::*;

use serde::{Serialize, Serializer};
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown project")]
    UnknownProject,
    #[error("io error occurred")]
    IO(#[from] io::Error),
    #[error("the provided path does not belong to the project")]
    UnrelatedPath,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
