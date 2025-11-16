use std::{collections::VecDeque, fs};

fn collapse<F>(polymer: &[u8], result: &mut VecDeque<u8>, f: F)
where
    F: Fn(&&u8) -> bool,
{
    for b in polymer.iter().filter(f) {
        if !result.is_empty()
            && b.eq_ignore_ascii_case(result.back().unwrap())
            && (b.is_ascii_lowercase() != result.back().unwrap().is_ascii_lowercase())
        {
            result.pop_back();
        } else {
            result.push_back(*b);
        }
    }
}

fn main() {
    let polymer = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .trim()
        .bytes()
        .collect::<Vec<_>>();

    let mut result = VecDeque::with_capacity(polymer.len());

    // part 1
    collapse(&polymer, &mut result, |_| true);
    println!("{}", result.len());

    // part 2
    let mut min = usize::MAX;
    for u in b'a'..=b'z' {
        result.clear();
        collapse(&polymer, &mut result, |c| !c.eq_ignore_ascii_case(&u));
        min = min.min(result.len());
    }
    println!("{}", min);
}
