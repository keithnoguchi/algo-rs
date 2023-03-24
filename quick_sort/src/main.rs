//! A quick sort

use std::fmt::Debug;
use std::thread;

/// O(n * ln(n))
///
/// O(n^2) in case `v` is sorted.
pub fn quick_sort<T: Debug + PartialOrd + Send>(v: &mut [T]) {
    let id = thread::current().id();
    println!("{id:?}: quick_sort: {v:?}");
    if v.len() <= 1 {
        return;
    }
    let pivot = pivot(v);
    let (a, b) = v.split_at_mut(pivot);
    rayon::join(|| quick_sort(&mut a[..]), || quick_sort(&mut b[1..]));
    println!("{id:?}: quick_sort: {v:?}");
}

fn pivot<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let mut pivot = random::rand(v.len());
    v.swap(0, pivot);
    pivot = 0;
    for i in 1..v.len() {
        if v[i] < v[0] {
            pivot += 1;
            v.swap(i, pivot);
        }
    }
    v.swap(0, pivot);
    pivot
}

#[cfg(test)]
mod test;

fn main() {
    let mut v = vec!['a', 'c', 'z', 'i', 'y'];
    quick_sort(&mut v);
    assert_eq!(v, vec!['a', 'c', 'i', 'y', 'z']);
}
