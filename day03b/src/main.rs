use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

use regex::Regex;

fn is_gear(c: char) -> bool {
    c == '*'
}

fn has_gear(start: usize, end: usize, line_n: usize, lines: &Vec<String>) -> Option<[usize; 2]> {
    let s = if start > 0 { start - 1 } else { start };
    let e = if end == lines[line_n].len() { end - 1 } else { end };
    if line_n > 0 {
        let prev_line = &lines[line_n - 1];
        for i in s..=e {
            if is_gear(prev_line.chars().nth(i).unwrap()) {
                return Some([i, line_n - 1]);
            }
        }
    }
    
    let line = &lines[line_n];
    if is_gear(line.chars().nth(s).unwrap()) {
        return Some([s, line_n]);
    }
    if is_gear(line.chars().nth(e).unwrap()) {
        return Some([e, line_n]);
    }

    if line_n < lines.len() - 1 {
        let next_line = &lines[line_n + 1];
        for i in s..=e {
            if is_gear(next_line.chars().nth(i).unwrap()) {
                return Some([i, line_n + 1]);
            }
        }
    }

    None
}

fn main() {
    let r = Regex::new(r"\d+").unwrap();

    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        lines.push(line);
    }
    
    let mut sum = 0;
    let mut gears: HashMap<[usize; 2], Vec<u32>> = HashMap::new();
    for n in 0..lines.len() {
        let line = &lines[n];
        for m in r.find_iter(line) {
            let gear = has_gear(m.start(), m.end(), n, &lines);
            match gear {
                Some(g) => gears.entry(g).or_default().push(m.as_str().parse::<u32>().unwrap()),
                None => {} // nothing to do here
            }
        }
    }

    for e in gears.values() {
        if e.len() == 2 {
            let ratio = e[0] * e[1];
            sum += ratio;
        }
    }

    println!("Result: {}", sum);
}
