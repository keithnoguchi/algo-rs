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
}

fn main() {
    let mut list = LinkedList::<&str>::new();
    list.push_front("first");
    list.push_front("second");
    list.push_front("third");
    println!("{list:?}");
}
