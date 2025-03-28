use std::{cell::Cell, mem};

enum NodeType {
    Leaf,
    Internal
}

//If pages are less than 64KB, u16 is fine for offsets / size
#[derive(Clone, Copy)]
struct CellPointer {
    offset: u16,
    cell_size: u16,
}



pub struct SlottedPage<const SORT: bool> { 
    page_type: NodeType,
    free_end_offset: u16,
    pointers: Vec<Option<CellPointer>>,
    data: Box<[u8]>
}

impl<const SORT: bool> SlottedPage<SORT> {
    pub fn get(&self, index: usize) -> Option<&[u8]> {
        let CellPointer { offset, cell_size } =  self.pointers[index]?;// todo should we panic on invalid index

        let (l,r) = ((offset - cell_size) as usize, offset as usize);

        Some(&self.data[l..r])
    }
}

impl SlottedPage<false> {
    pub fn new() { //where is this called, after a page has been created in a file? before?
        
    }

    pub fn insert(&mut self, record: &[u8]) -> usize {
        let cell_size = record.len() as u16;

        if self.free_end_offset < cell_size {
            todo!();
            // self.vacuum();
            // or overflow pages
            // or dynamic node capacity
        }
        self.free_end_offset -= cell_size;

        let cell = &mut self.data[(self.free_end_offset as usize)..((self.free_end_offset + cell_size) as usize)];
        cell.copy_from_slice(record);

        let cell_pointer = CellPointer {
            offset: self.free_end_offset + cell_size,
            cell_size: cell_size
        };

        self.pointers.push(Some(cell_pointer));
        self.pointers.len() - 1
    }

    pub fn remove(&mut self, index: usize) {
        self.pointers[index] = None; 
    }

    //naive solution
    pub fn compact(&mut self) {
        let mut cur_offset = self.data.len();

        for cell_pointer in self.pointers.iter().copied() {
            if let Some(CellPointer { offset, cell_size }) = cell_pointer {
                let offset = offset as usize;
                let cell_size = cell_size  as usize;
                if offset < cur_offset {
                    cur_offset -= cell_size;
                    self.data.copy_within((offset - cell_size as usize)..offset, cur_offset);
                }
            }
        }

        self.free_end_offset = cur_offset as u16;

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    pub fn test_basics(){
        const CAPACITY: usize = 200;
        
        let mut page: SlottedPage::<false> = SlottedPage {
            page_type: NodeType::Leaf,
            free_end_offset: CAPACITY as u16,
            pointers: Vec::new(),
            data: Box::new([0; CAPACITY])
        };

        let capacity: u16 = CAPACITY as u16;

        assert_eq!(page.free_end_offset, capacity);
        
        let record = "samanthas tits".as_bytes().to_owned();
        let index = page.insert(&record); 
        assert_eq!(page.get(index), Some(&record[..]));
        assert_eq!(page.free_end_offset, capacity - record.len() as u16);
        
        let record2 = "ass".as_bytes().to_owned();
        let index2 = page.insert(&record2); 
        assert_eq!(page.get(index2), Some(&record2[..]));
        assert_eq!(page.free_end_offset, capacity - (record2.len() as u16 + record.len() as u16));

        page.remove(index);
        assert_eq!(page.get(index), None);
        assert_eq!(page.get(index2), Some(&record2[..]));
        assert_eq!(page.free_end_offset, capacity - (record2.len() as u16 + record.len() as u16));

        page.compact();
        assert_eq!(page.get(index), None);
        assert_eq!(page.get(index2), Some(&record2[..]));
        assert_eq!(page.free_end_offset, capacity - (record2.len() as u16));
    }
}