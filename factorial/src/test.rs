fn factorial(n: usize) -> usize {
    if n < 2 {
        return 1;
    }
    factorial(n - 1) * n
}

#[test]
fn test() {
    for n in 0..=20 {
        assert_eq!(super::factorial(n), factorial(n));
    }
}
