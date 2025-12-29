use std::{collections::VecDeque, fs};

const W: usize = 50;

fn is_space(x: i32, y: i32, num: i32) -> bool {
    let r = (x * x + 3 * x + 2 * x * y + y + y * y) + num;
    r.count_ones().is_multiple_of(2)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let num = input.trim().parse::<i32>().unwrap();

    let mut seen = vec![false; W * W];

    let mut queue = VecDeque::new();
    queue.push_back((1, 1, 0));
    seen[W + 1] = true;

    let mut total1 = 0;
    let mut total2 = 0;
    while let Some((x, y, steps)) = queue.pop_front() {
        if steps <= 50 {
            total2 += 1;
        }
        if x == 31 && y == 39 {
            total1 = steps;
            break;
        }

        for d in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x + d.0;
            let ny = y + d.1;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < W
                && (ny as usize) < W
                && !seen[ny as usize * W + nx as usize]
                && is_space(nx, ny, num)
            {
                seen[ny as usize * W + nx as usize] = true;
                queue.push_back((nx, ny, steps + 1));
            }
        }
    }

    println!("{total1}");
    println!("{total2}");
}
