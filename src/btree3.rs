use super::page::{PAGE_SIZE, PageHeader};
use core::panic;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Display, Write};
use std::rc::Rc;
use std::usize::MAX;

// TODO BTree
// figure out that issue we solved by inserting 0
// root node maybe shouldnt be an option

const MAX_KEYS: usize = const {
    let key_and_child = size_of::<(u32, NodeRef)>();
    (PAGE_SIZE - size_of::<PageHeader>())  / key_and_child
};
// const MAX_KEYS: usize = 5;

#[derive(Debug)]
enum NodeRef {
    Internal(Box<InternalNode>),
    Leaf(Rc<RefCell<LeafNode>>)
}

#[derive(Debug)]
struct InternalNode {
    data: Vec<(u32, NodeRef)>,
}

#[derive(Debug)]
struct LeafNode {
    data: Vec<(u32, char)>,
    next_leaf: Option<Rc<RefCell<LeafNode>>>,
    dirty: bool
}

impl InternalNode {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(MAX_KEYS),
        }
    }

    pub fn insert(&mut self, key: u32, val: char) -> Result<(), (NodeRef, u32)> {
        let index = self.data.binary_search_by(|probe| {
            probe.0.cmp(&key)
        }).expect_err("Key already exists") - 1;

        let res = match &mut self.data[index].1 {
            NodeRef::Internal(the_box) => the_box.insert(key, val),
            NodeRef::Leaf(rc) => rc.borrow_mut().insert_dirty(key, val)
        };

        if let Err((new_node_ref, child_median_key)) = res {
            if self.data.len() < MAX_KEYS {
                self.data.insert(index + 1, (child_median_key, new_node_ref));
                
                Ok(()) 
            } else {
                let right_split = self.data.split_off(MAX_KEYS / 2);
                let median_key = right_split[0].0;
                let new_internal = InternalNode {
                    data: right_split,
                };

                let mut leaf_ref = Box::new(new_internal);
                match child_median_key.cmp(&median_key) {
                    Ordering::Greater => {
                        let idx = leaf_ref.data.binary_search_by(|probe| probe.0.cmp(&child_median_key)).unwrap_err();
                        leaf_ref.data.insert(idx, (child_median_key, new_node_ref));
                    },
                    Ordering::Less => {
                        let idx = self.data.binary_search_by(|probe| probe.0.cmp(&child_median_key)).unwrap_err();
                        self.data.insert(idx, (child_median_key, new_node_ref));
                    }
                    Ordering::Equal => panic!("Shouldve already checked for existing key.")
                }

                Err((NodeRef::Internal(leaf_ref), median_key))
            }
        } else {
            Ok(())    
        }
    }

    pub fn search(&self, key: u32) -> Option<char> {
        let index = match self.data.binary_search_by(|probe| {
            probe.0.cmp(&key)
        }) {
            Ok(idx) => idx,
            Err(idx) => if idx == 0 { return None } else {idx - 1}
        };

        match &self.data[index].1 {
            NodeRef::Internal(node_ref) => node_ref.search(key),
            NodeRef::Leaf(leaf_ref) => leaf_ref.borrow().search(key)
        }
    }
}

impl LeafNode {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(MAX_KEYS),
            next_leaf: None,
            dirty: false
        }
    }

    pub fn insert(&mut self, key: u32, val: char) -> Result<(), (NodeRef, u32)> {
        if self.dirty {
            self.data.sort();
            self.dirty = false;
        }

        if self.data.len() < MAX_KEYS {
            let index = self.data.binary_search_by(|probe| {
                probe.0.cmp(&key)
            }).expect_err("Key already exists");
            self.data.insert(index, (key, val));

            Ok(())    
        } else {
            let right_split = self.data.split_off((MAX_KEYS + 1) / 2);
            let median_key = right_split[0].0;
            let new_leaf = LeafNode {
                data: right_split,
                next_leaf: self.next_leaf.take(),
                dirty: false
            };

            let leaf_ref = Rc::new(RefCell::new(new_leaf));
            self.next_leaf = Some(leaf_ref.clone());

            match key.cmp(&median_key) {
                Ordering::Greater => {
                    leaf_ref.borrow_mut().insert(key, val).expect("Split Leaf shouldve had space.")
                },
                Ordering::Less => {
                    self.insert(key, val).expect("Split Leaf shouldve had space.")
                }
                Ordering::Equal => panic!("Shouldve already checked for existing key.")
            }

            Err((NodeRef::Leaf(leaf_ref), median_key))
        }
    }

    pub fn insert_dirty(&mut self, key: u32, val: char) -> Result<(), (NodeRef, u32)> {
        if self.data.len() < MAX_KEYS {
            self.data.push((key, val));
            self.dirty = true;
            Ok(())
        } else {
            self.insert(key, val)
        }
    }
    

    pub fn search(&self, key: u32) -> Option<char> {
        if self.dirty {
            panic!("Can't search when node is dirty.");
        }

        self.data.binary_search_by(|probe| {
            probe.0.cmp(&key)
        }).ok().map(|idx| self.data[idx].1)
    }
}



#[derive(Debug)]
pub struct BTree {
    root: NodeRef
}

impl BTree {
    pub fn new() -> Self {
        let mut btree = Self {
            root: NodeRef::Leaf(Rc::new(RefCell::new(LeafNode::new())))
        };
        btree.insert(0, 'a');
        btree
    }

    pub fn insert(&mut self, key: u32, val: char) {
        let res = match &mut self.root {
            NodeRef::Internal(the_box) => the_box.insert(key, val),
            NodeRef::Leaf(rc) => rc.borrow_mut().insert_dirty(key, val)
        };

        if let Err((root_split, key)) = res {
            let og_key = match &mut self.root {
                NodeRef::Internal(the_box) => the_box.data[0].0,
                NodeRef::Leaf(rc) => rc.borrow().data[0].0
            };

            let mut temp = NodeRef::Internal(Box::new(InternalNode::new()));
            std::mem::swap(&mut self.root, &mut temp);
            if let NodeRef::Internal(the_box) = &mut self.root {
                the_box.data = vec![(og_key, temp),(key, root_split)];
            }
        }
    }


    
    pub fn search(&self, key: u32) -> Option<char> {
        match &self.root {
            NodeRef::Internal(node) => node.search(key),
            NodeRef::Leaf(node) => node.borrow().search(key)
        }
    }

    pub fn height(&self) -> usize {
        let mut height = 0;
        let mut cur = &self.root;
        while let NodeRef::Internal(the_box) = cur {
            height += 1;
            let first_child = &the_box.data[0];
            cur = &first_child.1;
        }
        height
    }

    pub fn iter(&self) -> BTreeIter {
        //Find leftmost leaf
        let mut cur = &self.root;
        while let NodeRef::Internal(the_box) = cur {
            let first_child = &the_box.data[0];
            cur = &first_child.1;
        }

        if let NodeRef::Leaf(rc) = cur {
            BTreeIter {
                current_leaf: Some(rc.clone()),
                current_index: 0,
            }
        } else  {
            panic!("How did we get here?")
        }
    }

    pub fn fix(&mut self) {
        let mut cur = &self.root;
        while let NodeRef::Internal(the_box) = cur {
            let first_child = &the_box.data[0];
            cur = &first_child.1;
        }

        let mut cur_leaf = match cur {
            NodeRef::Internal(_) => panic!(),
            NodeRef::Leaf(rc) => Some(rc.clone())
        };

        while let Some(leaf) = cur_leaf.take() {
            leaf.borrow_mut().data.sort();
            leaf.borrow_mut().dirty = false;
            cur_leaf = leaf.borrow().next_leaf.clone()
        }
        
    }
}

impl Display for BTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        let mut queue = VecDeque::new();
        queue.push_back(&self.root);
        while !queue.is_empty() {
            let width = queue.len();

            for _ in 0..width {
                let next = queue.pop_front().unwrap();
                match next {
                    NodeRef::Internal(internal) => {
                        f.write_char('(')?;
                        for key in internal.data.iter().map(|tuple| tuple.0) {
                            f.write_fmt(format_args!("{},", key))?;
                        }
                        f.write_char(')')?;
                        queue.extend(internal.data.iter().map(|tuple| &tuple.1));
                    },
                    NodeRef::Leaf(leaf) => {
                        f.write_char('(')?;
                        for key in leaf.borrow().data.iter().map(|tuple| tuple.0) {
                            f.write_fmt(format_args!("{},", key))?;
                        }
                        f.write_char(')')?;
                    }
                }
                f.write_str(" | ")?;
            }
            f.write_str("\n\n")?;
        }
    Ok(())
    }
}


pub struct BTreeIter {
    current_leaf: Option<Rc<RefCell<LeafNode>>>,
    current_index: usize
}

impl Iterator for BTreeIter {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let (ret_val, next_leaf) = {
            if let Some(x) = self.current_leaf.as_deref() {
                let leaf_node = x.borrow();
                let next_leaf = if (self.current_index + 1) == leaf_node.data.len() {
                    Some(leaf_node.next_leaf.clone())
                } else {
                    None
                };
                (leaf_node.data[self.current_index].1, next_leaf)
            } else {
                return None;
            }
        };

        self.current_index = if let Some(next_leaf) = next_leaf {
            self.current_leaf = next_leaf;
            0
        } else {
            self.current_index + 1
        };

        Some(ret_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{random::random, time::Instant};

    // #[test]
    pub fn test_insert_and_search() {
        let mut btree = BTree::new();

        btree.insert(1, 'a');
        btree.insert(2, 'b');
        btree.insert(3, 'c');
        println!("{}",&btree);
        
        assert_eq!(btree.search(0), None);
        assert_eq!(btree.search(1), Some('a'));
        assert_eq!(btree.search(2), Some('b'));
        assert_eq!(btree.search(3), Some('c'));
        assert_eq!(btree.search(4), None);

        btree.insert(4, 'd');
        println!("{}",&btree);
    }

    #[test]
    pub fn test_split() {
    
    }

    #[test]
    pub fn test_big() {
        let mut btree = BTree::new();
        println!("MAX_KEYS: {}", MAX_KEYS);

        let mut nums: Vec<(u32, u32)> = (1..10_000_000).map(|n| (random::<u32>(),n)).collect();
        nums.sort();

        println!("Starting!\n\n");
        let start = Instant::now();
        for key in nums.into_iter().map(|tuple| tuple.1) {
            btree.insert(key, 'a');
        }
        btree.fix();

        println!("{:?}", start.elapsed());
    }

    #[test]
    pub fn test_iter() {
        let mut btree = BTree::new();

        let test_string = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        let mut data: Vec<(u32, char)> = test_string.chars().enumerate().map(|(index, c)| (index as u32 + 1, c)).collect();
        let randomized: Vec<(u32, char)> = {
            let mut attach_random: Vec<(u32, (u32, char))> = data.into_iter().map(|ele| (random(), ele)).collect();
            attach_random.sort();
            attach_random.into_iter().map(|(_, ele)| ele).collect()
        };

        for (key, val) in randomized.into_iter() {
            btree.insert(key, val);
        }

        btree.fix();
        let btree_iter = btree.iter().skip(1);
        let output: String = btree_iter.collect();

        assert_eq!(&output, test_string);
    }
}






