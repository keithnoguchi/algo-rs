//! Greedy algorithm for the traveling sales person problem
//!
//! H - 6 - D - 18 - C - 10 - B
//! | \             / \
//! 5   7 - A - 4 -    + 12 +
//! |      / \               \
//! G - 8 -   - 3 - F - 15 - E

#![forbid(unsafe_code, missing_debug_implementations)]

use std::cmp::{self, Ord};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error;
use std::fmt::{self, Debug, Display};
use std::hash::Hash;
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Graph<K: Hash, V, E> {
    vertices: HashMap<K, Vertex<K, V>>,
    edges: HashMap<K, Edge<K, E>>,
}

pub trait Weighted {
    fn weight(&self) -> i32;
}

impl Weighted for f32 {
    fn weight(&self) -> i32 {
        *self as i32
    }
}

impl<K: Hash, V, E> Default for Graph<K, V, E> {
    fn default() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl<K: Clone + Eq + Ord + Hash, V, E: Weighted> Graph<K, V, E> {
    pub fn greedy_salesman(&self, from: K) -> Option<Rc<Path<K>>> {
        let mut start = Rc::new(Path::from(from.clone()));
        let mut dests: HashSet<_> = self
            .vertices
            .keys()
            .filter(|&v| *v != from)
            .cloned()
            .collect();
        let mut visited = HashSet::new();
        visited.insert(from);

        while let Some(path) = self.closest(start, &dests) {
            dests.retain(|v| !visited.contains(v));
            if dests.is_empty() {
                return Some(path);
            }
            visited.insert(path.id.clone());
            start = path;
        }
        None
    }

    pub fn shortest_path(&self, from: K, to: K) -> Option<Rc<Path<K>>> {
        let from = Rc::new(Path::from(from));
        let mut to_set = HashSet::new();
        to_set.insert(to);
        self.closest(from, &to_set)
    }

    fn closest(&self, from: Rc<Path<K>>, to: &HashSet<K>) -> Option<Rc<Path<K>>> {
        let mut visited = HashSet::new();

        let mut candidates = BinaryHeap::new();
        candidates.push(from);
        while let Some(path) = candidates.pop() {
            // Reaches to the destination.
            if to.contains(&path.id) {
                return Some(path);
            }

            // Checks the loop.
            if visited.contains(&path.id) {
                continue;
            }
            visited.insert(path.id.clone());

            // Searches for the next vertices.
            let v = self.vertices.get(&path.id)?;
            for edge_id in &v.edges {
                let edge = self.edges.get(edge_id)?;
                let id = if edge.from == path.id {
                    edge.to.clone()
                } else {
                    edge.from.clone()
                };
                let nexthop = Rc::new(Path {
                    id,
                    prev: Some(path.clone()),
                    weight: path.weight + edge.data.weight(),
                });
                candidates.push(nexthop);
            }
        }
        None
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
pub struct Path<K> {
    id: K,
    weight: i32,
    prev: Option<Rc<Self>>,
}

impl<K> From<K> for Path<K> {
    fn from(id: K) -> Self {
        Self {
            id,
            prev: None,
            weight: 0,
        }
    }
}

impl<K: Display> Display for Path<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { id, weight, prev } = self;
        if let Some(path) = prev {
            // depth first display.
            write!(f, "{path}-{weight}-")?;
        }
        write!(f, "{id}")
    }
}

impl<K: PartialEq> Eq for Path<K> {}

impl<K: PartialEq> PartialEq for Path<K> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<K: Ord> Ord for Path<K> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Less weight is the higher priority.
        match self.weight.cmp(&other.weight) {
            cmp::Ordering::Less => cmp::Ordering::Greater,
            cmp::Ordering::Greater => cmp::Ordering::Less,
            cmp::Ordering::Equal => self.id.cmp(&other.id),
        }
    }
}

impl<K: PartialEq> PartialOrd for Path<K> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        // Less weight is the higher priority.
        match self.weight.cmp(&other.weight) {
            cmp::Ordering::Less => Some(cmp::Ordering::Greater),
            cmp::Ordering::Greater => Some(cmp::Ordering::Less),
            cmp::Ordering::Equal => Some(cmp::Ordering::Equal),
        }
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

    for from in 'A'..='H' {
        if let Some(path) = g.greedy_salesman(from) {
            println!("{path}");
        }
    }
    Ok(())
}
