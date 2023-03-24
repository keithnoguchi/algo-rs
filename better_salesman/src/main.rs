//! An iterative approch for the traveling salesman
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
use std::ops::Add;
use std::rc::Rc;
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

impl<K, V, E> Graph<K, V, E>
where
    K: Clone + Eq + Ord + Hash,
    E: Copy + Add<Output = E> + Default + PartialOrd,
{
    /// Get the completed path of visiting all the vertices.
    pub fn complete_path(&self, vertices: &[K]) -> Option<Rc<MinPath<K, E>>> {
        if vertices.len() < 2 {
            return None;
        }

        let mut complete_path = Rc::new(MinPath::new(vertices[0].clone()));
        for v_id in &vertices[1..vertices.len() - 1] {
            if !complete_path.contains(v_id) {
                complete_path = self.shortest_path(complete_path, v_id.clone())?;
            }
        }
        self.shortest_path(complete_path, vertices[vertices.len() - 1].clone())
    }

    pub fn shortest_path(&self, start: Rc<MinPath<K, E>>, end: K) -> Option<Rc<MinPath<K, E>>> {
        let mut visited = HashSet::new();
        let mut shortest = BinaryHeap::new();
        shortest.push(start);

        while let Some(path) = shortest.pop() {
            if path.id == end {
                return Some(path);
            }
            if visited.contains(&path.id) {
                continue;
            }
            visited.insert(path.id.clone());

            let v = self.vertices.get(&path.id)?;
            for edge_id in &v.edges {
                let edge = self.edges.get(edge_id)?;
                let vertex_id = if edge.start == path.id {
                    edge.end.clone()
                } else {
                    edge.start.clone()
                };
                let candidate = Rc::new(MinPath {
                    id: vertex_id,
                    data: path.data + edge.data,
                    prev: Some(path.clone()),
                });
                shortest.push(candidate);
            }
        }
        None
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
pub struct MinPath<K, T> {
    id: K,
    data: T,
    prev: Option<Rc<Self>>,
}

impl<K: Display, T: Display> Display for MinPath<K, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = &self.prev {
            write!(f, "{path}-{}-", self.data)?;
        }
        write!(f, "{}", self.id)
    }
}

impl<K: PartialEq, T> Eq for MinPath<K, T> {}

impl<K: PartialEq, T> PartialEq for MinPath<K, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<K: PartialOrd, T: PartialOrd> PartialOrd for MinPath<K, T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.data.partial_cmp(&other.data) {
            Some(cmp::Ordering::Greater) => Some(cmp::Ordering::Less),
            Some(cmp::Ordering::Less) => Some(cmp::Ordering::Greater),
            Some(cmp::Ordering::Equal) => self.id.partial_cmp(&other.id),
            None => None,
        }
    }
}

impl<K: Ord, T: PartialOrd> Ord for MinPath<K, T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap_or(self.id.cmp(&other.id))
    }
}

impl<K: Eq, T: Default> MinPath<K, T> {
    pub fn new(id: K) -> Self {
        Self {
            id,
            data: T::default(),
            prev: None,
        }
    }

    pub fn contains(&self, id: &K) -> bool {
        if self.id == *id {
            return true;
        }
        match self.prev {
            Some(ref path) => path.contains(id),
            None => false,
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
    // Setup a graph.
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

    let mut destinations: Vec<_> = g.vertices.keys().copied().collect();
    destinations.push(destinations[0]);
    if let Some(path) = g.complete_path(&destinations) {
        println!("{path}");
    }

    Ok(())
}
