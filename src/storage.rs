use std::{fs::OpenOptions, io::BufReader, path::PathBuf};
use serde::*;

use crate::page;

use super::page::*;

const MAX_PAGES_PER_FILE: usize = const {u16::MAX as usize}; //65536

struct StorageEngine {
    files: Vec<PathBuf>
}

impl StorageEngine {
    pub fn new() -> Self {
        Self {
            files: Vec::new()
        }
    }

    pub fn create_page(&mut self, page: Box<dyn Page>) -> Result<(),()> {
        //todo place in file depending on page type?
        todo!()
    }

    pub fn get_page(&self, page_pointer: PagePointer) -> Box<dyn Page> {
        let file_id = page_pointer.file_id;

        let path = self.files.get(file_id as usize).unwrap();

        let file = OpenOptions::new().create(false).open(path).unwrap();
        let mut reader = BufReader::new(file);
        reader.seek_relative(page_pointer.offset() as i64);
        
        //let page = 
        todo!();
    }
}