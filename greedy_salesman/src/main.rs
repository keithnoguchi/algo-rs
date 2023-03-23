//! Greedy algorithm for the traveling sales person problem
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

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Graph<K: Hash, V, E> {
    vertices: HashMap<K, Vertex<K, V>>,
    edges: HashMap<K, Edge<K, E>>,
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

    pub fn add_vertex(&mut self, v: Vertex<K, V>) {
        self.vertices.insert(v.id.clone(), v);
    }

    pub fn add_edge(&mut self, e: Edge<K, E>) -> Result<()> {
        if !self.vertices.contains_key(&e.to) {
            return Err(Error::from(format!("missing vertex {:?}", e.to)));
        }
        if let Some(v) = self.vertices.get_mut(&e.from) {
            v.edges.push(e.id.clone());
        } else {
            return Err(Error::from(format!("missing vertex {:?}", e.from)));
        }
        self.vertices
            .get_mut(&e.to)
            .unwrap()
            .edges
            .push(e.id.clone());
        self.edges.insert(e.id.clone(), e);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Vertex<K, T> {
    id: K,
    data: T,
    edges: Vec<K>,
}

impl<K, T: Debug> Display for Vertex<K, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
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
    id: K,
    data: T,
    from: K,
    to: K,
}

impl<K: Debug, T: Debug> Display for Edge<K, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}-({:?})-{:?}", self.from, self.data, self.to)
    }
}

impl<K, T> Edge<K, T> {
    pub fn new(id: K, data: T, from: K, to: K) -> Self {
        Self { id, data, from, to }
    }
}

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let mut g = Graph::new();
    for point in 'A'..='H' {
        let v = Vertex::new(point, format!("Point {point}"));
        g.add_vertex(v);
    }
    g.add_edge(Edge::new('a', 6.0, 'H', 'D'))?;
    g.add_edge(Edge::new('b', 18.0, 'D', 'C'))?;
    g.add_edge(Edge::new('c', 10.0, 'C', 'B'))?;
    g.add_edge(Edge::new('d', 7.0, 'H', 'A'))?;
    g.add_edge(Edge::new('e', 4.0, 'A', 'C'))?;
    g.add_edge(Edge::new('f', 12.0, 'C', 'E'))?;
    g.add_edge(Edge::new('g', 5.0, 'H', 'G'))?;
    g.add_edge(Edge::new('h', 8.0, 'G', 'A'))?;
    g.add_edge(Edge::new('i', 3.0, 'A', 'F'))?;
    g.add_edge(Edge::new('j', 15.0, 'F', 'E'))?;
    dbg!(g);
    Ok(())
}
