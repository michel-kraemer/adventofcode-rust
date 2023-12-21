use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut floor = 0i64;
        let mut steps = 0;

        for c in input.chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!()
            }

            steps += 1;
            if !part1 && floor == -1 {
                break
            }
        }

        if part1 {
            println!("{}", floor);
        } else {
            println!("{}", steps);
        }
    }
}
