use std::collections::HashSet;

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

#[test]
fn different_hash_with_different_order() {
    let h = hash(55, "abc");
    assert!(hash(55, "cba") != h);
}

#[test]
fn diffrent_hash_with_different_numbers() {
    let mut result = HashSet::new();

    for i in 0..10_000 {
        // It returns true if there is no identical hash
        // before.
        assert!(result.insert(hash(55, i)));
    }
}
