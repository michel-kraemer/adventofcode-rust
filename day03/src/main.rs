use std::{
    collections::HashMap,
    fs,
};

use regex::Regex;

fn is_gear(c: char, part1: bool) -> bool {
    if part1 {
        !c.is_ascii_digit() && c != '.'
    } else {
        c == '*'
    }
}

fn has_gear(start: usize, end: usize, line_n: usize, lines: &Vec<Vec<char>>, part1: bool) -> Option<[usize; 2]> {
    let s = if start > 0 { start - 1 } else { start };
    let e = if end == lines[line_n].len() {
        end - 1
    } else {
        end
    };
    if line_n > 0 {
        let prev_line = &lines[line_n - 1];
        for i in s..=e {
            if is_gear(prev_line[i], part1) {
                return Some([i, line_n - 1]);
            }
        }
    }

    let line = &lines[line_n];
    if is_gear(line[s], part1) {
        return Some([s, line_n]);
    }
    if is_gear(line[e], part1) {
        return Some([e, line_n]);
    }

    if line_n < lines.len() - 1 {
        let next_line = &lines[line_n + 1];
        for i in s..=e {
            if is_gear(next_line[i], part1) {
                return Some([i, line_n + 1]);
            }
        }
    }

    None
}

fn main() {
    for part1 in [true, false] {
        let r = Regex::new(r"\d+").unwrap();

        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut sum = 0;
        let mut gears: HashMap<[usize; 2], Vec<usize>> = HashMap::new();
        for n in 0..lines.len() {
            let line = &lines[n];
            for m in r.find_iter(&line.iter().collect::<String>()) {
                let gear = has_gear(m.start(), m.end(), n, &lines, part1);
                match gear {
                    Some(g) => gears
                        .entry(g)
                        .or_default()
                        .push(m.as_str().parse().unwrap()),
                    None => {} // nothing to do here
                }
            }
        }

        for e in gears.values() {
            if part1 {
                sum += e.iter().sum::<usize>();
            } else {
                if e.len() == 2 {
                    let ratio = e[0] * e[1];
                    sum += ratio;
                }
            }
        }

        println!("{sum}");
    }
}
