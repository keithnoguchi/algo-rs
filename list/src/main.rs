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
}

fn main() {
    let list = LinkedList::<String>::new();

    println!("{list:?}");
}
