use std::{collections::HashMap, fs};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let codes = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut n: Vec<HashMap<char, usize>> = vec![HashMap::with_capacity(26); codes[0].len()];
        for c in codes {
            for i in 0..c.len() {
                *n[i].entry(c[i]).or_default() += 1;
            }
        }

        let v = n
            .into_iter()
            .map(|i| {
                let mut s = i.into_iter().collect::<Vec<_>>();
                s.sort_by(|a, b| b.1.cmp(&a.1));
                s[if part1 { 0 } else { s.len() - 1 }].0
            })
            .collect::<Vec<_>>();

        println!("{}", String::from_iter(v));
    }
}
