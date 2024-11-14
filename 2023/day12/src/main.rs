use std::{collections::HashMap, fs};

fn compare(s: &[char], cs: &[usize], cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(prev) = cache.get(&(s.len(), cs.len())) {
        return *prev;
    }

    if cs.is_empty() {
        // we don't expect more damaged springs; if the remainder of s only
        // consists of operational springs, it's a match
        if s.iter().all(|c| *c != '#') {
            return 1;
        } else {
            return 0;
        }
    }

    let mut r = 0;
    for i in 0..s.len() {
        // skip operational springs at the beginning
        let s = &s[i..];

        if s.len() < cs[0] {
            // remaining string is too short to match
            break;
        }

        if s.iter().take(cs[0]).all(|c| *c != '.') {
            // found a match
            if s.len() == cs[0] && cs.len() == 1 {
                // we don't expect more damaged springs and reached the end of s
                r += 1;
            } else if s.len() > cs[0] && s[cs[0]] != '#' {
                // there's at least one operational spring after the match;
                // try to match next sequence of damaged springs
                r += compare(&s[cs[0] + 1..], &cs[1..], cache);
            }
        }

        if s[0] == '#' {
            break;
        }
    }

    cache.insert((s.len(), cs.len()), r);
    r
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines: Vec<(&str, Vec<usize>)> = input
            .lines()
            .map(|l| {
                let (str, rest) = l.split_once(' ').unwrap();
                (str, rest.split(',').map(|n| n.parse().unwrap()).collect())
            })
            .collect();

        let repeat = if part1 { 1 } else { 5 };
        let mut total = 0;
        for l in lines {
            let str = [l.0].repeat(repeat).join("?").chars().collect::<Vec<_>>();
            let counts = l.1.repeat(repeat);

            let mut cache = HashMap::new();
            total += compare(&str, &counts, &mut cache);
        }

        println!("{}", total);
    }
}
