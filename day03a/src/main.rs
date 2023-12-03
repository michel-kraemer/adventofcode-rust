use std::{fs::File, io::{BufReader, BufRead}};

use regex::Regex;

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn has_symbol(start: usize, end: usize, line_n: usize, lines: &Vec<String>) -> bool {
    let s = if start > 0 { start - 1 } else { start };
    let e = if end == lines[line_n].len() { end - 1 } else { end };
    if line_n > 0 {
        let prev_line = &lines[line_n - 1];
        for i in s..=e {
            if is_symbol(prev_line.chars().nth(i).unwrap()) {
                return true;
            }
        }
    }
    
    let line = &lines[line_n];
    if is_symbol(line.chars().nth(s).unwrap()) || is_symbol(line.chars().nth(e).unwrap()) {
        return true;
    }

    if line_n < lines.len() - 1 {
        let next_line = &lines[line_n + 1];
        for i in s..=e {
            if is_symbol(next_line.chars().nth(i).unwrap()) {
                return true;
            }
        }
    }
    false
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
    for n in 0..lines.len() {
        let line = &lines[n];
        for m in r.find_iter(line) {
            if has_symbol(m.start(), m.end(), n, &lines) {
                let n = m.as_str().parse::<u32>().unwrap();
                sum += n;
            }
        }
    }

    println!("Result: {}", sum);
}
