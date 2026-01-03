use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let instructions = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for part1 in [true, false] {
        let mut instructions = instructions.clone();

        let mut steps = 0;
        let mut pointer = 0i32;
        while pointer >= 0 && (pointer as usize) < instructions.len() {
            steps += 1;
            let d = instructions[pointer as usize];
            if !part1 && d >= 3 {
                instructions[pointer as usize] -= 1;
            } else {
                instructions[pointer as usize] += 1;
            }
            pointer += d;
        }

        println!("{steps}");
    }
}
