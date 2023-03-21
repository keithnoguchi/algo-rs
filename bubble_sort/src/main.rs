//! A bubble sort.

use std::fmt::Debug;

/// O(n^2)
pub fn bubble_sort<T: Debug + PartialOrd>(v: &mut [T]) {
    for i in 1..v.len() {
        let mut sorted = true;
        println!("{i:03}: {v:?}");
        for j in 0..v.len() - i {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                sorted = false;
            }
        }
        println!("{i:03}: {v:?}");
        if sorted {
            return;
        }
    }
}

fn main() {
    let mut v = vec![4, 6, 12, 11, 8, 1];
    bubble_sort(&mut v);
    assert_eq!(v, vec![1, 4, 6, 8, 11, 12]);
}
