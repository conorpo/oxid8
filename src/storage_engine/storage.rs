use std::{fs::OpenOptions, io::BufReader, path::PathBuf};
use serde::*;

use std::io;

use super::directory;
use super::directory::*;
use super::btree4::*;
use super::table::*;

// Tables, Indexes
struct StorageEngine {
    tables: Vec<Table>, //temp
    directory: Directory
}

impl StorageEngine {
    pub fn new() -> Self {
        Self {
            directory: Directory::new(),
            tables: Vec::new(),
        }
    }

    //todo this shouldnt take a mut self
    pub fn new_table(&mut self, table_name: String) -> Result<(), io::Error> {
        let file_index = self.directory.register_file(format!("_{table_name}.data"))?;
        let table = Table::new(table_name, )
    }

    
}