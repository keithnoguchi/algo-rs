use super::quick_sort;

#[test]
fn sorted() {
    let mut v = vec![1, 2, 3, 4, 5];
    quick_sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}
