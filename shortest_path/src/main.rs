//! A shortest path first.
//!
//! H - 6 - D - 18 - C - 10 - B
//! | \             / \
//! 5   7 - A - 4 -    + 12 +
//! |      / \               \
//! G - 8 -   - 3 - F - 15 - E

#![forbid(missing_debug_implementations)]

use std::collections::{HashMap, HashSet};
use std::error;
use std::fmt::{self, Debug, Display};
use std::hash::Hash;
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Graph<T, E, ID: Hash> {
    nodes: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

impl<T, E, ID: Hash> Default for Graph<T, E, ID> {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl<T, E: Weighted, ID: Clone + Debug + Eq + Hash> Graph<T, E, ID> {
    pub fn shortest_path(&self, from: ID, to: ID) -> Option<Rc<Path<ID>>> {
        let mut visited = HashSet::new();
        let mut paths = Vec::new();

        paths.push(Rc::new(Path::from(from)));
        loop {
            let current = paths.pop()?;
            if to == current.id {
                return Some(current);
            }
            if visited.contains(&current.id) {
                continue;
            }
            visited.insert(current.id.clone());

            let node = self.nodes.get(&current.id)?;
            for edge_id in &node.1 {
                let edge = self.edges.get(edge_id)?;
                let node_id = if edge.1 == current.id {
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };
                let path = Rc::new(Path {
                    id: node_id,
                    cost: current.cost + edge.0.weight(),
                    path: Some(current.clone()),
                });
                // least cost path last.
                if paths.is_empty() {
                    paths.push(path);
                    continue;
                }
                let mut i = paths.len() - 1;
                loop {
                    if path.cost < paths[i].cost {
                        paths.insert(i + 1, path);
                        break;
                    }
                    if i == 0 {
                        paths.insert(i, path);
                        break;
                    }
                    i -= 1;
                }
            }
        }
    }
}

impl<T, E, ID: Clone + Debug + Eq + Hash> Graph<T, E, ID> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, id: ID, data: T) {
        self.nodes.insert(id, (data, Vec::new()));
    }

    pub fn add_edge(&mut self, id: ID, from: ID, to: ID, data: E) -> Result<()> {
        if !self.nodes.contains_key(&from) {
            return Err(Error::from(format!("invalid from node: {:?}", from)));
        }
        if let Some(ref mut node) = self.nodes.get_mut(&to) {
            node.1.push(id.clone());
        } else {
            return Err(Error::from(format!("invalid to node: {:?}", to)));
        }
        self.edges.insert(id.clone(), (data, from.clone(), to));
        self.nodes.get_mut(&from).unwrap().1.push(id);
        Ok(())
    }
}

pub trait Weighted {
    fn weight(&self) -> i32;
}

impl Weighted for i32 {
    fn weight(&self) -> i32 {
        *self
    }
}

#[derive(Debug)]
pub struct Path<ID> {
    id: ID,
    cost: i32,
    path: Option<Rc<Self>>,
}

impl<ID: Debug> Display for Path<ID> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref path) = self.path {
            write!(f, "{}-{}-", path, self.cost)?;
        }
        write!(f, "{:?}", self.id)
    }
}

impl<ID> From<ID> for Path<ID> {
    fn from(id: ID) -> Self {
        Self {
            id,
            path: None,
            cost: 0,
        }
    }
}

impl<ID: PartialEq> Path<ID> {
    pub fn contains(&self, id: &ID) -> bool {
        if *id == self.id {
            true
        } else {
            self.path
                .as_ref()
                .map(|path| path.contains(id))
                .unwrap_or(false)
        }
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

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let mut g = Graph::new();
    for id in 'A'..='H' {
        g.add_node(id, ());
    }
    g.add_edge('a', 'H', 'D', 6)?;
    g.add_edge('b', 'D', 'C', 18)?;
    g.add_edge('c', 'C', 'B', 10)?;
    g.add_edge('d', 'H', 'A', 7)?;
    g.add_edge('e', 'A', 'C', 4)?;
    g.add_edge('f', 'H', 'G', 5)?;
    g.add_edge('g', 'G', 'A', 8)?;
    g.add_edge('h', 'A', 'F', 3)?;
    g.add_edge('i', 'F', 'E', 15)?;
    g.add_edge('j', 'C', 'E', 12)?;

    for to in 'B'..='H' {
        if let Some(path) = g.shortest_path('A', to) {
            println!("{path}");
        }
    }
    Ok(())
}
