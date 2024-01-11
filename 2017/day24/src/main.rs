use std::fs;

fn dfs(
    bridges: &mut Vec<(usize, usize, bool)>,
    e: usize,
    strength: usize,
    length: usize,
    max_strength: &mut usize,
    longest: &mut usize,
    longest_max: &mut usize,
) {
    if strength > *max_strength {
        *max_strength = strength;
    }

    if length > *longest {
        *longest = length;
        *longest_max = strength;
    } else if length == *longest && strength > *longest_max {
        *longest_max = strength;
    }

    for i in 0..bridges.len() {
        if !bridges[i].2 && (e == bridges[i].0 || e == bridges[i].1) {
            bridges[i].2 = true;
            dfs(
                bridges,
                if e == bridges[i].0 {
                    bridges[i].1
                } else {
                    bridges[i].0
                },
                strength + bridges[i].0 + bridges[i].1,
                length + 1,
                max_strength,
                longest,
                longest_max,
            );
            bridges[i].2 = false;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut bridges = input
        .lines()
        .map(|l| {
            let p = l.split_once('/').unwrap();
            (
                p.0.parse::<usize>().unwrap(),
                p.1.parse::<usize>().unwrap(),
                false,
            )
        })
        .collect::<Vec<_>>();

    let mut max_strength = 0;
    let mut longest = 0;
    let mut longest_max = 0;

    dfs(
        &mut bridges,
        0,
        0,
        0,
        &mut max_strength,
        &mut longest,
        &mut longest_max,
    );

    // part 1
    println!("{}", max_strength);

    // part 2
    println!("{}", longest_max);
}
