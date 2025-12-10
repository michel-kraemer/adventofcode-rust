use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::HashMap;
use std::fs;

fn dfs_part1(
    machine: &mut [bool],
    target_machine: &[bool],
    i: usize,
    buttons: &[Vec<u64>],
    cache: &mut HashMap<(Vec<bool>, usize), i64>,
) -> i64 {
    if machine == target_machine {
        return 0;
    }
    if i == buttons.len() {
        return -1;
    }

    if let Some(c) = cache.get(&(machine.to_vec(), i)) {
        return *c;
    }

    let mut result = i64::MAX;
    for j in i..buttons.len() {
        for k in &buttons[j] {
            machine[*k as usize] = !machine[*k as usize];
        }
        let r = 1 + dfs_part1(machine, target_machine, j + 1, buttons, cache);
        if r > 0 {
            result = result.min(r);
        }
        for k in &buttons[j] {
            machine[*k as usize] = !machine[*k as usize];
        }
    }

    cache.insert((machine.to_vec(), i), result);

    result
}

/// don't do this at home
fn combinations(max: u64, c: u64) -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    if c == 1 {
        result.push(vec![max]);
    } else if c == 2 {
        for a in 0..=max {
            result.push(vec![a, max - a]);
        }
    } else if c == 3 {
        for a in 0..=max {
            for b in 0..=max - a {
                let c = max - a - b;
                result.push(vec![a, b, c]);
            }
        }
    } else if c == 4 {
        for a in 0..=max {
            for b in 0..=max - a {
                for c in 0..=max - a - b {
                    let d = max - a - b - c;
                    result.push(vec![a, b, c, d]);
                }
            }
        }
    } else if c == 5 {
        for a in 0..=max {
            for b in 0..=max - a {
                for c in 0..=max - a - b {
                    for d in 0..=max - a - b - c {
                        let e = max - a - b - c - d;
                        result.push(vec![a, b, c, d, e]);
                    }
                }
            }
        }
    } else if c == 6 {
        for a in 0..=max {
            for b in 0..=max - a {
                for c in 0..=max - a - b {
                    for d in 0..=max - a - b - c {
                        for e in 0..=max - a - b - c - d {
                            let f = max - a - b - c - d - e;
                            result.push(vec![a, b, c, d, e, f]);
                        }
                    }
                }
            }
        }
    } else if c == 7 {
        for a in 0..=max {
            for b in 0..=max - a {
                for c in 0..=max - a - b {
                    for d in 0..=max - a - b - c {
                        for e in 0..=max - a - b - c - d {
                            for f in 0..=max - a - b - c - d - e {
                                let g = max - a - b - c - d - e - f;
                                result.push(vec![a, b, c, d, e, f, g]);
                            }
                        }
                    }
                }
            }
        }
    } else if c == 8 {
        for a in 0..=max {
            for b in 0..=max - a {
                for c in 0..=max - a - b {
                    for d in 0..=max - a - b - c {
                        for e in 0..=max - a - b - c - d {
                            for f in 0..=max - a - b - c - d - e {
                                for g in 0..=max - a - b - c - d - e - f {
                                    let h = max - a - b - c - d - e - f - g;
                                    result.push(vec![a, b, c, d, e, f, g, h]);
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        panic!("not enough c {c}");
    }
    result
}

fn dfs_part2(joltage: &[u64], buttons: &[Vec<u64>]) -> u64 {
    if joltage.iter().all(|j| *j == 0) {
        return 0;
    }

    let mut min = usize::MAX;
    let mut mini = 0;
    for (i, &jolt) in joltage.iter().enumerate() {
        if jolt > 0 {
            let n_matching_buttons = buttons.iter().filter(|b| b.contains(&(i as u64))).count();
            if n_matching_buttons < min {
                min = n_matching_buttons;
                mini = i;
            }
        }
    }
    let min = joltage[mini];

    let matching_buttons = buttons
        .iter()
        .filter(|b| b.contains(&(mini as u64)))
        .collect::<Vec<_>>();

    let mut result = u64::MAX;

    if !matching_buttons.is_empty() {
        let remaining_buttons = buttons
            .iter()
            .filter(|b| !b.contains(&(mini as u64)))
            .cloned()
            .collect::<Vec<_>>();

        for c in 1..=matching_buttons.len() {
            let counts = combinations(min, c as u64);
            let mut new_joltage = joltage.to_vec();
            for cnts in &counts {
                let mut good = true;
                new_joltage.copy_from_slice(joltage);
                'buttons: for (bi, cnt) in cnts.iter().enumerate() {
                    for b in matching_buttons[bi] {
                        if new_joltage[*b as usize] >= *cnt {
                            new_joltage[*b as usize] -= cnt;
                        } else {
                            good = false;
                            break 'buttons;
                        }
                    }
                }
                if !good {
                    continue;
                }

                let r = dfs_part2(&new_joltage, &remaining_buttons);
                if r != u64::MAX {
                    result = result.min(min + r);
                }
            }
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // part 1
    let mut total = 0;
    for l in &lines {
        let parts = l.split(" ").collect::<Vec<_>>();
        let target_machine = parts[0];
        let target_machine = target_machine.as_bytes()[1..target_machine.len() - 1]
            .iter()
            .map(|b| *b == b'#')
            .collect::<Vec<_>>();
        let buttons = &parts[1..parts.len() - 1];
        let buttons = buttons
            .iter()
            .map(|b| {
                b[1..b.len() - 1]
                    .split(',')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut machine = vec![false; target_machine.len()];
        total += dfs_part1(
            &mut machine,
            &target_machine,
            0,
            &buttons,
            &mut HashMap::new(),
        );
    }
    println!("{total}");

    // part 2
    let total = lines
        .iter()
        .par_bridge()
        .map(|l| {
            let parts = l.split(" ").collect::<Vec<_>>();
            let buttons = &parts[1..parts.len() - 1];
            let buttons = buttons
                .iter()
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let target_joltage = parts[parts.len() - 1];
            let target_joltage = target_joltage[1..target_joltage.len() - 1]
                .split(',')
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            dfs_part2(&target_joltage, &buttons)
        })
        .sum::<u64>();

    println!("{total}");
}
