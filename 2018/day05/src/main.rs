use std::fs;

// If you wonder why this is so complicated, I've had some fun optimizing the
// algorithm :-) Look into the Git history to find a simpler version that
// also works.
fn collapse(polymer: &[u8], marked: &mut [bool]) -> usize {
    let mut i = 0;
    let mut j = 1;
    let mut removed = 0;
    let mut lm = 0;
    while j < polymer.len() {
        if (polymer[i].to_ascii_lowercase() == polymer[j].to_ascii_lowercase())
            && (polymer[i].is_ascii_lowercase() != polymer[j].is_ascii_lowercase())
        {
            removed += 2;
            marked[i] = true;
            marked[j] = true;
            if i == lm {
                i = j + 1;
                j += 2;
                lm = i;
            } else {
                while i > lm {
                    i -= 1;
                    if !marked[i] {
                        break;
                    }
                }
                if i == lm && marked[i] {
                    i = j + 1;
                    j += 2;
                    lm = i;
                } else {
                    j += 1;
                }
            }
        } else {
            i = j;
            j += 1;
        }
    }
    removed
}

fn main() {
    let polymer = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .trim()
        .bytes()
        .collect::<Vec<_>>();

    let mut marked = vec![false; polymer.len()];

    // part 1
    let part1_removed = collapse(&polymer, &mut marked);
    println!("{}", polymer.len() - part1_removed);

    // part 2
    let mut min = usize::MAX;
    for u in b'a'..=b'z' {
        marked.fill(false);
        let part2_polymer = polymer
            .iter()
            .filter(|&c| c.to_ascii_lowercase() != u.to_ascii_lowercase())
            .copied()
            .collect::<Vec<_>>();

        let part2_removed = collapse(&part2_polymer, &mut marked);

        if part2_polymer.len() - part2_removed < min {
            min = part2_polymer.len() - part2_removed;
        }
    }
    println!("{}", min);
}
