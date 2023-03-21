//! A merge sort.

use std::fmt::Debug;

pub fn merge_sort<T: Debug + PartialOrd>(v: &mut [T]) {
    for i in 1..v.len() {
        println!("{i:03}: {v:?}");
        let mut sorted = true;
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
    let mut v = vec![9, 8, 10, 22, 1, -1];
    merge_sort(&mut v);
    assert_eq!(v, vec![-1, 1, 8, 9, 10, 22]);
}
