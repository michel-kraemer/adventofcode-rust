use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let elfs = input.trim().split("\n\n").collect::<Vec<_>>();

    let mut all = elfs
        .iter()
        .map(|e| e.lines().map(|l| l.parse::<u64>().unwrap()).sum())
        .collect::<Vec<_>>();
    all.sort();
    all.reverse();

    // part 1
    println!("{}", all[0]);

    // part 2
    println!("{}", all[0..3].iter().sum::<u64>());
}
