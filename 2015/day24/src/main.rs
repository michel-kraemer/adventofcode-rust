use std::{collections::VecDeque, fs};

fn find_combinations(
    packages: &Vec<u64>,
    i: usize,
    current: u64,
    target: u64,
    stack: &mut VecDeque<u64>,
    result: &mut Vec<Vec<u64>>,
) {
    let mut i = i;
    while i < packages.len() {
        let v = packages[i];
        if current + v == target {
            let mut r = Vec::with_capacity(stack.len() + 1);
            r.extend(stack.iter());
            r.push(v);
            result.push(r);
        } else if current + v < target {
            stack.push_back(v);
            find_combinations(packages, i + 1, current + v, target, stack, result);
            stack.pop_back();
        }
        i += 1;
    }
}

fn quantum_entanglement(v: &Vec<u64>) -> u64 {
    v.iter().map(|v| *v).reduce(|a, b| a * b).unwrap_or(0)
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut packages = input
            .lines()
            .map(|p| p.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        packages.sort();
        packages.reverse();

        let all_sum = packages.iter().sum::<u64>();
        let target_sum = all_sum / (if part1 { 3 } else { 4 });

        let mut stack1 = VecDeque::new();
        let mut combinations1 = Vec::new();
        find_combinations(&packages, 0, 0, target_sum, &mut stack1, &mut combinations1);

        combinations1.sort_unstable_by(|a, b| {
            a.len()
                .cmp(&b.len())
                .then(quantum_entanglement(&a).cmp(&quantum_entanglement(&b)))
        });

        'outer: for c1 in combinations1 {
            let mut np1 = packages.clone();
            np1.retain(|a| !c1.contains(a));

            let mut stack2 = VecDeque::new();
            let mut combinations2 = Vec::new();
            find_combinations(&np1, 0, 0, target_sum, &mut stack2, &mut combinations2);

            for c2 in combinations2 {
                let mut np2 = np1.clone();
                np2.retain(|a| !c2.contains(a));

                let mut stack3 = VecDeque::new();
                let mut combinations3 = Vec::new();
                find_combinations(&np2, 0, 0, target_sum, &mut stack3, &mut combinations3);

                if part1 {
                    if !combinations3.is_empty() {
                        println!("{}", quantum_entanglement(&c1));
                        break 'outer;
                    }
                } else {
                    for c3 in combinations3 {
                        let mut np3 = np2.clone();
                        np3.retain(|a| !c3.contains(a));

                        let mut stack4 = VecDeque::new();
                        let mut combinations4 = Vec::new();
                        find_combinations(&np3, 0, 0, target_sum, &mut stack4, &mut combinations4);

                        if !combinations4.is_empty() {
                            println!("{}", quantum_entanglement(&c1));
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
}
