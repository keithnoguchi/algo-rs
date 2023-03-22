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

impl<T: Debug + PartialOrd> Tree<T> {
    pub fn insert(&mut self, data: T) {
        let rotation_direction = match self.0 {
            None => {
                self.0 = Some(Box::new(Node::from(data)));
                0
            }
            Some(ref mut node) => {
                if data < node.data {
                    node.left.insert(data);
                    if node.left.height() - node.right.height() > 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    node.right.insert(data);
                    if node.right.height() - node.left.height() > 1 {
                        -1
                    } else {
                        0
                    }
                }
            }
        };
        match rotation_direction {
            1 => self.rotate_right(),
            -1 => self.rotate_left(),
            _ => self.set_height(),
        }
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
        if let Some(ref mut node) = self.0.as_mut() {
            node.set_height();
        }
    }

    pub fn rotate_left(&mut self) {
        self.0 = self.0.take().map(|node| node.rotate_left());
    }

    pub fn rotate_right(&mut self) {
        self.0 = self.0.take().map(|node| node.rotate_right());
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

impl<T: Debug> Node<T> {
    fn rotate_left(mut self) -> Box<Self> {
        // picks the right node as the new top node.
        let mut top = match self.right.0.take() {
            Some(right) => right,
            None => return Box::new(self),
        };
        self.right = Tree(top.left.0.take());
        self.right.set_height();
        top.left = Tree(Some(Box::new(self)));
        top.left.set_height();
        top.set_height();
        top
    }

    fn rotate_right(mut self) -> Box<Self> {
        let mut top = match self.left.0.take() {
            Some(node) => node,
            None => return Box::new(self),
        };
        self.left = Tree(top.right.0.take());
        self.left.set_height();
        top.right = Tree(Some(Box::new(self)));
        top.right.set_height();
        top.set_height();
        top
    }

    fn set_height(&mut self) {
        self.height = 1 + std::cmp::max(self.left.height(), self.right.height());
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
    println!("{tree}");

    tree.rotate_left();
    println!("{tree}");

    tree.rotate_right();
    println!("{tree}");
}
