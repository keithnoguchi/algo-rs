//! A doubly linked list

use dlist::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_front("3".to_string());
    list.push_front("2".to_string());
    list.push_front("1".to_string());
    list.push_back("4".to_string());
    list.push_back("5".to_string());
    assert_eq!(list.len(), 5);
    for item in list.iter() {
        println!("{}", **item.borrow());
    }
    assert_eq!(list.pop_front(), Some(String::from("1")));
    assert_eq!(list.pop_back(), Some(String::from("5")));
    assert_eq!(list.pop_front(), Some(String::from("2")));
    assert_eq!(list.pop_back(), Some(String::from("4")));
    assert_eq!(list.pop_front(), Some(String::from("3")));
    assert_eq!(list.pop_back(), None);
    assert_eq!(list.pop_front(), None);
}
