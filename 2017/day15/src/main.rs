use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut lines = input.lines();
        let mut gen_a = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let mut gen_b = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let mut matches = 0;
        for _ in 0..(if part1 { 40_000_000 } else { 5_000_000 }) {
            loop {
                gen_a = (gen_a * 16807) % 2147483647;
                if part1 || gen_a % 4 == 0 {
                    break;
                }
            }
            loop {
                gen_b = (gen_b * 48271) % 2147483647;
                if part1 || gen_b % 8 == 0 {
                    break;
                }
            }

            if gen_a & 0xffff == gen_b & 0xffff {
                matches += 1;
            }
        }

        println!("{}", matches);
    }
}
