use std::{collections::VecDeque, fs};

fn backtrack(
    containers: &[i32],
    current: &mut VecDeque<i32>,
    max: i32,
    matching_combinations: &mut Vec<VecDeque<i32>>,
) -> i32 {
    let mut matches = 0;
    for i in 0..containers.len() {
        current.push_back(containers[i]);
        let sum: i32 = current.iter().sum();
        if sum == max {
            matching_combinations.push(current.clone());
            matches += 1;
        } else if sum < max {
            matches += backtrack(&containers[i + 1..], current, max, matching_combinations);
        }
        current.pop_back();
    }
    matches
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let containers = input
        .lines()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut current = VecDeque::new();
    let mut matching_combinations = Vec::new();
    let r = backtrack(&containers, &mut current, 150, &mut matching_combinations);

    println!("{}", r);

    let min = matching_combinations.iter().map(|c| c.len()).min().unwrap();
    let total_min = matching_combinations
        .iter()
        .filter(|c| c.len() == min)
        .count();

    println!("{}", total_min);
}
