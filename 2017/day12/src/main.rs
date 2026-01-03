use std::fs;

use rustc_hash::{FxBuildHasher, FxHashSet};

struct Node {
    parent: usize,
    size: usize,
}

fn find(x: usize, nodes: &mut [Node]) -> usize {
    if nodes[x].parent != x {
        nodes[x].parent = find(nodes[x].parent, nodes);
        return nodes[x].parent;
    }
    x
}

fn union(mut x: usize, mut y: usize, nodes: &mut [Node]) {
    x = find(x, nodes);
    y = find(y, nodes);

    if x == y {
        return;
    }

    if nodes[x].size < nodes[y].size {
        (x, y) = (y, x);
    }

    nodes[y].parent = x;
    nodes[x].size += nodes[y].size;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let (mut nodes, lines): (Vec<Node>, Vec<&str>) = input
        .lines()
        .enumerate()
        .map(|(i, l)| (Node { parent: i, size: 1 }, l))
        .unzip();

    for l in lines {
        let (from, to) = l.split_once(" <-> ").unwrap();
        let from = from.parse::<usize>().unwrap();
        for t in to.split(", ").map(|t| t.parse::<usize>().unwrap()) {
            union(from, t, &mut nodes);
        }
    }

    // part 1
    let zero = find(0, &mut nodes);
    println!("{}", nodes[zero].size);

    // part 2
    let mut groups: FxHashSet<usize> =
        FxHashSet::with_capacity_and_hasher(nodes.len(), FxBuildHasher);
    for i in 0..nodes.len() {
        let parent = find(i, &mut nodes);
        groups.insert(parent);
    }
    println!("{}", groups.len());
}
