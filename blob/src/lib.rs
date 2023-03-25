//! A blob data structure.

#![forbid(unsafe_code, missing_debug_implementations)]

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No Room")]
    NoRoom,
    #[error("Too Big")]
    TooBig(u64),
    #[error("Not Found")]
    NotFound,
    #[error("Bincode Error")]
    BinCode(bincode::Error),
    #[error("IO Error")]
    Io(std::io::Error),
}
