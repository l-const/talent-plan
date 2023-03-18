use serde::{de, ser};

use std::fmt::{self, Display};


pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(ref msg) => write!(f, "{}", msg),
        }
    }
}

impl ser::Error for Error {
    fn custom<T>(msg:T) -> Self where T:Display {
        Error::Message(msg.to_string())
    }
}