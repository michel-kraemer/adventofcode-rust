use std::fs;

use regex::Regex;

fn main() {
    let re_row = Regex::new(r"row (\d+)").unwrap();
    let re_col = Regex::new(r"column (\d+)").unwrap();

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let target_row = re_row.captures(&input).unwrap()[1]
        .parse::<usize>()
        .unwrap();
    let target_col = re_col.captures(&input).unwrap()[1]
        .parse::<usize>()
        .unwrap();

    let mut x = 1usize;
    let mut y = 1usize;
    let mut current = 20151125usize;

    loop {
        y -= 1;
        x += 1;
        if y == 0 {
            y = x;
            x = 1;
        }
        current *= 252533;
        current %= 33554393;
        if x == target_col && y == target_row {
            println!("{}", current);
            break;
        }
    }
}
