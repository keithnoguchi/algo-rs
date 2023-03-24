//! A hasher.

#![forbid(unsafe_code, missing_debug_implementations)]

use std::hash::{Hash, Hasher};

/// A hash function using `MHash` as the hasher.
pub fn hash<T: Hash>(seed: u64, t: T) -> u64 {
    let mut hasher = MHash::default();
    hasher.write_u64(seed);
    t.hash(&mut hasher);
    hasher.finish()
}

/// A hasher.
#[derive(Debug, Default)]
struct MHash {
    prev: u8,
    n: u128,
}

impl Hasher for MHash {
    fn write(&mut self, data: &[u8]) {
        for d in data {
            self.n = ((self.n + 11) * (*d as u128 + 13) + (*d ^ self.prev) as u128)
                % (std::u64::MAX as u128);
            self.prev = *d;
        }
    }

    fn finish(&self) -> u64 {
        self.n as u64
    }
}

#[cfg(test)]
mod test;
