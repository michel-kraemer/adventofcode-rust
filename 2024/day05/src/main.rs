use std::{cmp::Ordering, collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut edges: HashSet<(u32, u32)> = HashSet::new();

    for rule in rules.lines() {
        let (first, second) = rule.split_once("|").unwrap();
        let first = first.parse::<u32>().unwrap();
        let second = second.parse::<u32>().unwrap();
        edges.insert((first, second));
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for update in updates.lines() {
        let mut pages = update
            .split(",")
            .map(|p| p.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut ok = true;
        for (i, page) in pages.iter().enumerate() {
            if pages[i + 1..].iter().any(|x| edges.contains(&(*x, *page))) {
                ok = false;
                break;
            }
        }

        if ok {
            part1 += pages[pages.len() / 2];
        } else {
            pages.sort_unstable_by(|&a, &b| {
                if edges.contains(&(a, b)) {
                    Ordering::Less
                } else if edges.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            part2 += pages[pages.len() / 2];
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
