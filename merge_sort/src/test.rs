#[test]
fn merge_sort() {
    let v = vec![9, 8, 10, 22, 1, -1];
    let v = super::merge_sort(v);
    assert_eq!(v, vec![-1, 1, 8, 9, 10, 22]);
}
