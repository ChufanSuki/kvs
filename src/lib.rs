#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use kvs::KvStore;

mod error;
mod kvs;
