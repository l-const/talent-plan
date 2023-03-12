//! src/lib.rs
#![deny(missing_docs)]
mod domain;
mod error;
mod kvs;
pub use crate::error::Result;
pub use crate::kvs::*;
