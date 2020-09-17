#![allow(dead_code)]
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

pub fn encode(data: &str, code: &HashMap<char, Vec<u8>>) -> Vec<u8> {
    let mut nbits = 0;
    data.chars().for_each(|c| {
        nbits += code.get(&c).unwrap().len();
    });
    let mut ret = Vec::<u8>::with_capacity(nbits);
    data.chars().for_each(|c| {
        let v = code.get(&c).unwrap();
        v.iter().for_each(|bit| ret.push(*bit));
    });
    ret
}
pub fn decode(data: Vec<u8>, code: &HashMap<char, Vec<u8>>) -> String {
    fn reverse(h: &HashMap<char, Vec<u8>>) -> HashMap<Vec<u8>, char> {
        let mut ret = HashMap::new();
        h.iter().for_each(|(k, v)| {
            ret.insert(v.clone(), *k);
        });
        ret
    }
    let code = reverse(code);
    let mut temp = Vec::<u8>::new();
    let mut ret = String::new();
    data.into_iter().for_each(|b| {
        temp.push(b);
        if code.contains_key(&temp) {
            ret.push(*code.get(&temp).unwrap());
            temp.clear();
        }
    });
    ret
}

#[derive(Eq, Debug, Clone)]
struct Tree {
    count: i32,
    value: Option<char>,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Ord for Tree {
    fn cmp(&self, other: &Tree) -> Ordering {
        (self.count).cmp(&(other.count))
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Tree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Tree) -> bool {
        self.count == other.count
    }
}

impl Tree {
    fn new(value: char, count: i32) -> Box<Tree> {
        let t = Tree {
            count,
            value: Some(value),
            left: None,
            right: None,
        };

        Box::new(t)
    }

    fn merge(tree_smaller: Box<Tree>, tree_larger: Box<Tree>) -> Box<Tree> {
        let t = Tree {
            count: tree_smaller.count + tree_larger.count,
            value: None,
            left: Some(tree_smaller),
            right: Some(tree_larger),
        };

        Box::new(t)
    }
}

fn frequency(n: &str) -> HashMap<char, i32> {
    let mut output: HashMap<char, i32> = HashMap::new();
    n.chars().for_each(|c| {
        let new = if let Some(o) = output.get(&c) {
            o + 1i32
        } else {
            1i32
        };
        output.insert(c, new);
    });
    output
}

fn map_to_heap(map: HashMap<char, i32>) -> BinaryHeap<Box<Tree>> {
    let mut heap = BinaryHeap::new();
    map.into_iter().for_each(|(l, c)| {
        let t = Tree::new(l, c);
        heap.push(t);
    });
    heap
}

fn heap_to_tree(mut heap: BinaryHeap<Box<Tree>>) -> Box<Tree> {
    while heap.len() > 1 {
        let (t1, t2) = (heap.pop().unwrap(), heap.pop().unwrap());
        heap.push(Tree::merge(t1, t2));
    }
    heap.pop().unwrap()
}

fn tree_to_codes(
    root: &Option<Box<Tree>>,
    prefix: Vec<u8>,
    mut map: HashMap<char, Vec<u8>>,
) -> HashMap<char, Vec<u8>> {
    if let Some(ref tree) = *root {
        match tree.value {
            Some(t) => {
                map.insert(t, prefix);
            }
            None => {
                let (mut prefix_l, mut prefix_r) = (prefix.clone(), prefix);
                prefix_l.push(1u8);
                let map = tree_to_codes(&tree.left, prefix_l, map);
                prefix_r.push(0u8);
                return tree_to_codes(&tree.right, prefix_r, map);
            }
        }
    }
    map
}

fn huffman_codes(data: &str) -> HashMap<char, Vec<u8>> {
    let f_map = frequency(data);
    let heap = map_to_heap(f_map);
    let tree = heap_to_tree(heap);
    tree_to_codes(&Some(tree), Vec::new(), HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_counts_simple_string() {
        let a = "abcdeabaccaaaaa";

        let res_fn = frequency(a);
        let res: HashMap<_, _> = a.chars().zip(vec![8, 2, 3, 1, 1]).collect();

        assert_eq!(res_fn, res);
    }

    #[test]
    fn decoding_is_inverse() {
        let s = "abcdeabaccaaaaa";

        let hc = huffman_codes(s);

        let s2 = "babbc";

        let encoded = encode(s2, &hc);
        let decoded = decode(encoded, &hc);

        assert_eq!(s2, decoded);
    }
}
