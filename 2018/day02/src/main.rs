use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let words = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    let mut twos = 0;
    let mut threes = 0;
    for w in &words {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for &c in w {
            *counts.entry(c).or_default() += 1;
        }
        if counts.values().any(|&v| v == 2) {
            twos += 1;
        }
        if counts.values().any(|&v| v == 3) {
            threes += 1;
        }
    }
    println!("{}", twos * threes);

    // part 2
    for i in 0..words.len() {
        for j in i + 1..words.len() {
            let w1 = &words[i];
            let w2 = &words[j];
            if w1.len() != w2.len() {
                continue;
            }

            let mut diffs = 0;
            let mut diff_index = 0;
            for (i, &c) in w1.iter().enumerate() {
                if c != w2[i] {
                    diffs += 1;
                    diff_index = i;
                    if diffs > 1 {
                        break;
                    }
                }
            }

            if diffs == 1 {
                println!(
                    "{}{}",
                    String::from_iter(&w1[..diff_index]),
                    String::from_iter(&w1[diff_index + 1..])
                );
                break;
            }
        }
    }
}
