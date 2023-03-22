//! A skip list

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct SkipList<T: Debug>(Vec<Node<T>>);

impl<T: Debug> Default for SkipList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug + PartialOrd> SkipList<T> {
    pub fn insert(&mut self, data: T) {
        if self.is_empty() {
            self.0.push(Node::from(data));
            return;
        }
        // push the data in the highest available row.
        for i in (0..self.0.len()).rev() {
            if data > *self.0[i].data.borrow() {
                if let Some(_child) = self.0[i].insert(data) {
                    todo!();
                }
                return;
            }
        }
        // Needs to be in the bottom of the list.
        let mut node = Node::from(data);
        std::mem::swap(&mut node, &mut self.0[0]);
        let node = Rc::new(RefCell::new(node));
        self.0[0].next = Some(node.clone());
    }
}

impl<T: Debug> SkipList<T> {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

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

impl<T: Debug + PartialOrd> Node<T> {
    pub fn insert(&mut self, data: T) -> Option<Rc<RefCell<Self>>> {
        if let Some(ref mut next) = self.next {
            if data > *next.borrow().data.borrow() {
                return next.borrow_mut().insert(data);
            }
        }

        if let Some(ref mut down) = self.down {
            return match down.borrow_mut().insert(data) {
                Some(child) => match rand::rand(2) {
                    1 => {
                        let data = child.borrow().data.clone();
                        let node = Self {
                            data,
                            next: self.next.take(),
                            down: Some(child),
                        };
                        let node = Rc::new(RefCell::new(node));
                        self.next = Some(node.clone());
                        Some(node)
                    }
                    _ => None,
                },
                None => None,
            };
        }

        // We're at the bottom of the list.
        let mut node = Node::from(data);
        node.next = self.next.take();
        let node = Rc::new(RefCell::new(node));
        self.next = Some(node.clone());
        Some(node)
    }
}

fn main() {
    let mut node = Node::from(1);
    node.insert(4);
    node.insert(6);
    node.insert(77);
    node.insert(84);
    node.insert(1);
    println!("{node:?}");
}
