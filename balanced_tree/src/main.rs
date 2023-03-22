//! A balanced binary tree

#![forbid(missing_debug_implementations)]

use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub struct Tree<T: Debug>(Option<Box<Node<T>>>);

impl<T: Debug> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print_depth_first(f, 0)
    }
}

impl<T: Debug> Tree<T> {
    pub const fn new() -> Self {
        Self(None)
    }

    pub const fn height(&self) -> i8 {
        match self.0 {
            Some(ref node) => node.height,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(ref mut node) = self.0 {
            node.height = 1 + std::cmp::max(node.left.height(), node.right.height());
        }
    }

    fn print_depth_first(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        if let Some(ref node) = self.0 {
            node.left.print_depth_first(f, depth + 1)?;
            writeln!(f, "{}:{:.<depth$}{:?}", node.height, "", node.data)?;
            node.right.print_depth_first(f, depth + 1)?;
        }
        Ok(())
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
        self.set_height();
    }
}

#[derive(Debug)]
pub struct Node<T: Debug> {
    data: T,
    height: i8,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: Debug> From<T> for Node<T> {
    fn from(data: T) -> Self {
        Self {
            data,
            height: 0,
            left: Tree::default(),
            right: Tree::default(),
        }
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.insert(4);
    tree.insert(5);
    tree.insert(6);
    tree.insert(10);
    tree.insert(1);
    tree.insert(94);
    tree.insert(54);
    tree.insert(3);
    print!("{tree}");
}
