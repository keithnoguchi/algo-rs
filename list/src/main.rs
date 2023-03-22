//! A singly linked list

use std::fmt::Debug;

#[derive(Debug)]
pub struct LinkedList<T: Debug + PartialOrd>(Option<(T, Box<Self>)>);

impl<T: Debug + PartialOrd> Default for LinkedList<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Debug + PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_front(&mut self, data: T) {
        let next = self.0.take();
        self.0 = Some((data, Box::new(Self(next))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut next)) => next.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn insert(&mut self, data: T) {
        match self.0 {
            None => self.push_front(data),
            Some((ref current, ref mut next)) => {
                if data <= *current {
                    self.push_front(data);
                } else {
                    next.insert(data);
                }
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_back(11);
    list.push_back(12);
    list.push_back(13);
    list.insert(0);
    list.insert(14);
    list.insert(8);
    println!("{list:?}");
}
