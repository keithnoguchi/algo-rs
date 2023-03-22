//! Fibonacci sequences.

/// O(n)
fn fibonacci(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut res = 1;

    for _ in 1..n {
        res = a + b;
        a = b;
        b = res;
    }
    res
}

/// O(2n)
fn fibonacci_recursive(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn main() {
    for n in 1..=20 {
        println!("fibonacci({n}) = {}", fibonacci(n));
        assert_eq!(fibonacci(n), fibonacci_recursive(n));
    }
}
