//!  defines logging
use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}
/// Key-value Store
pub struct KvStore {
    path: PathBuf,
}

impl KvStore {
    /// Open active file given path.
    pub fn open<T: Into<PathBuf>>(path: T) -> Result<Self> {
        Ok(Self { path: path.into() })
    }
    ///  Set the value of a string key to a string. Return an error if the value is not written successfully.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        Ok(())
    }
    ///  Remove a given key. Return an error if the key does not exist or is not removed successfully.
    pub fn remove(&mut self, key: String) -> Result<()> {
        Ok(())
    }
    /// Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(None)
    }
}
