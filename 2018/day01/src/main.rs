use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input
        .lines()
        .map(|l| l.strip_prefix('+').unwrap_or(l).parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // part 1
    let sum: i32 = lines.iter().sum();
    println!("{}", sum);

    // part 2
    let mut fs = HashSet::new();
    fs.insert(0);
    let mut f = 0;
    let mut i = 0;
    loop {
        f += lines[i % lines.len()];
        if fs.contains(&f) {
            break;
        }
        fs.insert(f);
        i += 1;
    }
    println!("{}", f);
}
