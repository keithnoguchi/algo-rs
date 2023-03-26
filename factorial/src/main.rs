//! A factorial function

#![forbid(unsafe_code, missing_debug_implementations)]

pub fn factorial(n: usize) -> usize {
    let mut result = 1;
    for x in 2..=n {
        result *= x;
    }
    result
}

#[cfg(test)]
mod test;

fn main() {
    for n in 0..=20 {
        println!("factorial({n}) = {}", factorial(n));
    }
}
