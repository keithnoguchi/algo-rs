//! A blob store
//!
//! This store will act as one half of the hashmap as with
//! the hashmap wrap this in somthing to make growing work.

#![forbid(unsafe_code, missing_debug_implementations)]

use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

use serde::{Deserialize, Serialize};

use blob::{self, Blob, Result};

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
        fp.rewind()?;
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
        fp.rewind()?;
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

    pub fn new_or_open(fname: &str, block_size: u64, nblocks: u64) -> Result<Self> {
        Self::new(fname, block_size, nblocks).or_else(|_e| Self::open(fname))
    }

    pub fn inc_elems(&mut self, n: i32) -> Result<()> {
        if n > 0 {
            self.elems += n as u64;
        } else {
            let n2 = (-n) as u64;
            if self.elems > n2 {
                self.elems -= n2;
            }
        }
        self.file.seek(SeekFrom::Start(24))?;
        Blob::write_u64(&mut self.file, self.elems)?;
        Ok(())
    }

    fn insert_only<K: Serialize, V: Serialize>(&mut self, k: K, v: V) -> Result<()> {
        let blob = Blob::from(&k, &v)?;
        if blob.len() > self.block_size as usize {
            return Err(blob::Error::TooBig(blob.len()));
        }
        let bucket = blob.k_hash(self.hseed) % self.nblocks;
        let f = &mut self.file;
        let mut pos = f.seek(SeekFrom::Start(COUNT_SIZE + self.block_size + bucket))?;

        loop {
            if pos > COUNT_SIZE + self.block_size * (bucket + 1) {
                return Err(blob::Error::NoRoom);
            }
            let klen = Blob::read_u64(f)?;
            let vlen = Blob::read_u64(f)?;
            if klen == 0 && (blob.len() as u64) < vlen {
                f.seek(SeekFrom::Start(pos))?;
                blob.write(f)?;
                Blob::write_u64(f, 0)?;
                Blob::write_u64(f, (vlen - blob.len() as u64) - 16)?;
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod test;
