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
    Leaf(char),
}

impl HuffmanNode {
    pub fn encode_str(&self, s: &str) -> Option<Vec<char>> {
        let mut result = Vec::new();
        for c in s.chars() {
            if let Some(v) = self.encode_char(c) {
                result.extend(v);
            }
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn encode_char(&self, c: char) -> Option<Vec<char>> {
        match self {
            Self::Tree(l, r) => {
                if let Some(mut v) = l.encode_char(c) {
                    v.insert(0, '0');
                    return Some(v);
                }
                if let Some(mut v) = r.encode_char(c) {
                    v.insert(0, '1');
                    return Some(v);
                }
                None
            }
            Self::Leaf(leaf) => {
                if c == *leaf {
                    Some(Vec::new())
                } else {
                    None
                }
            }
        }
    }

    pub fn print_depth_first(&self, depth: usize, dir: char) {
        match self {
            Self::Tree(l, r) => {
                l.print_depth_first(depth + 1, '/');
                println!("{:.<depth$}{}*", "", dir);
                r.print_depth_first(depth + 1, '\\');
            }
            Self::Leaf(c) => {
                println!("{:.<depth$}{}{}", "", dir, c);
            }
        }
    }
}

pub fn build_tree(s: &str) -> HuffmanNode {
    let mut map = BTreeMap::new();

    s.chars().for_each(|c| {
        *map.entry(c).or_insert(0) += 1;
    });
    let mut tlist: Vec<HScore> = map
        .into_iter()
        .map(|(k, v)| HScore {
            h: HuffmanNode::Leaf(k),
            score: v,
        })
        .collect();

    while tlist.len() > 1 {
        // Gets two lowest score nodes.
        let last = tlist.len() - 1;
        for i in 0..last - 1 {
            if tlist[i].score < tlist[last - 1].score {
                tlist.swap(i, last - 1);
            }
            if tlist[last - 1].score < tlist[last].score {
                tlist.swap(last - 1, last);
            }
        }
        // Combines into one HuffmanNode.
        let a = tlist.pop().unwrap();
        let b = tlist.pop().unwrap();
        tlist.push(HScore {
            h: HuffmanNode::Tree(Box::new(a.h), Box::new(b.h)),
            score: a.score + b.score,
        });
    }
    // Now we got the HuffmanTree.
    tlist.pop().unwrap().h
}

fn main() {
    let s = "at an apple app";
    println!("{s}");
    let tree = build_tree(s);
    tree.print_depth_first(0, '<');

    println!("\nencoding");
    let mut chars = std::collections::BTreeSet::new();
    s.chars().for_each(|c| {
        chars.insert(c);
    });
    for c in chars {
        println!("{:?} = {:?}", c, tree.encode_char(c));
    }

    println!("{} = {:?}", s, tree.encode_str(s));
}
