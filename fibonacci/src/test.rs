fn fibonacci(n: usize) -> usize {
    if n < 2 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

#[test]
fn test_fibonacci() {
    for n in 0..=30 {
        assert_eq!(super::fibonacci(n), fibonacci(n));
    }
}
