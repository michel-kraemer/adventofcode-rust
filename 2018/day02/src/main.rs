use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let words = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    let mut twos = 0;
    let mut threes = 0;
    for w in &words {
        let mut counts = [0; 26];
        for &c in w {
            counts[(c - b'a') as usize] += 1;
        }
        if counts.contains(&2) {
            twos += 1;
        }
        if counts.contains(&3) {
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
                    w1[..diff_index]
                        .iter()
                        .map(|&b| b as char)
                        .collect::<String>(),
                    w1[diff_index + 1..]
                        .iter()
                        .map(|&b| b as char)
                        .collect::<String>()
                );
                break;
            }
        }
    }
}
