use std::path::PathBuf;

const PAGE_SIZE: usize = 16 * 1024;
const DATA_PATH: &str = "./data/";


//pub mod LSM;
pub mod btree4;
pub mod cache;
pub mod directory;
pub mod storage;
pub mod table;
