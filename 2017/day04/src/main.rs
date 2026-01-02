use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // part 1
    let valid1 = input
        .lines()
        .filter_map(|l| {
            let mut p = l
                .split_ascii_whitespace()
                .map(|w| w.bytes().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let len = p.len();
            p.sort_unstable();
            p.dedup();
            if len == p.len() { Some(p) } else { None }
        })
        .collect::<Vec<_>>();
    println!("{}", valid1.len());

    // part 2
    let total2 = valid1
        .into_iter()
        .filter_map(|mut p| {
            p.iter_mut().for_each(|w| w.sort_unstable());
            let len = p.len();
            p.sort_unstable();
            p.dedup();
            if len == p.len() { Some(true) } else { None }
        })
        .count();
    println!("{total2}");
}
