use std::fs;

/// Count the number of ways to fit exactly `remaining` liters of eggnogs into
/// the containers from index `i` on, while `used` containers have already been
/// used. Also returns the minimum number of containers necessary. Can be
/// told to only use at most `max_used` containers.
fn dfs(
    containers: &[usize],
    i: usize,
    used: u32,
    remaining: usize,
    max_used: u32,
    remaining_sums: &[usize],
    cache: &mut Vec<Vec<Vec<(u32, u32)>>>,
) -> (u32, u32) {
    if remaining == 0 {
        return (1, used);
    }
    if used == max_used || i == containers.len() || remaining > remaining_sums[i] {
        return (0, u32::MAX);
    }

    let c = cache[used as usize][i][remaining];
    if c.0 != u32::MAX {
        return c;
    }

    let mut result = (0, u32::MAX);
    for j in i..containers.len() {
        if remaining >= containers[j] {
            let r = dfs(
                containers,
                j + 1,
                used + 1,
                remaining - containers[j],
                max_used,
                remaining_sums,
                cache,
            );
            result.0 += r.0;
            result.1 = result.1.min(r.1);
        }
    }

    cache[used as usize][i][remaining] = result;

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut containers = input
        .lines()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // sorting is beneficial for performance and necessary to calculate prefix
    // sums
    containers.sort_unstable_by_key(|&c| -(c as isize));

    // calculate prefix sums, so we can exit the DFS early
    let mut remaining_sums = vec![0; containers.len()];
    let mut sum = 0;
    for (i, c) in containers.iter().enumerate().rev() {
        sum += c;
        remaining_sums[i] = sum;
    }

    // part 1 - count the total number of ways and retrieve the minimum number
    // of containers needed
    let mut cache =
        vec![vec![vec![(u32::MAX, u32::MAX); 150 + 1]; containers.len() + 1]; containers.len() + 1];
    let total1 = dfs(
        &containers,
        0,
        0,
        150,
        u32::MAX,
        &remaining_sums,
        &mut cache,
    );

    // reset cache
    for c in cache.iter_mut().take(total1.1 as usize + 1) {
        for v in c.iter_mut() {
            v.fill((u32::MAX, u32::MAX));
        }
    }

    // part 2 - count the total number of ways again, but use at most total1.1
    // containers
    let total2 = dfs(
        &containers,
        0,
        0,
        150,
        total1.1,
        &remaining_sums,
        &mut cache,
    );

    println!("{}", total1.0);
    println!("{}", total2.0);
}
