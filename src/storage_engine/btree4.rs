use serde::{Deserialize, Serialize};
use bincode;

use super::directory::*;
use std::{fmt::Debug, marker::PhantomData};

//Not sure if we will stay with this interface style

pub trait Key = Ord + Debug + Copy;
pub trait Val = Debug + Copy;

#[derive(Deserialize, Serialize, Copy, Clone)]
enum NodeType {
    Internal,
    Leaf
}

impl NodeType {
    pub fn is_leaf(self) -> bool {
        match self {
            NodeType::Internal => false,
            NodeType::Leaf => true
        }
    }
}

const NODE_TYPE_SIZE: usize = std::mem::size_of::<NodeType>();

pub struct BTreeIndex<'a, K: Key> {
    directory: &'a Directory,
    file_index: FileIndex, // u16
    target_file_index: FileIndex, //u16
    _boo: PhantomData<K>
}

impl<'a, K: Key> BTreeIndex<'a, K> {
    pub fn new(file_index: u16, directory: &'a Directory, target_file_index: u16) -> Self {
        Self {
            _boo: PhantomData,
            file_index,
            target_file_index,
            directory
        }
    }

    fn search_internal(&self, key: K, data: &[u8]) -> PageIndex {
        todo!()
    }


    fn search_leaf(&self, key: K, data: &[u8]) -> Option<(PageIndex, OffsetInPage)> {
        todo!()
    }

    pub fn search(&self, key: K) -> Option<(PageIndex, OffsetInPage)> {
        let mut page = self.directory.read_page((self.file_index, 0)).expect("Root missing.");

        while let NodeType::Internal = bincode::deserialize::<NodeType>(&page[..NODE_TYPE_SIZE]).expect("Failed to deserialize node type.") {
            let next_page = self.search_internal(key, &page[NODE_TYPE_SIZE..]);
            page = self.directory.read_page((self.file_index, next_page)).expect("Page missing.");
        }

        debug_assert!(bincode::deserialize::<NodeType>(&page[..NODE_TYPE_SIZE]).unwrap().is_leaf());
        self.search_leaf(key, &page[..NODE_TYPE_SIZE])
    }

    pub fn insert(&mut self, key: K, val: (PageIndex, OffsetInPage)) -> Result<(), &'static str> {
        todo!()
    }
}

