use std::{collections::HashMap, fs};

fn line_has_part(
    line: &str,
    start: usize,
    end: usize,
    is_part: impl Fn(char) -> bool,
) -> Option<usize> {
    let s = start.saturating_sub(1);
    for (i, c) in line.chars().skip(s).take(end + 1 - s).enumerate() {
        if !c.is_ascii_digit() && is_part(c) {
            return Some(i + s);
        }
    }
    None
}

fn has_part(
    lines: &[&str],
    i: usize,
    start: usize,
    end: usize,
    is_part: impl Fn(char) -> bool,
) -> Option<(usize, usize)> {
    if i > 0 {
        if let Some(x) = line_has_part(lines[i - 1], start, end, &is_part) {
            return Some((i - 1, x));
        }
    }
    if let Some(x) = line_has_part(lines[i], start, end, &is_part) {
        return Some((i, x));
    }
    if i < lines.len() - 1 {
        if let Some(x) = line_has_part(lines[i + 1], start, end, &is_part) {
            return Some((i + 1, x));
        }
    }
    None
}

fn main() {
    let r = regex::Regex::new(r"\d+").unwrap();

    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();

        let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

        let mut sum = 0;
        for (i, l) in lines.iter().enumerate() {
            let matches = r.find_iter(l);
            for m in matches {
                if let Some(g) = has_part(
                    &lines,
                    i,
                    m.start(),
                    m.end(),
                    if part1 { |c| c != '.' } else { |c| c == '*' },
                ) {
                    let v = m.as_str().parse::<u32>().unwrap();
                    gears.entry(g).or_default().push(v);
                }
            }
        }

        for (_, vs) in gears {
            if part1 {
                sum += vs.iter().sum::<u32>();
            } else if vs.len() == 2 {
                sum += vs[0] * vs[1];
            }
        }

        println!("{}", sum);
    }
}
