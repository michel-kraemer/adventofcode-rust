use std::fs;

fn parse_node(nodes: &Vec<usize>, i: usize, part1: bool) -> (usize, usize) {
    let mut i = i;

    let children = nodes[i];
    i += 1;
    let header_items = nodes[i];
    i += 1;

    let mut sums = Vec::with_capacity(children);
    for _ in 0..children {
        let (hi, j) = parse_node(nodes, i, part1);
        sums.push(hi);
        i = j;
    }

    let mut sum = if part1 { sums.iter().sum() } else { 0 };
    for _ in 0..header_items {
        let j = nodes[i];
        if !part1 && !sums.is_empty() {
            if j > 0 && j - 1 < sums.len() {
                sum += sums[j - 1];
            }
        } else {
            sum += j;
        }
        i += 1;
    }

    (sum, i)
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let nodes = input
            .split_whitespace()
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut sum = 0;
        let mut i = 0;
        while i < nodes.len() {
            let (hi, j) = parse_node(&nodes, i, part1);
            sum += hi;
            i = j;
        }

        println!("{}", sum);
    }
}
