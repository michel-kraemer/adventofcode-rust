use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut valid = 0;
        for l in input.lines() {
            let mut p = l
                .split_whitespace()
                .map(|w| {
                    let mut c = w.chars().collect::<Vec<_>>();
                    if !part1 {
                        c.sort();
                    }
                    String::from_iter(c.into_iter())
                })
                .collect::<Vec<_>>();
            let len = p.len();
            p.sort();
            p.dedup();
            if len == p.len() {
                valid += 1;
            }
        }

        println!("{}", valid);
    }
}
