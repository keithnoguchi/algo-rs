#![forbid(missing_debug_implementations)]

use std::sync::Mutex;

/// A simple psudo random number generator
pub fn rand(max: usize) -> usize {
    static RG: Mutex<RandGen> = Mutex::new(RandGen::new(31));
    RG.lock().unwrap().next(max)
}

#[derive(Debug)]
pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub const fn new(curr: usize) -> Self {
        Self {
            curr,
            mul: 56394237,
            inc: 346423491,
            modulo: 2325454456,
        }
    }

    pub fn next(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr % max
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn rand() {
        let mut hit = HashSet::new();
        for _ in 0..100 {
            let got = super::rand(10000);
            assert!(hit.insert(got));
        }
    }
}
