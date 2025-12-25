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

    let mut result = i64::MAX - 1;
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

/// Initializes a list so it can be used with [combination_next]. Distributes
/// the given number `n` to this list from the back to the front while observing
/// the given maxima. For example, given a list `[0, 0, 0, 0]`, the maxima `[1,
/// 2, 1, 3]`, and the number `n = 4`, the result will be `[0, 0, 1, 3]`. The
/// function returns `true` if `n` could be distributed completely or `false`
/// if the sum of the maxima is less than `n`.
fn combinations_distribute(combinations: &mut [usize], maxima: &[usize], mut n: usize) -> bool {
    assert!(!combinations.is_empty());
    let mut i = combinations.len() - 1;
    while n > 0 {
        let d = n.min(maxima[i] - combinations[i]);
        n -= d;
        combinations[i] += d;
        if i == 0 {
            break;
        }
        i -= 1;
    }
    n == 0
}

/// Initialize a slice of length `m` with [combinations_distribute] and then
/// repeatedly call this function to obtain all possible combinations of `m`
/// integers that sum to `n`, while observing the given maxima. The function
/// returns `false` if there is no other combination.
///
/// Example (m=3, n=4, maxima=[3,2,4]):
/// ```text
/// [0, 0, 4]
/// [0, 1, 3]
/// [0, 2, 2]
/// [1, 0, 3]
/// [1, 1, 2]
/// [1, 2, 1]
/// [2, 0, 2]
/// [2, 1, 1]
/// [2, 2, 0]
/// [3, 0, 1]
/// [3, 1, 0]
/// ```
fn combinations_next(combinations: &mut [usize], maxima: &[usize]) -> bool {
    let mut i = combinations.iter().rposition(|&v| v != 0).unwrap();
    let mut to_distribute = 0;
    loop {
        if i == 0 {
            return false;
        }
        to_distribute += combinations[i] - 1;
        combinations[i - 1] += 1;
        combinations[i] = 0;
        i -= 1;
        if combinations[i] <= maxima[i] {
            break;
        }
    }
    combinations_distribute(combinations, maxima, to_distribute)
}

/// Count how many buttons affect the joltage value at position `i`
fn count_affected_buttons(i: usize, buttons: &[Vec<usize>], available_buttons_mask: u32) -> usize {
    let mut result = 0;
    let mut km = available_buttons_mask;
    while km > 0 {
        // select LSB and reset it
        let k = km.trailing_zeros() as usize;
        km &= km - 1;

        if buttons[k].contains(&i) {
            result += 1;
        }
    }
    result
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
                count_affected_buttons(i, buttons, available_buttons_mask),
                // highest joltage value (negative because we're using `min_by_key`)
                -(v as isize),
            )
        })
        .unwrap();

    // get the buttons that affect the joltage value at position `mini`
    let mut matching_buttons = Vec::new();
    let mut km = available_buttons_mask;
    while km > 0 {
        // select LSB and reset it
        let k = km.trailing_zeros() as usize;
        km &= km - 1;

        if buttons[k].contains(&mini) {
            matching_buttons.push((k, &buttons[k]));
        }
    }

    // optimization: determine how many times we can press each button at most
    // so that the values it affects are not exceeded
    let mut maxima = vec![0; matching_buttons.len()];
    for (i, &(_, b)) in matching_buttons.iter().enumerate() {
        let mut min = usize::MAX;
        for &j in b {
            min = min.min(joltage[j]);
        }
        maxima[i] = min;
    }

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
        if !combinations_distribute(&mut counts, &maxima, min) {
            return result;
        }

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
            if !combinations_next(&mut counts, &maxima) {
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
    use super::*;

    #[test]
    fn test_combinations1() {
        let max = 4;
        let mut comb = vec![0; 4];
        let maxima = [2, 1, 2, 4];
        assert!(combinations_distribute(&mut comb, &maxima, max));
        for a in 0..=max {
            for b in 0..=max - a {
                for c in 0..=max - a - b {
                    let d = max - a - b - c;
                    let expected = vec![a, b, c, d];
                    if expected.iter().enumerate().any(|(i, e)| *e > maxima[i]) {
                        continue;
                    }
                    assert_eq!(expected, comb);
                    if comb == vec![2, 1, 1, 0] {
                        assert!(!combinations_next(&mut comb, &maxima));
                        return;
                    } else {
                        assert!(combinations_next(&mut comb, &maxima));
                    }
                }
            }
        }
    }

    #[test]
    fn test_combinations2() {
        let max = 4;
        let mut comb = vec![0; 4];
        let maxima = [2, 1, 2, 1];
        assert!(combinations_distribute(&mut comb, &maxima, max));
        for a in 0..=max {
            for b in 0..=max - a {
                for c in 0..=max - a - b {
                    let d = max - a - b - c;
                    let expected = vec![a, b, c, d];
                    if expected.iter().enumerate().any(|(i, e)| *e > maxima[i]) {
                        continue;
                    }
                    assert_eq!(expected, comb);
                    if comb == vec![2, 1, 1, 0] {
                        assert!(!combinations_next(&mut comb, &maxima));
                        return;
                    } else {
                        assert!(combinations_next(&mut comb, &maxima));
                    }
                }
            }
        }
    }

    #[test]
    fn test_combinations_full() {
        let mut comb = vec![0; 4];
        let maxima = vec![0, 1, 2, 1];
        assert!(combinations_distribute(&mut comb, &maxima, 4));
        assert_eq!(maxima, comb);
        assert!(!combinations_next(&mut comb, &maxima));
    }

    #[test]
    fn test_combinations_distribute_too_much() {
        let mut comb = vec![0; 4];
        let maxima = vec![0, 0, 2, 1];
        assert!(!combinations_distribute(&mut comb, &maxima, 4));
    }
}
