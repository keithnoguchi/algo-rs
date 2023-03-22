//! A doubly linked list

use dlist::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.push_back(4);
    list.push_back(5);
    println!("{list:?}");
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_back(), Some(4));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_back(), None);
    assert_eq!(list.pop_front(), None);
}
