//! A bubble sort.

/// O(n^2)
pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for i in 1..v.len() {
        let mut sorted = true;
        for j in 0..v.len() - i {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

fn main() {
    let mut v = vec![4, 6, 1, 8, 11, 12];
    bubble_sort(&mut v);
    assert_eq!(v, vec![1, 4, 6, 8, 11, 12]);
}
