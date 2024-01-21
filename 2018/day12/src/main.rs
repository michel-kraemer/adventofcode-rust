use std::{collections::HashSet, fs};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut lines = input.lines();
        let initial_state = lines.next().unwrap();
        let (_, initial_state) = initial_state.split_once(": ").unwrap();

        lines.next();
        let mut rules = [false; 32];
        lines.for_each(|l| {
            let parts = l.split_once(" => ").unwrap();
            if parts.1 == "#" {
                let mut p = 0usize;
                for (i, c) in parts.0.chars().enumerate() {
                    if c == '#' {
                        p += 1 << i;
                    }
                }
                rules[p] = true;
            }
        });

        let mut min_i = i64::MAX;
        let mut max_i = i64::MIN;
        let mut state = HashSet::new();
        let mut prev = String::new();
        for (i, c) in initial_state.chars().enumerate() {
            if c == '#' {
                state.insert(i as i64);
                min_i = min_i.min(i as i64);
                max_i = max_i.max(i as i64);
            }
        }

        let max_len = if part1 { 20 } else { 50_000_000_000usize };
        let mut k = 0;
        while k < max_len {
            let mut new_state = HashSet::new();
            let mut new_min_i = i64::MAX;
            let mut new_max_i = i64::MIN;
            let mut current = String::new();
            for i in min_i - 2..=max_i + 2 {
                let mut p = 0usize;
                for j in 0..5 {
                    if state.contains(&(i + j - 2)) {
                        p += 1 << j;
                    }
                }
                if rules[p] {
                    new_state.insert(i);
                    new_min_i = new_min_i.min(i);
                    new_max_i = new_max_i.max(i);
                    current.push('#');
                } else {
                    current.push('.');
                }
            }
            state = new_state;
            min_i = new_min_i;
            max_i = new_max_i;

            if !part1 {
                if prev == current {
                    break;
                }
                prev = current;
            }

            k += 1;
        }

        let sum = if part1 {
            state.into_iter().sum::<i64>()
        } else {
            state
                .into_iter()
                .map(|v| v + (max_len - k - 1) as i64)
                .sum::<i64>()
        };
        println!("{}", sum);
    }
}
