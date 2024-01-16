use std::fs;

fn collapse(polymer: &mut Vec<u8>) {
    loop {
        let mut found = false;
        for i in 0..polymer.len() - 1 {
            if (polymer[i].to_ascii_lowercase() == polymer[i + 1].to_ascii_lowercase())
                && (polymer[i].is_ascii_lowercase() != polymer[i + 1].is_ascii_lowercase())
            {
                polymer.remove(i);
                polymer.remove(i);
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
    }
}

fn main() {
    let polymer = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .trim()
        .bytes()
        .collect::<Vec<_>>();

    // part 1
    let mut part1_polymer = polymer.clone();
    collapse(&mut part1_polymer);
    println!("{}", part1_polymer.len());

    // part 2
    let mut min = usize::MAX;
    for u in b'a'..=b'z' {
        let mut part2_polymer = polymer
            .iter()
            .filter(|&c| c.to_ascii_lowercase() != u.to_ascii_lowercase())
            .copied()
            .collect::<Vec<_>>();

        collapse(&mut part2_polymer);

        if part2_polymer.len() < min {
            min = part2_polymer.len();
        }
    }
    println!("{}", min);
}
