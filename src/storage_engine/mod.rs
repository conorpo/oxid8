use std::path::PathBuf;

const PAGE_SIZE: usize = 16 * 1024;
const DATA_PATH: &str = "./data/";

pub mod doubly_linked;

//pub mod LSM;
pub mod directory;
pub mod btree4;
pub mod cache;
pub mod storage;
pub mod table;