use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{fs, thread};

struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    target_joltage: Vec<usize>,
}

/// Part 1: Simple DFS with memoization
fn dfs_part1(
    lights: &mut [bool],
    target_lights: &[bool],
    i: usize,
    buttons: &[Vec<usize>],
    cache: &mut HashMap<(Vec<bool>, usize), i64>,
) -> i64 {
    if lights == target_lights {
        return 0;
    }
    if i == buttons.len() {
        return -1;
    }

    let key = (lights.to_vec(), i);
    if let Some(c) = cache.get(&key) {
        return *c;
    }

    let mut result = i64::MAX;
    for j in i..buttons.len() {
        for &k in &buttons[j] {
            lights[k] = !lights[k];
        }
        let r = 1 + dfs_part1(lights, target_lights, j + 1, buttons, cache);
        if r > 0 {
            result = result.min(r);
        }
        for &k in &buttons[j] {
            lights[k] = !lights[k];
        }
    }

    cache.insert(key, result);

    result
}

/// Initialize a slice of length `m` with the contents `[0, 0, ..., n]` and then
/// repeatedly call this function to obtain all possible combinations of `m`
/// integers that sum to `n`. The function returns `false` if there is no other
/// combination.
///
/// Example (m=3, n=4):
/// ```text
/// [0, 0, 4]
/// [0, 1, 3]
/// [0, 2, 2]
/// [0, 3, 1]
/// [0, 4, 0]
/// [1, 0, 3]
/// [1, 1, 2]
/// [1, 2, 1]
/// [1, 3, 0]
/// [2, 0, 2]
/// [2, 1, 1]
/// [2, 2, 0]
/// [3, 0, 1]
/// [3, 1, 0]
/// [4, 0, 0]
/// ```
fn next_combination(combinations: &mut [usize]) -> bool {
    let i = combinations.iter().rposition(|&v| v != 0).unwrap();
    if i == 0 {
        return false;
    }
    let v = combinations[i];
    combinations[i - 1] += 1;
    combinations[i] = 0;
    combinations[combinations.len() - 1] = v - 1;
    true
}

fn is_button_available(i: usize, mask: u32) -> bool {
    mask & (1 << i) > 0
}

/// Part 2: Optimized DFS that tries to prune as many branches as possible
fn dfs_part2(joltage: &[usize], available_buttons_mask: u32, buttons: &[Vec<usize>]) -> usize {
    if joltage.iter().all(|j| *j == 0) {
        return 0;
    }

    // Important optimization: Find the joltage value with the lowest number of
    // combinations of buttons to try. This allows us to prune branches as early
    // as possible.
    // Second optimization (not so important, but still quite good): If multiple
    // joltage values are affected by the same number of buttons, select the
    // highest value
    let (mini, &min) = joltage
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v > 0)
        .min_by_key(|&(i, &v)| {
            (
                // lowest number of buttons
                buttons
                    .iter()
                    .enumerate()
                    .filter(|&(j, b)| {
                        is_button_available(j, available_buttons_mask) && b.contains(&i)
                    })
                    .count(),
                // highest joltage value (negative because we're using `min_by_key`)
                -(v as isize),
            )
        })
        .unwrap();

    // get the buttons that affect the joltage value at position `mini`
    let matching_buttons = buttons
        .iter()
        .enumerate()
        .filter(|&(i, b)| is_button_available(i, available_buttons_mask) && b.contains(&mini))
        .collect::<Vec<_>>();

    let mut result = usize::MAX;

    if !matching_buttons.is_empty() {
        // create new mask so only those buttons remain that do not affect the
        // joltage value at position `mini`
        let mut new_mask = available_buttons_mask;
        for (i, _) in &matching_buttons {
            new_mask &= !(1 << i);
        }

        // try all combinations of matching buttons
        let mut new_joltage = joltage.to_vec();
        let mut counts = vec![0; matching_buttons.len()];
        counts[matching_buttons.len() - 1] = min;
        loop {
            // count down joltage values and make sure we don't press a button
            // too often (i.e. that the number of button presses is not higher
            // than any of the values to decrease)
            let mut good = true;
            new_joltage.copy_from_slice(joltage);
            'buttons: for (bi, &cnt) in counts.iter().enumerate() {
                if cnt == 0 {
                    continue;
                }
                for &j in matching_buttons[bi].1 {
                    if new_joltage[j] >= cnt {
                        new_joltage[j] -= cnt;
                    } else {
                        good = false;
                        break 'buttons;
                    }
                }
            }

            if good {
                // recurse with decreased joltage values and with remaining buttons
                let r = dfs_part2(&new_joltage, new_mask, buttons);
                if r != usize::MAX {
                    result = result.min(min + r);
                }
            }

            // try next combination
            if !next_combination(&mut counts) {
                break;
            }
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // parse input
    let machines = input
        .lines()
        .map(|l| {
            let parts = l.split(" ").collect::<Vec<_>>();
            let target_lights = parts[0].as_bytes()[1..parts[0].len() - 1]
                .iter()
                .map(|b| *b == b'#')
                .collect::<Vec<_>>();
            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let target_joltage = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Machine {
                target_lights,
                buttons,
                target_joltage,
            }
        })
        .collect::<Vec<_>>();

    // part 1 - simple DFS with memoization
    let mut total1 = 0;
    for m in &machines {
        let mut lights = vec![false; m.target_lights.len()];
        total1 += dfs_part1(
            &mut lights,
            &m.target_lights,
            0,
            &m.buttons,
            &mut HashMap::new(),
        );
    }
    println!("{total1}");

    // part 2 - optimized DFS that tries to prune as many branches as possible
    let n_threads = thread::available_parallelism().unwrap().into();
    let machines = Arc::new(machines);
    let index = Arc::new(AtomicUsize::new(0));
    let threads = (0..n_threads)
        .map(|_| {
            let machines = Arc::clone(&machines);
            let index = Arc::clone(&index);
            thread::spawn(move || {
                let mut result = 0;
                loop {
                    let i = index.fetch_add(1, Ordering::Relaxed);
                    if i >= machines.len() {
                        break;
                    }
                    let m = &machines[i];
                    result += dfs_part2(&m.target_joltage, (1 << m.buttons.len()) - 1, &m.buttons);
                }
                result
            })
        })
        .collect::<Vec<_>>();

    let total2 = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .sum::<usize>();

    println!("{total2}");
}

#[cfg(test)]
mod tests {
    use crate::next_combination;

    #[test]
    fn test_next_combination() {
        let max = 4;
        let mut comb = vec![0, 0, 0, max];
        for a in 0..=max {
            for b in 0..=max - a {
                for c in 0..=max - a - b {
                    let d = max - a - b - c;
                    assert_eq!(vec![a, b, c, d], comb);
                    if a == 4 {
                        assert!(!next_combination(&mut comb));
                    } else {
                        assert!(next_combination(&mut comb));
                    }
                }
            }
        }
    }
}
