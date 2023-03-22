//! A doubly linked list

#![forbid(missing_debug_implementations)]

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct LinkedList<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Debug> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_front(&mut self, data: T) {
        match self.head.take() {
            Some(head) => {
                let node = Rc::new(RefCell::new(Node {
                    data,
                    next: Some(head.clone()),
                    prev: None,
                }));
                let mut next = head.borrow_mut();
                next.prev = Some(Rc::downgrade(&node));
                self.head = Some(node);
            }
            None => {
                let node = Rc::new(RefCell::new(Node::new(data)));
                self.tail = Some(Rc::downgrade(&node));
                self.head = Some(node);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.tail.take() {
            Some(tail) => {
                let node = Rc::new(RefCell::new(Node {
                    data,
                    next: None,
                    prev: Some(tail.clone()),
                }));
                let tail = tail.upgrade().unwrap();
                let mut tail = tail.borrow_mut();
                self.tail = Some(Rc::downgrade(&node));
                tail.next = Some(node);
            }
            None => {
                let node = Rc::new(RefCell::new(Node::new(data)));
                self.tail = Some(Rc::downgrade(&node));
                self.head = Some(node);
            }
        }
    }
}

#[derive(Debug)]
pub struct Node<T: Debug> {
    data: T,
    next: Option<Rc<RefCell<Self>>>,
    prev: Option<Weak<RefCell<Self>>>,
}

impl<T: Debug> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: None,
            prev: None,
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.push_back(4);
    list.push_back(5);
    println!("{list:?}");
}
