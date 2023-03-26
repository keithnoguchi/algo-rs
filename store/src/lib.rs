#![forbid(unsafe_code, missing_debug_implementations)]

use std::fs::{File, OpenOptions};

use blob::Error;
use blob::Blob;

#[derive(Debug)]
pub struct Store {
    file: File,
    hseed: u64,
    block_size: u64,
    nblocks: u64,
}
