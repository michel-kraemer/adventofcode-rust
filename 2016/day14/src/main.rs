use std::{collections::VecDeque, fs};

use rayon::prelude::*;

fn hex_to_u8(c: u8) -> u8 {
    if c.is_ascii_digit() {
        c - b'0'
    } else {
        c - b'a' + 10
    }
}

fn find_three(s: &[u8]) -> Option<u8> {
    for i in 0..s.len() - 2 {
        let c = s[i];
        if s[i + 1] == c && s[i + 2] == c {
            return Some(hex_to_u8(c));
        }
    }
    None
}

fn find_fives(s: &[u8]) -> u64 {
    let mut result = 0;
    let mut i = 0;
    while i < s.len() - 5 {
        let c = s[i];
        if s[i + 1] == c && s[i + 2] == c && s[i + 3] == c && s[i + 4] == c {
            result |= 1 << hex_to_u8(c);
            i += 5;
        } else {
            i += 1;
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let salt = input.trim();

    let mut keys_found1 = 0;
    let mut keys_found2 = 0;

    let mut result1 = 0;
    let mut result2 = 0;

    let mut queue = VecDeque::new();
    let mut i = 0;
    loop {
        if queue.len() < 1001 {
            queue.extend(
                (0..1001)
                    .into_par_iter()
                    .map(|j| {
                        let s1 = format!(
                            "{:x}",
                            md5::compute(format!("{}{}", salt, i + queue.len() + j))
                        );
                        let mut s2 = s1.clone();
                        for _ in 0..2016 {
                            s2 = format!("{:x}", md5::compute(s2));
                        }
                        let s1_fives = find_fives(s1.as_bytes());
                        let s2_fives = find_fives(s2.as_bytes());
                        (s1, s1_fives, s2, s2_fives)
                    })
                    .collect::<Vec<_>>(),
            );
        }

        let (s1, _, s2, _) = queue.pop_front().unwrap();
        if result1 == 0 {
            let b = s1.as_bytes();
            if let Some(t) = find_three(b)
                && queue
                    .iter()
                    .take(1000)
                    .any(|(_, fives, _, _)| *fives & (1 << t) > 0)
            {
                keys_found1 += 1;
                if keys_found1 == 64 {
                    result1 = i;
                }
            }
        }
        if result2 == 0 {
            let b = s2.as_bytes();
            if let Some(t) = find_three(b)
                && queue
                    .iter()
                    .take(1000)
                    .any(|(_, _, _, fives)| *fives & (1 << t) > 0)
            {
                keys_found2 += 1;
                if keys_found2 == 64 {
                    result2 = i;
                }
            }
        }

        if result1 > 0 && result2 > 0 {
            break;
        }

        i += 1;
    }

    println!("{result1}");
    println!("{result2}");
}
