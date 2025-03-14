use bit_set::BitSet;
use serde::{Serialize, Deserialize};

pub const PAGE_SIZE: usize = 1024 * 32;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PagePointer {
    pub file_id: u16,
    pub page_num: u16
}

impl PagePointer {
    pub fn offset(&self) -> usize {
        PAGE_SIZE * self.page_num as usize
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageHeader {
    typ: u32
}

pub trait Page {

}