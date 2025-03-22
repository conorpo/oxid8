use super::directory::*;
use std::collections::HashMap;
use std::collections::VecDeque;


use std::{cell::{Ref, RefCell}, rc::Rc};

struct Node<T> {
    pub next: Option<Rc<RefCell<Node<T>>>>,
    data: T,
    key: (u16, u16),
    pub prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T, key: (u16, u16))  -> Self {
        Self {
            next: None,
            data,
            key,
            prev: None,
        }
    }
}

pub struct HashQueue<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
    hashmap: HashMap<(u16, u16), Rc<RefCell<Node<T>>>>,
}

impl<T> HashQueue<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            hashmap: HashMap::new()
        }
    }

    pub fn get_node(&self, key: &(u16, u16)) -> Option<Rc<RefCell<Node<T>>>> {
        self.hashmap.get(key).map(|rc| {rc.clone()})
    }

    pub fn take_node(&mut self, key: &(u16, u16)) -> Option<Rc<RefCell<Node<T>>>> {
        self.hashmap.remove(key).map(|node| {
            {
                
                // Break Links, Fix Gap
                let mut borrow = node.borrow_mut();

                let prev = borrow.prev.take();
                let next = borrow.next.take();
                if let Some(next) = next.as_deref() {
                    next.borrow_mut().prev = prev.clone();
                } else {
                    self.tail = prev.clone();
                }

                if let Some(prev) = prev {
                    prev.borrow_mut().next = next;
                } else {
                    self.head = next;
                }
            }
            node
        })
    }
    
    pub fn push_node(&mut self, node: Rc<RefCell<Node<T>>>) {
        self.head = Some(node);
        
        if self.tail.is_none() {
            self.tail = Some(self.head.as_ref().unwrap().clone());
        }
        self.len += 1;

        //todo figure out hashmap insertions
    }
      
    pub fn push_val(&mut self, key: (u16, u16), val: T) {
        let mut temp = Node::<T>::new(val);
        temp.next = self.head.take();
        let head = Rc::new(RefCell::new(temp));

        self.push_node(head.clone());

        self.hashmap.insert(key, head);
    }

    pub fn pop(&mut self) -> Option<Node<T>> {
        if let Some(tail_rc) = self.tail.take() {
            if let Some(prev_ref) = tail_rc.borrow().prev.as_deref() {
                prev_ref.borrow_mut().next = None;
            };

            if self.len == 1 {
                self.head = None;
            }
            
            let mut tail = Rc::into_inner(tail_rc).expect("things still referencing this.").into_inner();
            self.tail = tail.prev.take();
            self.len -= 1;

            self.hashmap.remove(&tail.key);

            Some(tail)
        } else {
            None
        }
    }
}

// todo: figure out max cache size, eviction strategy (2Q)
pub struct PageCache {
    main: HashQueue<Box<[u8]>>,
    fifo_in: HashQueue<Box<[u8]>>,
    fifo_out: HashQueue<Box<[u8]>>
}


impl PageCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new()
        }
    }

    pub fn read_page(&self, full_page_index: (u16, u16)) -> Option<Box<[u8]>> {
        todo!()
    }

    pub fn get_mut(&mut self, full_page_index: (u16, u16)) -> Option<&mut Box<[u8]>> {
        self.cache.get_mut(&full_page_index)
    }
}

fn main(){
    println!("hello world?")
}