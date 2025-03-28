use winapi;

use std::fs::File;
use std::io::{self, Read};
use std::thread::panicking;
use std::{collections::HashMap, path::PathBuf};
use std::os::windows::fs::FileExt;

use super::cache::*;
use super::{PAGE_SIZE, DATA_PATH};
use super::page::*;

// General File Structure?
const HEADER_SIZE: usize = 64;
pub type FileIndex = u16;
pub type PageIndex = u16;
pub type OffsetInPage = usize;


//Make monomorphic over pagesize?
struct Pager {
    page_count: usize,
    file: File
}

impl Pager {
    fn new(file_path: PathBuf) -> Result<Self, io::Error> {
        //custom flags here?
        let file = File::options().write(true).read(true).open(file_path)?;
        
        Ok(Self {
            page_count: 0,
            file
        })
    }

    fn create_page(&mut self) {
        self.page_count += 1;
        self.file.set_len((self.page_count * PAGE_SIZE) as u64).expect("Failed to set len, either because no write permission or overflow.");
    }

    fn read_page(&self, page_index: u16) -> Box<[u8]> {
        let mut buffer = Box::new([0 as u8; PAGE_SIZE]);
        self.file.seek_read(buffer.as_mut(), page_index as u64 * PAGE_SIZE as u64).expect("Failed to read for some reason.");
        buffer
    }
    
    //Maybe make this type checked with a const generic
    fn update_page(&mut self, page_index: u16, data: Box<[u8]>) {
        self.file.seek_write(data.as_ref(), page_index as u64 * PAGE_SIZE as u64).expect("Failed to update page.");
    }
}

// Another problem, having parametric polymorphism
pub struct Directory {
    pagers: Vec<Pager>,
    page_cache: TwoQcache<(FileIndex, PageIndex), SlottedPage<true>>, 
}

impl Directory {
    pub fn new() -> Self {
        Self {
            pagers: Vec::new(),
            page_cache: TwoQcache::new(100) //todo: change
        }
    }

    pub fn register_file(&mut self, file_name: String) -> Result<usize,io::Error> {
        let file_path = PathBuf::from(format!("{DATA_PATH}{file_name}"));
        self.pagers.push(Pager::new(file_path)?);

        Ok(self.pagers.len() - 1)
    }

    // pub fn read_page(&mut self, full_page_index: (FileIndex, PageIndex)) -> Option<&[u8]> {
    //     match self.page_cache.access(&full_page_index) {
    //         Ok(page) => Some(page),
    //         Err(raw_entry) => {
    //             let (file_index, page_index) = full_page_index;
    //             if let Some(page) = self.pagers.get(file_index as usize).map(|pager| {
    //                 pager.read_page(page_index)
    //             }) {
    //                 Some(&**raw_entry.or_insert(full_page_index, page).1)
    //             } else {
    //                 None
    //             }
    //         }
    //     }
    // }

    // pub fn update_page(&mut self, full_page_index: (FileIndex, PageIndex), data: Box<[u8]>) -> Result<(), &'static str> {
    //     if let Some(cached_page) = self.page_cache.get_mut(full_page_index) {
    //         *cached_page = data;

    //         Ok(())
    //     } else {
    //         self.pagers.get_mut(full_page_index.0 as usize).map(|pager| {
    //             pager.update_page(full_page_index.1, data);
    //             ()
    //         }).ok_or("Could not find specified page.")
    //     }
    // }
}