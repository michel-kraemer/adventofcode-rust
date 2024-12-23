//! This solution is probably a little bit slower than other solutions you can
//! find on the Internet but it uses a proper Clique finding algorithm that
//! works independently from the order of the edges in the input file. Other
//! solutions using a greedy algorithm are extremely fast, but may fail to find
//! the largest clique if they, for every node in the clique, accidentally
//! first visit a node that does not belong to the clique.

use std::collections::VecDeque;
use std::fs;

// all node names consist of exactly two lower-case chars between 'a' and 'z'
fn encode(node: &str) -> usize {
    debug_assert!(node.len() == 2);
    let b = node.as_bytes();
    (b[0] - b'a') as usize * 26 + (b[1] - b'a') as usize
}

fn decode(node: usize) -> String {
    let mut s = String::new();
    s.push(((node / 26) as u8 + b'a') as char);
    s.push(((node % 26) as u8 + b'a') as char);
    s
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let mut neighbors = [const { Vec::new() }; 26 * 26];
    let mut edges = [[false; 26 * 26]; 26 * 26];

    for l in lines {
        let (a, b) = l.split_once("-").unwrap();
        let ai = encode(a);
        let bi = encode(b);
        edges[ai][bi] = true;
        edges[bi][ai] = true;
        neighbors[ai].push(bi);
        neighbors[bi].push(ai);
    }

    let mut total1 = 0;
    let mut largest_clique = Vec::new();
    for (start, _) in neighbors.iter().enumerate().filter(|(_, s)| !s.is_empty()) {
        // perform BFS to find the largest clique for this node
        let mut queue = VecDeque::new();
        queue.push_back((start, vec![start]));

        while let Some((computer, seen)) = queue.pop_front() {
            // iterate over all neighbors of `computer`
            for &n in &neighbors[computer] {
                // Only add nodes lexicographically greater than the current
                // node. This serves two purposes: (a) the resulting clique will
                // be sorted, which is required for the puzzle answer, (b) it
                // automatically avoids duplicate cliques and saves us from
                // keeping a HashSet of cliques already seen. Since we apply
                // this BFS to all possible nodes, we can be sure that we will
                // find exactly one largest clique where all peers are sorted.
                if n < computer {
                    continue;
                }

                // only add this neighbor if it is a neighbor of all nodes
                // seen so far
                let nn = &edges[n];
                if seen.iter().any(|&s| !nn[s]) {
                    continue;
                }

                // extend clique
                let mut new_seen = seen.clone();
                new_seen.push(n);

                // part 1
                if new_seen.len() == 3 && new_seen.iter().any(|s| (s / 26) as u8 == b't' - b'a') {
                    total1 += 1;
                }

                // part 2
                if new_seen.len() > largest_clique.len() {
                    largest_clique = new_seen.clone();
                }

                // try to add more nodes
                queue.push_back((n, new_seen));
            }
        }
    }

    // part 1
    println!("{}", total1);

    // part 2
    println!(
        "{}",
        largest_clique
            .iter()
            .map(|&c| decode(c))
            .collect::<Vec<_>>()
            .join(",")
    );
}
