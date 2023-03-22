use std::fmt::Debug;

#[derive(Debug)]
pub struct LinkedList<T: Debug>(Option<(T, Box<Self>)>);

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self(None)
    }
}

fn main() {
    let list = LinkedList::<String>::new();

    println!("{list:?}");
}
