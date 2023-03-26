//! A blob store
//!
//! This store will act as one half of the hashmap as with
//! the hashmap wrap this in somthing to make growing work.

#![forbid(unsafe_code, missing_debug_implementations)]

use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

use blob::Blob;
use blob::Result;

const COUNT_SIZE: u64 = 32;

#[derive(Debug)]
pub struct Store {
    file: File,
    hseed: u64,
    block_size: u64,
    nblocks: u64,
    elems: u64,
}

impl Store {
    pub fn new(fname: &str, block_size: u64, nblocks: u64) -> Result<Self> {
        let hseed = rand::random::<u64>();

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .open(fname)?;
        let fp = &mut file;
        fp.set_len(COUNT_SIZE + block_size * nblocks)?;
        fp.seek(SeekFrom::Start(0))?;
        Blob::write_u64(fp, hseed)?;
        Blob::write_u64(fp, block_size)?;
        Blob::write_u64(fp, nblocks)?;
        Blob::write_u64(fp, 0)?; // 0 elems in new store.

        // mark beginnings of each block to show empty.
        for x in 0..nblocks {
            fp.seek(SeekFrom::Start(COUNT_SIZE + x * block_size))?;
            Blob::write_u64(fp, 0)?; // key length 0 means no item.
            Blob::write_u64(fp, block_size - 16)?;
        }
        Ok(Self {
            file,
            hseed,
            block_size,
            nblocks,
            elems: 0,
        })
    }

    pub fn open(fname: &str) -> Result<Self> {
        let mut file = OpenOptions::new().write(true).read(true).open(fname)?;
        let fp = &mut file;
        fp.seek(SeekFrom::Start(0))?;
        let hseed = Blob::read_u64(fp)?;
        let block_size = Blob::read_u64(fp)?;
        let nblocks = Blob::read_u64(fp)?;
        let elems = Blob::read_u64(fp)?;
        Ok(Self {
            file,
            hseed,
            block_size,
            nblocks,
            elems,
        })
    }
}
