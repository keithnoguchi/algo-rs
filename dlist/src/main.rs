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
    let mut list = LinkedList::<&str>::new();

    println!("{list:?}");
}
