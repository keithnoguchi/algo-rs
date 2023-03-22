use std::fmt::Debug;

#[derive(Debug)]
pub struct LinkedList<T: Debug>(Option<(T, Box<Self>)>);

impl<T: Debug> Default for LinkedList<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Debug> LinkedList<T> {
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
}

fn main() {
    let mut list = LinkedList::<&str>::new();
    list.push_front("push front first");
    list.push_front("push front second");
    list.push_front("push front third");
    list.push_back("push back first");
    list.push_back("push back second");
    println!("{list:?}");
}
