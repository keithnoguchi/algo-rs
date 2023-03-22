//! A doubly linked list

#![forbid(missing_debug_implementations)]

use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct LinkedList<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Debug> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
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

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            match node.borrow_mut().next.take() {
                Some(next) => {
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                }
                None => self.tail = None,
            }
            Rc::try_unwrap(node).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            let node = tail.upgrade().unwrap();
            match node.borrow_mut().prev.take() {
                Some(prev) => {
                    prev.upgrade().unwrap().borrow_mut().next = None;
                    self.tail = Some(prev);
                }
                None => self.head = None,
            }
            Rc::try_unwrap(node).ok().unwrap().into_inner().data
        })
    }

    pub const fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[derive(Debug)]
struct Node<T: Debug> {
    data: T,
    next: Option<Rc<RefCell<Self>>>,
    prev: Option<Weak<RefCell<Self>>>,
}

impl<T: Debug> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Debug> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
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
    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_back(), Some(4));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_back(), None);
    assert_eq!(list.pop_front(), None);
}
