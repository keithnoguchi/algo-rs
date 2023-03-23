//! Greedy algorithm for the traveling sales person problem

#![forbid(unsafe_code, missing_debug_implementations)]

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Graph<K: Hash, V, E> {
    vertices: HashMap<K, Vertex<K, V>>,
    edges: HashMap<K, Edge<K, E>>,
}

#[derive(Debug)]
pub struct Vertex<K, T> {
    id: K,
    data: T,
}

#[derive(Debug)]
pub struct Edge<K, T> {
    id: K,
    from: K,
    to: K,
    data: T,
}

fn main() {
    println!("Hello greedy salesman");
}
