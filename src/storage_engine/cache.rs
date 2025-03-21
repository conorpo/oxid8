use super::directory::*;
use std::collections::HashMap;

// todo: figure out max cache size, eviction strategy, flushing
pub struct PageCache {
    cache: HashMap<FullPageIndex, Box<[u8]>>
}

impl PageCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new()
        }
    }

    pub fn read_page(&self, full_page_index: FullPageIndex) -> Option<Box<[u8]>> {
        todo!()
    }

    pub fn get_mut(&mut self, full_page_index: FullPageIndex) -> Option<&mut Box<[u8]>> {
        self.cache.get_mut(&full_page_index)
    }
}