use std::fs;

fn generate(a: &str, min_len: usize) -> String {
    let mut a = a.to_string();
    loop {
        let b = a
            .chars()
            .rev()
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect::<String>();
        a = format!("{}0{}", a, b);
        if a.len() >= min_len {
            return a;
        }
    }
}

fn checksum(s: &str) -> String {
    let mut checksum = s.to_string();
    loop {
        checksum = checksum
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|t| if t[0] == t[1] { '1' } else { '0' })
            .collect::<String>();
        if checksum.len() % 2 == 1 {
            return checksum;
        }
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let a = input.trim();
        let min_len = if part1 { 272 } else { 35651584 };
        let data = generate(a, min_len);
        let c = checksum(&data[0..min_len]);
        println!("{}", c);
    }
}
