use std::{collections::VecDeque, fs};

fn collapse<'a, I>(polymer: I, result: &mut VecDeque<u8>)
where
    I: Iterator<Item = &'a u8>,
{
    for b in polymer {
        if let Some(a) = result.back()
            && b.abs_diff(*a) == b'a' - b'A'
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
    collapse(polymer.iter(), &mut result);
    println!("{}", result.len());

    // Performance optimization: For part 2, we can start with the result of
    // part 1. It's not necessary to do the same reductions again.
    let polymer = Vec::from_iter(result.iter().copied());

    // part 2
    let mut min = usize::MAX;
    for u in b'a'..=b'z' {
        result.clear();
        collapse(
            polymer.iter().filter(|b| !b.eq_ignore_ascii_case(&u)),
            &mut result,
        );
        min = min.min(result.len());
    }
    println!("{min}");
}
