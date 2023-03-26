//! A fibonacci sequence.

#![forbid(unsafe_code, missing_debug_implementations)]

pub fn fibonacci(n: usize) -> usize {
    let mut a = 1;
    let mut b = 1;

    let mut result = a;
    for _ in 1..n {
        result = a + b;
        a = b;
        b = result;
    }
    result
}

#[cfg(test)]
mod test;

fn main() {
    for n in 0..=92 {
        println!("fibonacci({n}) = {}", fibonacci(n));
    }
}
