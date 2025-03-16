use super::directory::*;
use std::collections::HashMap;

// todo: figure out max cache size, eviction strategy, flushing
pub struct PageCache {
    cache: HashMap<FullPageIndex, Box<[u8]>>
}

impl PageCache {
    pub fn read_page(&self, full_page_index: FullPageIndex) -> Option<Box<[u8]>> {
        todo!()
    }
}