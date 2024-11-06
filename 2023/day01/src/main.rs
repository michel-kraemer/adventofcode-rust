use std::fs;

static PATTERNS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn search(l: &str, r: impl Iterator<Item = usize>, part1: bool) -> u32 {
    for i in r {
        let c = l.as_bytes()[i] as char;
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }
        if !part1 {
            for (j, p) in PATTERNS.iter().enumerate() {
                if i + p.len() <= l.len() && &l[i..i + p.len()] == *p {
                    return j as u32 + 1;
                }
            }
        }
    }
    0
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();
        let mut sum = 0;
        for l in lines {
            let d1 = search(l, 0..l.len(), part1);
            let d2 = search(l, (0..l.len()).rev(), part1);
            sum += d1 * 10 + d2;
        }
        println!("{}", sum);
    }
}
