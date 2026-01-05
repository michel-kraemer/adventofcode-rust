use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let steps = input.trim().parse::<usize>().unwrap();

    // part 1 - Simulate 2017 rounds and record the position (index) at which
    // each number is inserted. We then find the index after the number 2017.
    // Finally, we iterate backwards through the stored indexes to find the
    // first (i.e. last) number inserted at that index.
    // The approach is based on an idea from maneatingape. See the PR I made in
    // their repository: https://github.com/maneatingape/advent-of-code-rust/pull/20
    let mut i = 0;
    let mut indexes = vec![0; 2017];
    for len in 1..=2017 {
        i = (i + 1 + steps) % len;
        indexes[len - 1] = i;
    }
    let mut next_index = (indexes[2016] + 1) % 2017;

    let mut result1 = 0;
    for (i, &o) in indexes.iter().enumerate().rev() {
        if o == next_index {
            result1 = i + 1;
            break;
        }
        if o < next_index {
            next_index -= 1;
        }
    }
    println!("{result1}");

    // part 2 - Since no element will ever be inserted *before* 0, we're
    // actually looking for the element at index 1. We can simulate the
    // insertion of 50,000,000 elements and record every instance of an element
    // being inserted after 0.
    let mut result2 = 0;
    let mut i = 0;
    let mut len = 1;
    while len < 50_000_000 {
        // jump ahead to just before the end of the buffer
        let add = (len - i) / (steps + 1);
        if add > 0 {
            i += (steps + 1) * add;
            len += add;
        }

        // increment once more to wrap around
        i = (i + steps) % len;
        if i == 0 {
            result2 = len;
        }
        len += 1;
        i += 1;
    }
    println!("{result2}");
}
