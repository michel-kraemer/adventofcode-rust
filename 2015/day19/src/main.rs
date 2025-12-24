use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let (replacements, input) = input
        .split_once("\n\n")
        .map(|p| (p.0.trim(), p.1.trim()))
        .unwrap();

    let mut replacements = replacements
        .lines()
        .map(|c| c.split_once(" => ").unwrap())
        .collect::<Vec<_>>();

    // part 1 - simple brute force
    let mut all_molecules = Vec::new();
    for r in &replacements {
        for (i, _) in input.match_indices(r.0) {
            all_molecules.push(format!(
                "{}{}{}",
                &input[0..i],
                r.1,
                &input[i + r.0.len()..]
            ));
        }
    }
    all_molecules.sort_unstable(); // this is faster than using a HashSet
    all_molecules.dedup();
    println!("{}", all_molecules.len());

    // part 2 - As it turns out, a greedy approach that always replaces the
    // longest substring first works just fine. As Eric Wastl has confirmed
    // himself in the Day 19 Solution Megathread, the "fewest number of steps"
    // is just a decoy: https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4p1td/
    replacements.sort_by_key(|r| -(r.1.len() as isize));
    let mut str = input.to_string();
    let mut new_str = String::new();
    let mut last = 0;
    let mut total = 0;
    while str != "e" {
        for r in &replacements {
            for (i, _) in str.match_indices(r.1) {
                total += 1;
                new_str.push_str(&str[last..i]);
                new_str.push_str(r.0);
                last = i + r.1.len();
            }
            if !new_str.is_empty() {
                new_str.push_str(&str[last..]);
                (str, new_str) = (new_str, str);
                new_str.clear();
                last = 0;
            }
        }
    }
    println!("{total}");
}
