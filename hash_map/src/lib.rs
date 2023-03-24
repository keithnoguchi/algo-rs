//! A hash map

#![forbid(unsafe_code, missing_debug_implementations)]

use bucket_list::BucketList;

#[derive(Debug)]
pub struct HMap<K, V> {
    n_moved: usize,
    main: BucketList<K, V>,
    grow: BucketList<K, V>,
}
