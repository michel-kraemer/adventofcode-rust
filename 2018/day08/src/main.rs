use std::fs;

fn parse_node<I>(nodes: &mut I, part1: bool) -> usize
where
    I: Iterator<Item = usize>,
{
    let children = nodes.next().unwrap();
    let header_items = nodes.next().unwrap();

    let mut sums = Vec::with_capacity(children);
    for _ in 0..children {
        sums.push(parse_node(nodes, part1));
    }

    let mut sum = if part1 { sums.iter().sum() } else { 0 };
    for _ in 0..header_items {
        let j = nodes.next().unwrap();
        if !part1 && !sums.is_empty() {
            if j > 0 && j - 1 < sums.len() {
                sum += sums[j - 1];
            }
        } else {
            sum += j;
        }
    }

    sum
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut nodes = input
            .split_whitespace()
            .map(|e| e.parse::<usize>().unwrap());

        let sum = parse_node(&mut nodes, part1);
        println!("{}", sum);
    }
}
