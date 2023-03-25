//! An ID generator.

#![forbid(unsafe_code, missing_debug_implementations)]

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenData {
    pos: usize,
    gen: u64,
}

#[derive(Debug)]
pub struct EntityActive {
    active: bool,
    gen: u64,
}

#[derive(Debug)]
pub struct GenManager {
    items: Vec<EntityActive>,
    drops: Vec<usize>,
}

impl Default for GenManager {
    fn default() -> Self {
        Self {
            items: vec![],
            drops: vec![],
        }
    }
}

fn main() {}
