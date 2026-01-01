use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let s = input.trim().bytes().collect::<Vec<_>>();

    let mut total1 = 0;
    let mut total2 = 0;
    for (i, &c) in s.iter().enumerate() {
        let j1 = (i + 1) % s.len();
        let j2 = (i + s.len() / 2) % s.len();
        if c == s[j1] {
            total1 += (c - b'0') as u64;
        }
        if c == s[j2] {
            total2 += (c - b'0') as u64;
        }
    }
    println!("{total1}");
    println!("{total2}");
}
