use std::{collections::HashMap, fs};

fn count(a: &str) -> Vec<(char, u64)> {
    let mut result = HashMap::new();
    for c in a.chars() {
        *result.entry(c).or_default() += 1;
    }
    result.into_iter().collect()
}

fn rank(a: &str, part1: bool) -> usize {
    let mut l = count(a);

    if !part1 && l.len() > 1 {
        for i in 0..l.len() {
            if l[i].0 == '1' {
                let e = l.remove(i);
                l.iter_mut().max_by_key(|e| e.1).unwrap().1 += e.1;
                break;
            }
        }
    }

    if l.len() == 1 {
        7 // five of a kind
    } else if l.len() == 2 {
        if l.iter().any(|e| e.1 == 4) {
            6 // four of a kind
        } else {
            5 // full house
        }
    } else if l.len() == 3 {
        if l.iter().any(|e| e.1 == 3) {
            4 // three of a kind
        } else {
            3 // two pair
        }
    } else if l.len() == 4 {
        2 // one pair
    } else {
        1 // high card
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut lines = input
            .lines()
            .map(|l| {
                let (hand, bid) = l.split_once(" ").unwrap();
                let hand = hand
                    .replace("J", if part1 { "U" } else { "1" })
                    .replace("Q", "V")
                    .replace("K", "W")
                    .replace("A", "X");
                (hand, bid.parse::<usize>().unwrap())
            })
            .collect::<Vec<_>>();

        lines.sort_unstable_by(|(a, _), (b, _)| {
            let ra = rank(a, part1);
            let rb = rank(b, part1);
            ra.cmp(&rb).then(a.cmp(b))
        });

        let total = lines
            .into_iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum::<usize>();

        println!("{:?}", total);
    }
}
