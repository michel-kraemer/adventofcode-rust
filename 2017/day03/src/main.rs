use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input.trim().parse::<usize>().unwrap();

    // part 1
    for i in (1..usize::MAX).step_by(2) {
        if i * i >= input {
            let s = (i - 2) * (i - 2) + 1;
            let d = ((input - s) % (i - 1)).abs_diff(i / 2 - 1);
            println!("{}", d + i / 2);
            break;
        }
    }

    // part 2
    let mut grid = HashMap::new();

    let mut x = 0i32;
    let mut y = 0i32;
    grid.insert((x, y), 1);
    x += 1;
    grid.insert((x, y), 1);
    y -= 1;
    let mut dir_x = 0i32;
    let mut dir_y = -1i32;

    for s in 3usize.. {
        let mut sum = 0;
        for ny in y - 1..=y + 1 {
            for nx in x - 1..=x + 1 {
                sum += grid.get(&(nx, ny)).unwrap_or(&0);
            }
        }
        grid.insert((x, y), sum);

        if sum > input {
            println!("{}", sum);
            break;
        }

        for i in (1..).step_by(2) {
            if i * i >= s {
                let t = (i - 2) * (i - 2) + 1;
                let d = ((s - t) % (i - 1)).abs_diff(i / 2 - 1);
                if s != i * i && (s == t || d == i / 2) {
                    let t = dir_x;
                    dir_x = dir_y;
                    dir_y = -t;
                }
                break;
            }
        }

        x += dir_x;
        y += dir_y;
    }
}
