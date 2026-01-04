use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let steps = input.trim().parse::<usize>().unwrap();

    // part 1 - Instead of inserting elements into a Vec, which would have a
    // worst case complexity of O(n²), we use a simple data structure that
    // resembles a b+tree with only one internal node. A maximum leaf size of
    // 128 seems to be a sweet spot.
    const MAX_LEN: usize = 128;
    let mut buffers = vec![vec![0]];
    let mut len = 1;

    let mut i = 0;
    for s in 1..=2017 {
        // compute the index at which to insert the next element
        i = (i + steps) % len + 1;

        // find the leaf that covers this index
        let mut j = 0;
        for (k, b) in buffers.iter_mut().enumerate() {
            if i <= j + b.len() {
                // insert the element into the leaf and update the tree's total
                // length
                b.insert(i - j, s);
                len += 1;

                // if the leaf has become too large, split it into two
                if b.len() > MAX_LEN {
                    let new_b = b.split_off(b.len() / 2);
                    buffers.insert(k + 1, new_b);
                }

                break;
            }
            j += b.len();
        }
    }

    // `i` now points to the element we just inserted. Increase `i` and find the
    // element at this index.
    i = (i + 1) % len;
    let mut j = 0;
    for b in buffers {
        if i <= j + b.len() {
            // we found the leaf that covers i, return the element
            println!("{}", b[i - j]);
            break;
        }
        j += b.len();
    }

    // part 2 - Since no element will ever be inserted *before* 0, we're
    // actually looking for the element at index 1. We can simulate the
    // insertion of 50,000,000 elements and record every instance of an element
    // being inserted after 0.
    let mut result = 0;
    let mut i = 0;
    let mut len = 1;
    while len < 50_000_000 {
        // jump ahead to just before the end of the buffer
        let add = (len - i - 1) / steps;
        if add > 0 {
            i += (steps + 1) * add;
            len += add;
        }

        // increment once more to wrap around
        i = (i + steps) % len;
        if i == 0 {
            result = len;
        }
        len += 1;
        i += 1;
    }
    println!("{result}");
}
