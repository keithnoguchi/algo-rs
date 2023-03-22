//! A binary tree.

#![forbid(missing_debug_implementations)]

use std::fmt::Debug;

#[derive(Debug)]
pub struct Tree<T: Debug>(Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct Node<T: Debug> {
    data: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: Debug> Tree<T> {
    pub const fn new() -> Self {
        Self(None)
    }
}

fn main() {
    let mut tree = Tree::<String>::new();
    println!("{tree:?}");
}
