//! A balanced binary tree

#![forbid(missing_debug_implementations)]

use std::fmt::Debug;

#[derive(Debug)]
pub struct Tree<T: Debug>(Option<Box<Node<T>>>);

impl<T: Debug> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Tree<T> {
    pub const fn new() -> Self {
        Self(None)
    }
}

impl<T: Debug + PartialOrd> Tree<T> {
    pub fn insert(&mut self, data: T) {
        match self.0 {
            None => self.0 = Some(Box::new(Node::from(data))),
            Some(ref mut node) => {
                if data < node.data {
                    node.left.insert(data);
                } else if data > node.data {
                    node.right.insert(data);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Node<T: Debug> {
    data: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: Debug> From<T> for Node<T> {
    fn from(data: T) -> Self {
        Self {
            data,
            left: Tree::default(),
            right: Tree::default(),
        }
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.insert(100);
    tree.insert(1);
    tree.insert(100);
    tree.insert(99);
    tree.insert(100);
    tree.insert(9);
    tree.insert(44);
    tree.insert(59);

    println!("{tree:?}");
}
