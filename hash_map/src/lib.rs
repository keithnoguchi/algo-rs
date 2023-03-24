//! A hash map

#![forbid(unsafe_code, missing_debug_implementations)]

use std::borrow::{Borrow, BorrowMut};
use std::hash::Hash;

use bucket_list::BucketList;

// Each bucket size before growing the number of buckets.
const BSIZE: usize = 8;

#[derive(Debug)]
pub struct HMap<K, V> {
    n_moved: usize,
    main: BucketList<K, V>,
    grow: BucketList<K, V>,
}

impl<K, V> Default for HMap<K, V> {
    fn default() -> Self {
        Self {
            n_moved: 0,
            main: BucketList::default(),
            grow: BucketList::default(),
        }
    }
}

impl<K: Eq + Hash, V> HMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, k: K, v: V) {
        if let Some(iv) = self.main.get_mut(&k) {
            *iv = v;
            return;
        }
        if let Some(iv) = self.grow.get_mut(&k) {
            *iv = v;
            return;
        }

        if self.n_moved > 0 {
            self.grow.push(k, v);
            return;
        }
        if self.main.push(k, v) > BSIZE / 2 {
            // grow buckets.
        }
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.main.get(k).or_else(|| self.grow.get(k))
    }

    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: BorrowMut<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.main.get_mut(k).or_else(|| self.grow.get_mut(k))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.main.len() + self.grow.len()
    }
}
