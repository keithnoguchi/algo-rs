//! A bucket list

#![forbid(unsafe_code, missing_debug_implementations)]

use std::hash::Hash;

#[derive(Debug)]
pub struct BucketList<K, V> {
    seed: u64,
    len: usize,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Eq + Hash, V> Default for BucketList<K, V> {
    fn default() -> Self {
        Self {
            seed: rand::random(),
            len: 0,
            buckets: vec![Vec::new()],
        }
    }
}
