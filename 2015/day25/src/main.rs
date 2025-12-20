use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut parts = input.split_ascii_whitespace();
    let target_row = parts.nth(15).unwrap();
    let target_row = target_row[..target_row.len() - 1].parse::<usize>().unwrap();
    let target_col = parts.nth(1).unwrap();
    let target_col = target_col[..target_col.len() - 1].parse::<usize>().unwrap();

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
            println!("{current}");
            break;
        }
    }
}
