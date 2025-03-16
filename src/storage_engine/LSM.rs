use std::fmt::Debug;
use super::btree2::{BTree, Key, Val};
use serde::{Serialize, Deserialize};

// B tree to have memory inside of memtable and sstable
// when memtable exceeds certain memory limit that we set. we need to do compaction

// memtable
// on memory B tree

// sstable
// on disk Btree storing storing information

// compaction
// the operation of setting overflown data from memory to disk.  


const MEM_LIMIT: usize = 1024 * 1024 * 10;


#[derive(Serialize)]
pub struct LSMT<K: Key, V: Val> {
    mem_table: BTree<K, V>,
    count: usize
}

impl<K: Key, V: Val> LSMT<K,V> {
    pub fn new() -> Self {
        Self {
            mem_table: BTree::new(),
            count : 0
        }
    }
    pub fn insert(&mut self, key: K, val: V) {
        self.count += 1;
   
        if self.count > MEM_LIMIT {
        // todo this change this to account for memory overflow and send to SStable
            self.insert_sstable();
        }
        self.mem_table.insert(key, val);
    }


    // full table serialization
    // change to node by node serialization later.
    pub fn insert_sstable(&mut self) {
        
        let temp = bincode::serialize(&self).expect("Serialization failed");

        println!("Bytes written: {:?}", temp);
    }   

}

// impl<K: Key,V: Val> Serialize for LSMT<K,V> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
        

//     }
// }

#[cfg(test)]
mod tests {
    use super::LSMT;

    
    #[test]
    pub fn serialization(){
        let mut temp = LSMT::new();

        temp.insert(1, "123");
        temp.insert(1, "123");
        temp.insert(1, "123");
        temp.insert(1, "123");

        temp.insert_sstable();

    }
}
