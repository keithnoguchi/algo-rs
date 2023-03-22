//! A Huffman Encoding

#![forbid(missing_debug_implementations)]

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct HScore {
    h: HuffmanNode,
    score: i32,
}

#[derive(Debug)]
pub enum HuffmanNode {
    Tree(Box<Self>, Box<Self>),
    Lead(char),
}

pub fn build_tree(s: &str) {
    let mut map = BTreeMap::new();

    for c in s.chars() {
        let mut e = map.entry(c).or_insert(0);
        *e += 1;
    }
    let mut tlist: Vec<HScore> = map
        .into_iter()
        .map(|(k, v)| {
            HScore {
                h: HuffmanNode::Lead(k),
                score: v,
            }
        })
        .collect();
    println!("{tlist:#?}");
}

fn main() {
    build_tree("Hello, world!");
}
