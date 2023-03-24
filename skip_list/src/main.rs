//! A skip list

use std::cell::RefCell;
use std::fmt::{self, Debug, Display};
use std::rc::Rc;

#[derive(Debug)]
pub struct SkipList<T: Debug>(Vec<Node<T>>);

impl<T: Debug> Default for SkipList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Display for SkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            row.print_row(f)?;
            writeln!(f)?;
        }
        Ok(())
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
                if let Some(child) = self.0[i].insert(data) {
                    self.loop_up(child, i + 1);
                }
                return;
            }
        }
        // Needs to be in the bottom of the list.
        let mut node = Node::from(data);
        std::mem::swap(&mut node, &mut self.0[0]);
        let node = Rc::new(RefCell::new(node));
        self.0[0].next = Some(node.clone());
        self.loop_up(node, 1);
    }

    fn loop_up(&mut self, child: Rc<RefCell<Node<T>>>, n: usize) {
        if random::rand(2) == 1 {
            return;
        }
        let data = child.borrow().data.clone();
        let mut node = Node {
            data,
            next: None,
            down: Some(child),
        };
        if n >= self.0.len() {
            self.0.push(node);
            return;
        }
        std::mem::swap(&mut node, &mut self.0[n]);
        self.0[n].next = Some(Rc::new(RefCell::new(node)));
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
                Some(child) => match random::rand(2) {
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

impl<T: Debug> Node<T> {
    fn print_row<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "{:?}", self.data.borrow())?;
        if let Some(ref next) = self.next {
            write!(w, ",")?;
            next.borrow().print_row(w)?;
        }
        Ok(())
    }
}

fn main() {
    let mut list = SkipList::new();
    list.insert(4);
    list.insert(6);
    list.insert(77);
    list.insert(84);
    list.insert(27);
    list.insert(1);
    print!("{list}");
}
