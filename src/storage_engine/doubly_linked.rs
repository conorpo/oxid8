use std::{cell::{Ref, RefCell}, rc::Rc};

struct Node<T> {
    pub next: Option<Rc<RefCell<Node<T>>>>,
    data: T,
    pub prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T)  -> Self {
        Self {
            next: None,
            data,
            prev: None,
        }
    }
}

pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0
        }
    }
    
      
    pub fn push_front(&mut self, new_val: T) {
        let mut temp = Node::<T>::new(new_val);
        temp.next = self.head.take();
        self.head = Some(Rc::new(RefCell::new(temp)));
        
        if self.tail.is_none() {
            self.tail = Some(self.head.as_ref().unwrap().clone());
        }
        self.len += 1;
    }
    pub fn push_back(&mut self, val: T) {
        let mut temp = Node::<T>::new(val);
        temp.prev = self.tail.take();
        self.tail = Some(Rc::new(RefCell::new(temp)));
        
        if self.head.is_none() {
            self.head = Some(self.head.as_ref().unwrap().clone());
        }
        self.len += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(head_rc) = self.head.take() {
            if let Some(next_ref) = head_rc.borrow().next.as_deref() {
                next_ref.borrow_mut().prev = None;
            };

            if self.len == 1 {
                self.tail = None;
            }
            
            let mut head = Rc::into_inner(head_rc).expect("things still referencing this.").into_inner();
            self.head = head.next.take();
            self.len -= 1;

            Some(head.data)
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
    
    }
}