//! An iterative approch for the traveling salesman
//!
//! H - 6 - D - 18 - C - 10 - B
//! | \             / \
//! 5   7 - A - 4 -    + 12 +
//! |      / \               \
//! G - 8 -   - 3 - F - 15 - E

#![forbid(unsafe_code, missing_debug_implementations)]

use std::collections::HashMap;
use std::error;
use std::fmt::{self, Debug, Display};
use std::hash::Hash;
use std::result;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Graph<K: Hash, V, E> {
    vertices: HashMap<K, Vertex<K, V>>,
    edges: HashMap<usize, Edge<K, E>>,
}

impl<K: Hash, V, E> Default for Graph<K, V, E> {
    fn default() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl<K: Clone + Debug + Eq + Hash, V, E> Graph<K, V, E> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vertex(&mut self, vertex: Vertex<K, V>) {
        self.vertices.insert(vertex.id.clone(), vertex);
    }

    pub fn add_edge(&mut self, edge: Edge<K, E>) -> Result<()> {
        if !self.vertices.contains_key(&edge.start) {
            return Err(Error(format!("invalid start vertex: {:?}", edge.start)));
        }
        if let Some(v) = self.vertices.get_mut(&edge.end) {
            v.edges.push(edge.id);
        } else {
            return Err(Error(format!("invalid end vertex: {:?}", edge.end)));
        }
        self.vertices
            .get_mut(&edge.start)
            .unwrap()
            .edges
            .push(edge.id);
        self.edges.insert(edge.id, edge);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Vertex<K, T> {
    id: K,
    data: T,
    edges: Vec<usize>,
}

impl<K, T> Vertex<K, T> {
    pub fn new(id: K, data: T) -> Self {
        Self {
            id,
            data,
            edges: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Edge<K, T> {
    id: usize,
    start: K,
    end: K,
    data: T,
}

impl<K, T> Edge<K, T> {
    pub fn new(start: K, end: K, data: T) -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        let id = ID.fetch_add(1, Relaxed);
        Self {
            id,
            start,
            end,
            data,
        }
    }
}

#[derive(Debug)]
pub struct Error(String);

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() -> result::Result<(), Box<dyn error::Error>> {
    // Setup graph.
    let mut g = Graph::new();
    for k in 'A'..='H' {
        g.add_vertex(Vertex::new(k, ()));
    }
    g.add_edge(Edge::new('A', 'C', 4.0))?;
    g.add_edge(Edge::new('A', 'G', 8.0))?;
    g.add_edge(Edge::new('A', 'F', 3.0))?;
    g.add_edge(Edge::new('A', 'H', 7.0))?;
    g.add_edge(Edge::new('C', 'B', 10.0))?;
    g.add_edge(Edge::new('C', 'D', 18.0))?;
    g.add_edge(Edge::new('C', 'E', 12.0))?;
    g.add_edge(Edge::new('D', 'H', 6.0))?;
    g.add_edge(Edge::new('E', 'F', 15.0))?;
    g.add_edge(Edge::new('G', 'H', 5.0))?;
    dbg!(g);
    Ok(())
}
