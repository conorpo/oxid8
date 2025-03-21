use std::{fs::OpenOptions, io::BufReader, path::PathBuf};
use serde::*;

use super::directory::*;
use super::btree4::*;
struct StorageEngine {
    temp_btree: BTreeMethod,
    directory: Directory
}

impl StorageEngine {
    pub fn new() -> Self {
        Self {
            directory: Directory::new(),
            temp_btree: BTreeMethod {}
        }
    }
}