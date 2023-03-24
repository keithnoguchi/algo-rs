use super::hash;

#[test]
fn same_input_same_hash() {
    let h = hash(55, "cat");
    assert_eq!(hash(55, "cat"), h);
}

#[test]
fn different_input_different_hash() {
    let h = hash(55, "cat");
    assert!(hash(55, "cats") != h);
}
