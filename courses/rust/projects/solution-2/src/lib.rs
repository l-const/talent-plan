//! src/lib.rs
#![deny(missing_docs)]
mod error;
mod kvs;
mod ser;
pub use crate::error::Result;
pub use crate::kvs::*;
