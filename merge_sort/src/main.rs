//! A merge sort.

use std::fmt::Debug;

/// O(n * ln(n))
pub fn merge_sort<T: Debug + PartialOrd>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }
    let mut result = Vec::with_capacity(v.len());
    let right = v.split_off(v.len() / 2);

    // sort the left half.
    let a = merge_sort(v);

    // sort the right half.
    let b = merge_sort(right);

    // bring the sorted halfs together.
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();
    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        result.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        result.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    result.push(a_peek.take().unwrap());
                    result.extend(a_it);
                    return result;
                }
            },
            None => {
                if let Some(val) = b_peek {
                    result.push(val);
                }
                result.extend(b_it);
                return result;
            }
        }
    }
}

#[cfg(test)]
mod test;

fn main() {
    let v = vec![9, 8, 10, 22, 1, -1];
    let v = merge_sort(v);
    assert_eq!(v, vec![-1, 1, 8, 9, 10, 22]);
}
