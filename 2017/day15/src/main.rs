use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut lines = input.lines();
    let mut gen_a = lines
        .next()
        .unwrap()
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let mut gen_b = lines
        .next()
        .unwrap()
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();

    // solve part 1 and part 2 simultaneously
    let mut steps1 = 0;
    let mut matches1 = 0;
    let mut steps2 = 0;
    let mut matches2 = 0;
    let mut q_a = VecDeque::new();
    let mut q_b = VecDeque::new();
    while steps1 < 40_000_000 && steps2 < 5_000_000 {
        gen_a = (gen_a * 16807) % 2147483647;
        gen_b = (gen_b * 48271) % 2147483647;

        if steps1 < 40_000_000 && gen_a & 0xffff == gen_b & 0xffff {
            matches1 += 1;
        }
        steps1 += 1;

        if steps2 < 5_000_000 {
            if gen_a % 4 == 0 {
                q_a.push_back(gen_a);
            }
            if gen_b % 8 == 0 {
                q_b.push_back(gen_b);
            }
            if !q_a.is_empty() && !q_b.is_empty() {
                let next_a = q_a.pop_front().unwrap();
                let next_b = q_b.pop_front().unwrap();
                if next_a & 0xffff == next_b & 0xffff {
                    matches2 += 1;
                }
                steps2 += 1;
            }
        }
    }

    println!("{matches1}");
    println!("{matches2}");
}
