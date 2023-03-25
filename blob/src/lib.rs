//! A blob data structure.

#![forbid(unsafe_code, missing_debug_implementations)]

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
    Io(std::io::Error),
}
