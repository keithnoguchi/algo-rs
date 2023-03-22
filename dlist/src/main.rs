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
                let node = Rc::new(RefCell::new(Node::from(data)));
                self.tail = Some(Rc::downgrade(&node));
                self.head = Some(node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.borrow_mut().next.take().map(|next| {
                next.borrow_mut().prev = None;
                next
            });
            Rc::try_unwrap(head).ok().unwrap().into_inner().data
        })
    }

    pub fn push_back(&mut self, data: T) {
        match self.tail.take() {
            Some(tail) => {
                let node = Rc::new(RefCell::new(Node {
                    data,
                    next: None,
                    prev: Some(tail.clone()),
                }));
                let prev = tail.upgrade().unwrap();
                let mut prev = prev.borrow_mut();
                self.tail = Some(Rc::downgrade(&node));
                prev.next = Some(node);
            }
            None => {
                let node = Rc::new(RefCell::new(Node::from(data)));
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

impl<T: Debug> From<T> for Node<T> {
    fn from(data: T) -> Self {
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
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), None);
}
