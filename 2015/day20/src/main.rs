use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let min_presents = input.trim().parse::<usize>().unwrap();

    // Part 1: Pre-compute divisor sums up to 1 million. This limit was enough
    // for my input. If it does not work for you, just increase it until it
    // works.
    const LIMIT: usize = 1_000_000;
    let mut divisors = vec![10; LIMIT];

    for i in 2..LIMIT {
        for j in (i..LIMIT).step_by(i) {
            divisors[j] += i * 10;
        }
    }
    println!(
        "{}",
        divisors.iter().position(|d| *d >= min_presents).unwrap()
    );

    // Part 2: Similar to part 1, but multiply by 11 and only take the first 50
    // steps
    divisors.fill(11);
    for i in 2..LIMIT {
        for j in (i..LIMIT).step_by(i).take(50) {
            divisors[j] += i * 11;
        }
    }
    println!(
        "{}",
        divisors.iter().position(|d| *d >= min_presents).unwrap()
    );
}
