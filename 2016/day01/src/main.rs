use std::{collections::HashSet, fs};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let instructions = input
            .split(", ")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty());

        let mut seen = HashSet::new();
        seen.insert((0, 0));

        let mut x = 0;
        let mut y = 0;
        let mut dir = (0i64, 1i64);

        'outer: for i in instructions {
            let d = i[1..].parse::<i64>().unwrap();
            match (dir, &i[0..1]) {
                ((0, 1), "L") => dir = (-1, 0),
                ((1, 0), "L") => dir = (0, 1),
                ((0, -1), "L") => dir = (1, 0),
                ((-1, 0), "L") => dir = (0, -1),

                ((0, 1), "R") => dir = (1, 0),
                ((1, 0), "R") => dir = (0, -1),
                ((0, -1), "R") => dir = (-1, 0),
                ((-1, 0), "R") => dir = (0, 1),

                _ => unreachable!(),
            }

            for _ in 0..d {
                x += dir.0;
                y += dir.1;
                let loc = (x, y);
                if !part1 && seen.contains(&loc) {
                    break 'outer;
                }
                seen.insert(loc);
            }
        }

        println!("{}", (x + y).abs());
    }
}
