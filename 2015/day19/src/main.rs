use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(Eq, PartialEq)]
struct Molecule(String, usize);

impl Ord for Molecule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.len().cmp(&self.0.len())
    }
}

impl PartialOrd for Molecule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let (replacements, input) = input
            .split_once("\n\n")
            .map(|p| (p.0.trim(), p.1.trim()))
            .unwrap();

        let replacements = replacements
            .lines()
            .map(|c| c.split_once(" => ").unwrap())
            .collect::<Vec<_>>();

        let replacements = if part1 {
            replacements
        } else {
            replacements.into_iter().map(|r| (r.1, r.0)).collect()
        };

        let mut seen = HashSet::new();
        let mut molecules = BinaryHeap::new();
        molecules.push(Molecule(input.to_string(), 0));

        let mut result = 0;
        'outer: while let Some(m) = molecules.pop() {
            for r in &replacements {
                for i in 0..m.0.len() {
                    if m.0[i..].starts_with(r.0) {
                        let n = format!("{}{}{}", &m.0[0..i], r.1, &m.0[i + r.0.len()..]);
                        if seen.insert(n.clone()) && !part1 {
                            if n == "e" {
                                result = m.1 + 1;
                                break 'outer;
                            }
                            molecules.push(Molecule(n, m.1 + 1));
                        }
                    }
                }
            }
        }

        if part1 {
            println!("{}", seen.len());
        } else {
            println!("{result}");
        }
    }
}
