//! A blob data structure.

#![forbid(unsafe_code, missing_debug_implementations)]

use std::io;
use std::result;

use serde::{Deserialize, Serialize};

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Blob {
    k: Vec<u8>,
    v: Vec<u8>,
}

impl Blob {
    pub fn from<K: Serialize, V: Serialize>(k: &K, v: &V) -> Result<Self> {
        Ok(Self {
            k: bincode::serialize(k)?,
            v: bincode::serialize(v)?,
        })
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn len(&self) -> usize {
        16 + self.k.len() + self.v.len()
    }

    pub fn k_hash(&self, seed: u64) -> u64 {
        hasher::hash(seed, &self.k)
    }

    pub fn key_match(&self, rhs: &Self) -> bool {
        self.k == rhs.k
    }

    pub fn get_v<'a, V: Deserialize<'a>>(&'a self) -> Result<V> {
        Ok(bincode::deserialize(&self.v)?)
    }

    pub fn write<W: io::Write>(&self, w: &mut W) -> Result<()> {
        Self::write_u64(w, self.k.len() as u64)?;
        Self::write_u64(w, self.v.len() as u64)?;
        w.write_all(&self.k)?;
        w.write_all(&self.v)?;
        Ok(())
    }

    pub fn read<R: io::Read>(r: &mut R) -> Result<Self> {
        let klen = Self::read_u64(r)? as usize;
        let vlen = Self::read_u64(r)? as usize;
        let mut k = vec![0u8; klen];
        let mut v = vec![0u8; vlen];
        r.read_exact(&mut k)?;
        r.read_exact(&mut v)?;
        Ok(Self { k, v })
    }

    pub fn read_u64<R: io::Read>(r: &mut R) -> Result<u64> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        Ok(bincode::deserialize(&buf)?)
    }

    pub fn write_u64<W: io::Write>(w: &mut W, data: u64) -> Result<()> {
        let data = bincode::serialize(&data)?;
        Ok(w.write_all(&data)?)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No Room")]
    NoRoom,
    #[error("Too Big {}", 0)]
    TooBig(u64),
    #[error("Not Found")]
    NotFound,
    #[error("Bincode {}", 0)]
    BinCode(bincode::Error),
    #[error("IO {}", 0)]
    Io(io::Error),
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Self::BinCode(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

#[cfg(test)]
mod test;
