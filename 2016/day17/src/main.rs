use std::{collections::VecDeque, fs};

use md5::Context;

fn is_open(b: u8) -> bool {
    !b.is_ascii_digit() && b != b'a'
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let passcode = input.trim();

    for part1 in [true, false] {
        let mut queue = VecDeque::new();
        let mut initial_context = Context::new();
        initial_context.consume(passcode);
        queue.push_back((0, 0, initial_context, "".to_string()));

        let mut max = 0;
        while let Some((x, y, context, path)) = queue.pop_front() {
            if x == 3 && y == 3 {
                if part1 {
                    println!("{path}");
                    break;
                } else {
                    max = max.max(path.len());
                    continue;
                }
            }

            let hash = format!("{:x}", context.clone().compute());
            let up = is_open(hash.as_bytes()[0]);
            let down = is_open(hash.as_bytes()[1]);
            let left = is_open(hash.as_bytes()[2]);
            let right = is_open(hash.as_bytes()[3]);

            if up && y > 0 {
                let mut new_path = path.clone();
                new_path.push('U');
                let mut new_context = context.clone();
                new_context.consume(b"U");
                queue.push_back((x, y - 1, new_context, new_path));
            }
            if down && y < 3 {
                let mut new_path = path.clone();
                new_path.push('D');
                let mut new_context = context.clone();
                new_context.consume(b"D");
                queue.push_back((x, y + 1, new_context, new_path));
            }
            if left && x > 0 {
                let mut new_path = path.clone();
                new_path.push('L');
                let mut new_context = context.clone();
                new_context.consume(b"L");
                queue.push_back((x - 1, y, new_context, new_path));
            }
            if right && x < 3 {
                let mut new_path = path.clone();
                new_path.push('R');
                let mut new_context = context.clone();
                new_context.consume(b"R");
                queue.push_back((x + 1, y, new_context, new_path));
            }
        }

        if !part1 {
            println!("{max}");
        }
    }
}
