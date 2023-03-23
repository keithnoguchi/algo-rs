//! A shortest path first.

#![forbid(missing_debug_implementations)]

use std::collections::HashMap;
use std::error;
use std::fmt::{self, Debug, Display};
use std::hash::Hash;
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, Error>;

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
    path: Option<Rc<Self>>,
    cost: i32,
}

impl<ID: Debug> Display for Path<ID> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref path) = self.path {
            write!(f, "{}-{}-", path, self.cost)?;
        }
        write!(f, "{:?} ", self.id)
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

    dbg!(g);
    Ok(())
}
