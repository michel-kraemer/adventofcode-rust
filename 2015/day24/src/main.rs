use std::fs;

fn main() {
    // After solving this puzzle differently (see previous Git commit), I
    // noticed that the shortest possible combination to fill the first group
    // with the lowest possible numbers is always the answer. The other groups
    // can be ignored. Since the packages are already sorted by weight in the
    // input file, we don't need to sort them again. We then perform a BFS.
    // Starting with the lightest one, we add one package after the other until
    // we reach the target sum. By definition, as soon as we reach this sum, we
    // have found the combination with the least packages and the lowest quantum
    // entanglement.

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let packages = input
        .lines()
        .map(|p| p.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let sum = packages.iter().sum::<usize>();

    for part1 in [true, false] {
        let target_sum = sum / if part1 { 3 } else { 4 };

        let mut combinations = Vec::new();
        for (i, &p) in packages.iter().enumerate() {
            combinations.push((p, p, i));
        }
        'outer: loop {
            let mut new_combinations = Vec::new();
            for (current_sum, current_qe, last_i) in combinations {
                for (i, &p) in packages.iter().enumerate().skip(last_i + 1) {
                    let new_sum = current_sum + p;
                    let new_qe = current_qe * p;
                    if new_sum == target_sum {
                        println!("{new_qe}");
                        break 'outer;
                    } else if new_sum <= target_sum {
                        new_combinations.push((new_sum, new_qe, i));
                    }
                }
            }
            combinations = new_combinations;
        }
    }
}
