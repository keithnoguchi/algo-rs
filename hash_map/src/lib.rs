//! A hash map

#![forbid(unsafe_code, missing_debug_implementations)]

use bucket_list::BucketList;

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
