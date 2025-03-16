use std::{collections::HashMap, path::PathBuf};
use super::cache::*;
use super::{PAGE_SIZE};

// General File Structure?
const HEADER_SIZE: usize = 64;

pub type FullPageIndex = (u16, u16);
struct Pager {
    page_size: usize,
    file_path: PathBuf
}

impl Pager {
    fn new(file_path: PathBuf, page_size: usize) -> Self {
        Self {
            page_size,
            file_path
        }
    }

    fn create_page(&mut self) {
        
    }

    fn read_page(&self, page_index: u16) -> Box<[u8]> {
        todo!()
    }
    
    //Maybe make this type checked with a const generic
    fn update_page(&mut self, page_index: u16, data: Box<[u8]>) {
        debug_assert_eq!(data.len(), self.page_size);
    }
}

struct Directory {
    pagers: Vec<Pager>,
    page_cache: PageCache
}

impl Directory {
    pub fn register_file(&mut self, file_path: PathBuf, page_size: usize) {
        self.pagers.push(Pager::new(file_path, page_size));
    }

    pub fn read_page(&self, full_page_index: FullPageIndex) -> Option<Box<[u8]>> {
        self.page_cache.read_page(full_page_index).or_else(|| {
            let (file_index, page_index) = full_page_index;
            self.pagers.get(file_index as usize).map(|pager| {
                pager.read_page(page_index)
            })
        })    
    }

    // pub fn update_page(&mut self, full_page_index: FullPageIndex) {
        
    // }
}