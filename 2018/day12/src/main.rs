use std::fs;

fn step(
    state: &[u64],
    pos: i64,
    len: usize,
    rules: &[bool; 32],
    new_state: &mut Vec<u64>,
) -> (i64, usize) {
    new_state.clear();
    new_state.push(0);

    let mut index_last = 0;
    let mut j = 4;

    let mut i = state[0].trailing_zeros() as usize - 4;
    let pos = pos + i as i64 - 2;
    while i < len {
        let d = i / 64;
        let m = i % 64;
        let mut w = ((state[d] >> m) & 0b11111) as usize;
        if m >= 60 && d + 1 < state.len() {
            w |= (state[d + 1] as usize & ((1 << (m - 59)) - 1)) << (64 - m);
        }

        if j % 64 == 0 {
            new_state.push(0);
            index_last += 1;
        }
        if rules[w] {
            new_state[index_last] |= 1 << j;
        }

        j += 1;
        i += 1;
    }

    (
        pos,
        index_last * 64 + (64 - new_state[index_last].leading_zeros() as usize),
    )
}

fn sum(state: &[u64], pos: i64, len: usize) -> i64 {
    let mut result = 0;
    for i in state[0].trailing_zeros() as usize..len {
        if state[i / 64] >> (i % 64) & 1 > 0 {
            result += i as i64 + pos;
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut lines = input.lines();
    let initial_state = lines.next().unwrap();
    let initial_state = &initial_state[15..];

    // Parse initial state into a bit vector. Leave four bits empty at the
    // beginning, so extracting bits in `step()` is easier.
    let mut state: Vec<u64> = vec![0];
    let mut index_last = 0;
    for (i, b) in initial_state.bytes().enumerate() {
        if (i + 4) % 64 == 0 {
            state.push(0);
            index_last += 1;
        }
        if b == b'#' {
            state[index_last] |= 1 << ((i + 4) % 64);
        }
    }

    // `pos` represents the ID of the pot at the beginning of the bit vector
    let mut pos = -4;

    // `len` is the length of the bit vector. Truncate it at the last set bit.
    let mut len = index_last * 64 + (64 - state[index_last].leading_zeros() as usize);

    // parse rules into a table with all possible 2^5 bit patterns
    lines.next();
    let mut rules = [false; 32];
    for l in lines {
        let bytes = l.as_bytes();
        let from = &bytes[0..5];
        let to = bytes[9];
        if to == b'#' {
            let mut p = 0_usize;
            for (i, &b) in from.iter().enumerate() {
                if b == b'#' {
                    p += 1 << i;
                }
            }
            rules[p] = true;
        }
    }

    // part 1 - simulate the first 20 steps
    let mut prev_state = Vec::new();
    let mut steps = 0_i64;
    while steps < 20 {
        (state, prev_state) = (prev_state, state);
        (pos, len) = step(&prev_state, pos, len, &rules, &mut state);
        steps += 1;
    }
    println!("{}", sum(&state, pos, len));

    // part 2 - only simulate until the pattern repeats
    let mut prev_pos = 0;
    while steps < 50_000_000_000 {
        (state, prev_state) = (prev_state, state);
        prev_pos = pos;
        (pos, len) = step(&prev_state, prev_pos, len, &rules, &mut state);
        steps += 1;
        if state == prev_state {
            // pattern has repeated
            break;
        }
    }

    // extrapolate to 50 billion steps
    println!(
        "{}",
        sum(
            &state,
            pos + (pos - prev_pos) * (50_000_000_000 - steps),
            len,
        )
    );
}
