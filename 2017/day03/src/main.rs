use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input.trim().parse::<usize>().unwrap();

    for part1 in [true, false] {
        let mut grid = HashMap::new();
        grid.insert((0, 0), 1);
        let mut w = 1;
        let mut s = 2;
        let mut result = 0;
        'outer: while s <= input {
            let mut x: i32 = w / 2;
            let mut y: i32 = w / 2;
            for dir in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
                for _ in 0..w - 1 {
                    x += dir.0;
                    y += dir.1;
                    if part1 {
                        grid.insert((x, y), s);
                        if s == input {
                            result = x.abs() + y.abs();
                            break 'outer;
                        }
                    } else {
                        let mut sum = 0;
                        for ny in y - 1..=y + 1 {
                            for nx in x - 1..=x + 1 {
                                sum += grid.get(&(nx, ny)).unwrap_or(&0);
                            }
                        }
                        grid.insert((x, y), sum);
                        if sum > input {
                            result = sum as i32;
                            break 'outer;
                        }
                    }
                    s += 1;
                }
            }
            w += 2;
        }

        println!("{}", result);
    }
}
