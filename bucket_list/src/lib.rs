//! A bucket list

#![forbid(unsafe_code, missing_debug_implementations)]

use std::borrow::{Borrow, BorrowMut};
use std::hash::Hash;

#[derive(Debug)]
pub struct BucketList<K, V> {
    seed: u64,
    len: usize,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> Default for BucketList<K, V> {
    fn default() -> Self {
        Self {
            seed: rand::random(),
            len: 0,
            buckets: vec![Vec::new()],
        }
    }
}

impl<K: Eq + Hash, V> BucketList<K, V> {
    /// Pushes the new key/value and returns the current size of the bucket.
    pub fn push(&mut self, k: K, v: V) -> usize {
        let h = (hasher::hash(self.seed, &k) as usize) % self.buckets.len();
        self.buckets[h].push((k, v));
        self.len += 1;
        self.buckets[h].len()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let h = (hasher::hash(self.seed, k) as usize) % self.buckets.len();
        for (ik, iv) in &self.buckets[h] {
            if k == ik.borrow() {
                return Some(iv);
            }
        }
        None
    }

    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: BorrowMut<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let h = (hasher::hash(self.seed, k) as usize) % self.buckets.len();
        for (ik, iv) in &mut self.buckets[h] {
            if k == ik.borrow_mut() {
                return Some(iv);
            }
        }
        None
    }

    pub fn bucket(&mut self, n: usize) -> Option<Vec<(K, V)>> {
        if n >= self.buckets.len() {
            return None;
        }
        let mut bucket = Vec::new();
        std::mem::swap(&mut bucket, &mut self.buckets[n]);
        self.len -= bucket.len();
        Some(bucket)
    }

    /// # Panics
    ///
    /// It panics when there is a data in the bucket list.
    pub fn set_buckets(&mut self, n: usize) {
        assert!(self.len == 0);
        for _ in self.buckets.len()..n {
            self.buckets.push(Vec::new());
        }
    }
}
