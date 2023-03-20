//! A N-arity max heap.

#![forbid(missing_debug_implementations)]

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

    fn insert(&mut self, value: T) {
        let mut index = self.data.len();
        self.data.push(value);
        while let Some(parent) = self.parent(index) {
            if self.data[parent] >= self.data[index] {
                break;
            }
            self.data.swap(parent, index);
            index = parent;
        }
    }

    fn parent(&self, index: usize) -> Option<usize> {
        match index {
            0 => None,
            _ => Some((index - 1) / self.arity),
        }
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

    let mut heap = Heap::new(3);
    heap.insert(10);
    heap.insert(11);
    heap.insert(51);
    heap.insert(20);
    heap.insert(39);
    heap.insert(15);

    println!("{heap:?}");
}
