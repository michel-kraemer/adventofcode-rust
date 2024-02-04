use std::{collections::HashMap, fs};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let range = input.trim().split_once('-').unwrap();
        let range = range.0.parse::<usize>().unwrap()..=range.1.parse::<usize>().unwrap();

        let mut total = 0;
        for i in range {
            let s = format!("{}", i);
            if s.len() != 6 {
                continue;
            }

            let mut increasing = true;
            let mut last_digit = 0;
            let mut same: HashMap<u32, u32> = HashMap::new();
            for c in s.chars() {
                let digit = c.to_digit(10).unwrap();
                if digit < last_digit {
                    increasing = false;
                    break;
                }
                if digit == last_digit {
                    *same.entry(digit).or_default() += 1;
                }
                last_digit = digit;
            }

            let ok = increasing
                && match part1 {
                    true => !same.is_empty(),
                    false => same.values().any(|&v| v == 1),
                };
            if ok {
                total += 1;
            }
        }

        println!("{}", total);
    }
}
