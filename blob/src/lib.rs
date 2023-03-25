//! A blob data structure.

#![forbid(unsafe_code, missing_debug_implementations)]

use std::io;
use std::result;

type Result<T> = result::Result<T, Error>;

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

pub fn read_u64<R: io::Read>(r: &mut R) -> Result<u64> {
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf)?;
    Ok(bincode::deserialize(&buf)?)
}

pub fn write_u64<W: io::Write>(w: &mut W, data: u64) -> Result<()> {
    let data = bincode::serialize(&data)?;
    Ok(w.write_all(&data)?)
}
