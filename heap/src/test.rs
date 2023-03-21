use super::Heap;

#[test]
fn binary_heap() {
    let mut heap = Heap::new(2);

    heap.insert(15);
    heap.insert(11);
    heap.insert(10);
    assert_eq!(heap.top(), Some(15));
    assert_eq!(heap.top(), Some(11));
    assert_eq!(heap.top(), Some(10));
}

#[test]
fn ternary_heap() {
    let mut heap = Heap::new(3);

    heap.insert(15);
    heap.insert(11);
    heap.insert(10);
    assert_eq!(heap.top(), Some(15));
    assert_eq!(heap.top(), Some(11));
    assert_eq!(heap.top(), Some(10));
}
