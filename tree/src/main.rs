//! A binary tree.

#![forbid(missing_debug_implementations)]

use std::fmt::Debug;

#[derive(Debug)]
pub struct Tree<T: Debug>(Option<Box<Node<T>>>);

impl<T: Debug> Default for Tree<T> {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Debug)]
pub struct Node<T: Debug> {
    data: T,
    left: Tree<T>,
    right: Tree<T>,
}

fn main() {
    let tree = Tree::<String>::default();
    println!("{tree:?}");
}
