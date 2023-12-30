use std::{fs, str::from_utf8};

fn decompress(s: &[u8], part1: bool) -> usize {
    let mut i = 0;
    let mut result = 0;

    while i < s.len() {
        if s[i] == b'(' {
            let mut e = i;
            let mut j = 0;
            let mut r = 0;

            while e < s.len() {
                if s[e] == b'x' {
                    j = from_utf8(&s[i + 1..e]).unwrap().parse::<usize>().unwrap();
                    r = e + 1;
                } else if s[e] == b')' {
                    r = from_utf8(&s[r..e]).unwrap().parse::<usize>().unwrap();
                    break;
                }
                e += 1;
            }

            let add = if part1 {
                j
            } else {
                decompress(&s[e + 1..e + j + 1], part1)
            };

            i = e + j + 1;
            result += add * r;
        } else {
            i += 1;
            result += 1;
        }
    }

    result
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let result: usize = input.lines().map(|l| decompress(l.as_bytes(), part1)).sum();
        println!("{}", result);
    }
}
