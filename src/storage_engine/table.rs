use std::sync::Arc;

use super::directory::{self, Directory};


pub struct Table {
    table_name: String,
    schema: String, //temp, enum or type parameter?
    file_index: u16,
    indexes: Vec<u16>,
    directory: Arc<Directory>
}


impl Table {
    pub fn new(table_name: String, file_index: u16, directory: Arc<Directory>) -> Self {
        Self {
            table_name,
            file_index,
            directory,
            indexes: Vec::new(),
            schema: "TEST".to_owned()
        }
    }


}