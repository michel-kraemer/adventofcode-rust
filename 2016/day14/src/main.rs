use std::{collections::VecDeque, fs};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn find_three(s: &[u8]) -> Option<u8> {
    for i in 0..s.len() - 2 {
        let c = s[i];
        if s[i + 1] == c && s[i + 2] == c {
            return Some(c);
        }
    }
    None
}

fn has_five(b: &[u8], l: u8) -> bool {
    for i in 0..b.len() - 4 {
        if b[i] == l && b[i + 1] == l && b[i + 2] == l && b[i + 3] == l && b[i + 4] == l {
            return true;
        }
    }
    false
}

fn find_last_index(salt: &str, part1: bool) -> usize {
    let mut keys_found = 0;
    let mut queue = VecDeque::new();
    let mut i = 0;
    loop {
        if queue.len() < 1001 {
            queue.extend(
                (0..4000)
                    .into_par_iter()
                    .map(|j| {
                        let mut s = format!(
                            "{:x}",
                            md5::compute(format!("{}{}", salt, i + queue.len() + j))
                        );
                        if !part1 {
                            for _ in 0..2016 {
                                s = format!("{:x}", md5::compute(s));
                            }
                        }
                        s
                    })
                    .collect::<Vec<_>>(),
            );
        }

        let s = queue.pop_front().unwrap();
        let b = s.as_bytes();
        if let Some(t) = find_three(b) {
            for o in queue.iter().take(1000) {
                if has_five(o.as_bytes(), t) {
                    keys_found += 1;
                    if keys_found == 64 {
                        return i;
                    }
                    break;
                }
            }
        }

        i += 1;
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let salt = input.trim();
        let last_index = find_last_index(salt, part1);
        println!("{:?}", last_index);
    }
}
