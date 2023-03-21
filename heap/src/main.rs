//! A N-arity max heap.

#![forbid(missing_debug_implementations)]

use std::ops::Range;

/// A N-arity max-heap.
#[derive(Debug)]
pub struct Heap<T: PartialOrd> {
    data: Vec<T>,
    arity: usize,
}

impl<T: PartialOrd> Heap<T> {
    pub fn new(arity: usize) -> Self {
        Self {
            data: Vec::new(),
            arity,
        }
    }

    /// Inserts the new value to the heap.
    pub fn insert(&mut self, value: T) {
        let index = self.data.len();
        self.data.push(value);
        self.bubble_up(index);
    }

    /// Pop the highest priority item from the heap.
    pub fn top(&mut self) -> Option<T> {
        if self.data.len() <= 1 {
            self.data.pop()
        } else {
            let tail = self.data.len() - 1;
            self.data.swap(0, tail);
            let item = self.data.pop();
            self.push_down(0);
            item
        }
    }

    fn bubble_up(&mut self, mut index: usize) {
        debug_assert!(index < self.data.len());
        while let Some(parent) = self.parent_index(index) {
            if self.data[parent] > self.data[index] {
                break;
            }
            self.data.swap(parent, index);
            index = parent;
        }
    }

    fn push_down(&mut self, mut index: usize) {
        while let Some(children) = self.children_indices(index) {
            let max = self.max_index(children);
            if self.data[index] > self.data[max] {
                break;
            }
            self.data.swap(index, max);
            index = max;
        }
    }

    fn parent_index(&self, child: usize) -> Option<usize> {
        match child {
            0 => None,
            _ => Some((child - 1) / self.arity),
        }
    }

    fn children_indices(&self, parent: usize) -> Option<Range<usize>> {
        let start = parent * self.arity + 1;
        if start >= self.data.len() - 1 {
            return None;
        }
        let end = if start + self.arity < self.data.len() {
            start + self.arity
        } else {
            self.data.len()
        };
        Some(start..end)
    }

    fn max_index(&self, children: Range<usize>) -> usize {
        let start = children.start;
        self.data[children]
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| (*a).partial_cmp(*b).unwrap())
            .map(|(index, _)| start + index)
            .unwrap()
    }
}

fn main() {
    let mut heap = Heap::new(2);
    heap.insert(10);
    heap.insert(11);
    heap.insert(51);
    heap.insert(20);
    heap.insert(39);
    heap.insert(15);

    println!("{heap:?}");
    while let Some(top) = heap.top() {
        println!("{top}");
    }

    let mut heap = Heap::new(3);
    heap.insert(10);
    heap.insert(11);
    heap.insert(51);
    heap.insert(20);
    heap.insert(39);
    heap.insert(15);
    println!("{heap:?}");
    while let Some(top) = heap.top() {
        println!("{top}");
    }
}
