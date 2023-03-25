//! A blob data structure.

#![forbid(unsafe_code, missing_debug_implementations)]

#[derive(Debug)]
pub enum Error {
    NoRoom,
    TooBig(u64),
    NotFound,
}
