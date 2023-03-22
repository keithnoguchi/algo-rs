//! A skip list

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T: Debug> {
    data: Rc<RefCell<T>>,
    next: Option<Rc<RefCell<Self>>>,
    down: Option<Rc<RefCell<Self>>>,
}

impl<T: Debug> From<T> for Node<T> {
    fn from(data: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(data)),
            next: None,
            down: None,
        }
    }
}

fn main() {
    let node = Node::from(1);

    println!("{node:?}");
}
