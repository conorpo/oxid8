
use std::fmt::Debug;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Display, Write};
use std::rc::Rc;

// TODO BTree
// figure out that issue we solved by inserting 0
// root node maybe shouldnt be an option

pub trait Key = Ord + Debug + Copy;
pub trait Val = Debug + Copy;

const MAX_KEYS: usize = 1000; // todo make specific to key-value pair
// const MAX_KEYS: usize = 5;

#[derive(Debug)]
enum NodeRef<K: Key,V : Val> {
    Internal(Box<InternalNode<K,V>>),
    Leaf(Rc<RefCell<LeafNode<K,V>>>)
}
use NodeRef as NR;

#[derive(Debug)]
struct InternalNode<K: Key, V: Val> {
    keys: Vec<K>,
    children: Vec<NodeRef<K,V>>
}


#[derive(Debug)]
struct LeafNode<K: Key,V : Val> {
    key_val_pairs: Vec<(K, V)>,
    next_leaf: Option<Rc<RefCell<LeafNode<K,V>>>>,
}
impl<K: Key,V : Val> InternalNode<K,V> {
   

    pub fn new(child: NodeRef<K,V>) -> Self {
        Self {
            keys: Vec::new(),
            children: vec![child]
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Result<(), (NodeRef<K,V>, K)> {
        let index = self.keys.binary_search_by(|probe| {
            probe.cmp(&key)
        }).expect_err("Key already exists");

        let child_insert_result = match &mut self.children[index] {
            NR::Internal(the_box) => the_box.insert(key, val),
            NR::Leaf(rc) => rc.borrow_mut().insert(key, val)
        };

        if let Err((new_child_node, child_median_key)) = child_insert_result {
            if self.keys.len() < MAX_KEYS {
                self.keys.insert(index, child_median_key);
                self.children.insert(index + 1, new_child_node);
                
                Ok(()) 
            } else {
                let right_split = self.keys.split_off(MAX_KEYS / 2);
                let right_split_children = self.children.split_off(MAX_KEYS / 2 + 1);
                
                let mut right_node = Box::new(InternalNode {
                    keys: right_split,
                    children: right_split_children
                });
                
                let median_key = self.keys.pop().unwrap();
                debug_assert_eq!(self.keys.len() + 1, self.children.len());
                
                match child_median_key.cmp(&median_key) {
                    Ordering::Greater => {
                        let idx = right_node.keys.binary_search_by(|probe| probe.cmp(&child_median_key)).unwrap_err();
                        right_node.keys.insert(idx, child_median_key);
                        right_node.children.insert(idx + 1, new_child_node);
                    },
                    Ordering::Less => {
                        let idx = self.keys.binary_search_by(|probe| probe.cmp(&child_median_key)).unwrap_err();
                        self.keys.insert(idx, child_median_key);
                        self.children.insert(idx + 1, new_child_node);
                    }
                    Ordering::Equal => panic!("Shouldve already checked for existing key.")
                }

                Err((NodeRef::Internal(right_node), median_key))
            }
        } else {
            Ok(())    
        }
    }

    pub fn search(&self, key: K) -> Option<V> {

        let index = match self.keys.binary_search_by(|probe| {
            probe.cmp(&key)
        }) {
            Ok(idx) => idx,
            Err(idx) => if idx == 0 { return None } else {idx - 1}
        };

        match &self.children[index + 1] {
            NR::Internal(node_ref) => node_ref.search(key),
            NR::Leaf(leaf_ref) => leaf_ref.borrow().search(key)
        }
    }
}

impl<K: Key, V: Val> LeafNode<K,V> {
    pub fn new() -> Self {
        Self {
            key_val_pairs: Vec::new(),
            next_leaf: None
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Result<(), (NodeRef<K,V>, K)> {
        if self.key_val_pairs.len() < MAX_KEYS {
            let index = self.key_val_pairs.binary_search_by(|probe| {
                probe.0.cmp(&key)
            }).expect_err("Key already exists");
            self.key_val_pairs.insert(index, (key, val));

            Ok(())    
        } else {
            let right_split = self.key_val_pairs.split_off((MAX_KEYS + 1) / 2);
            let median_key = right_split[0].0;
            let new_leaf = LeafNode {
                key_val_pairs: right_split,
                next_leaf: self.next_leaf.take()
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
                Ordering::Equal => panic!("Shouldcve already checked for existing key.")
            }

            Err((NodeRef::Leaf(leaf_ref), median_key))
        }
    }
    

    pub fn search(&self, key: K) -> Option<V> {
        self.key_val_pairs.binary_search_by(|probe| {
            probe.0.cmp(&key)
        }).ok().map(|idx| self.key_val_pairs[idx].1)
    }
}



#[derive(Debug)]
pub struct BTree<K: Key, V: Val> {
    root: Option<NodeRef<K,V>>
}

impl<K: Key,V: Val> BTree<K,V> {
    pub fn new() -> Self {
        Self {
            root: Some(NodeRef::Leaf(Rc::new(RefCell::new(LeafNode::new()))))
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        let res = match self.root.as_mut().expect("why is the root gone?") {
            NR::Internal(the_box) => the_box.insert(key, val),
            NR::Leaf(rc) => rc.borrow_mut().insert(key, val)
        };

        if let Err((root_split, key)) = res {
            let new_root = InternalNode {
                keys: vec![key],
                children: vec![self.root.take().unwrap(), root_split]
            };

            self.root = Some(NR::Internal(Box::new(new_root)));
        }
    }


    
    pub fn search(&self, key: K) -> Option<V> {
        match self.root.as_ref().unwrap() {
            NodeRef::Internal(node) => node.search(key),
            NodeRef::Leaf(node) => node.borrow().search(key)
        }
    }

    pub fn height(&self) -> usize {
        let mut height = 0;
        let mut cur = self.root.as_ref();
        loop {
            cur = match cur {
                Some(NodeRef::Internal(the_box)) => {
                    Some(&the_box.children[0])
                },
                Some(NodeRef::Leaf(_)) => {
                    None
                },
                None => break
            };
            height += 1;
        }
        height
    }

    pub fn iter(&self) -> BTreeIter<K,V> {
        //Find leftmost leaf
        let mut cur = self.root.as_ref().unwrap();
        while let NodeRef::Internal(the_box) = cur {
            cur = &the_box.children[0];
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
}

impl<K: Key,V: Val> Display for BTree<K,V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        let mut queue = VecDeque::new();
        queue.push_back(self.root.as_ref().unwrap());
        while !queue.is_empty() {
            let width = queue.len();

            for _ in 0..width {
                let next = queue.pop_front().unwrap();
                match next {
                    NodeRef::Internal(internal) => {
                        f.write_char('(')?;
                        for key in internal.keys.iter() {
                            f.write_fmt(format_args!("{:?},", key))?;
                        }
                        f.write_char(')')?;
                        queue.extend(internal.children.iter());
                    },
                    NodeRef::Leaf(leaf) => {
                        f.write_char('(')?;
                        for key in leaf.borrow().key_val_pairs.iter().map(|tuple| tuple.0) {
                            f.write_fmt(format_args!("{:?},", key))?;
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


pub struct BTreeIter<K: Key,V: Val> {
    current_leaf: Option<Rc<RefCell<LeafNode<K,V>>>>,
    current_index: usize
}

impl<K: Key, V: Val> Iterator for BTreeIter<K,V> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        let (ret_val, next_leaf) = {
            if let Some(x) = self.current_leaf.as_deref() {
                let leaf_node = x.borrow();
                let next_leaf = if (self.current_index + 1) == leaf_node.key_val_pairs.len() {
                    Some(leaf_node.next_leaf.clone())
                } else {
                    None
                };
                (leaf_node.key_val_pairs[self.current_index].1, next_leaf)
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
    use std::random::random;

    // #[test]
    pub fn test_insert_and_search() {
        let mut btree = BTree::<u32, char>::new();

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

        let mut nums: Vec<(u32, u32)> = (1..200_000).map(|n| (random::<u32>(),n)).collect();
        nums.sort();

        println!("Starting!\n\n");
        for key in nums.into_iter().map(|tuple| tuple.1) {
            btree.insert(key, 'a');
        }

        println!("{}", btree.height());
        // assert_eq!(btree.search(9999999),None);
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

        println!("Starting: ");
        for (key, val) in randomized.into_iter() {
            btree.insert(key, val);
        }
        println!("Done inserting.");

        let btree_iter = btree.iter();
        let output: String = btree_iter.collect();

        assert_eq!(&output, test_string);
    }
}






