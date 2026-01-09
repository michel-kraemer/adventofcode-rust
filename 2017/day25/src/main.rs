use std::fs;

const WORD_LEN: usize = 12;

#[derive(Clone, Copy)]
struct Instruction {
    write: bool,
    move_right: bool,
    next_state: usize,
}

#[derive(Clone, Copy)]
struct CompressedInstruction {
    write: u64,
    move_right: bool,
    next_state: usize,
    steps: usize,
}

/// Performs tape compression. Takes a small `tape` of fixed `len` and simulates
/// the Turing machine when starting in the given `state` and at position
/// `cursor`. Keeps on simulating until the cursor leaves the small tape to the
/// left or to the right. Returns a compressed instruction with the state of the
/// small tape when the cursor has left it, whether the cursor left it to the
/// left or to the right, and how many steps the Turing machine performed.
///
/// See also <https://en.wikipedia.org/wiki/Linear_speedup_theorem>
fn compress(
    mut state: usize,
    mut cursor: usize,
    tape: u64,
    len: usize,
    instructions: &[Instruction],
) -> CompressedInstruction {
    let mut steps = 0;
    let mut new_tape = tape;
    let mut move_right = false;

    loop {
        let current = new_tape & (1 << cursor) > 0;
        let i = instructions[state * 2 + (current as usize)];
        if i.write {
            new_tape |= 1 << cursor;
        } else {
            new_tape &= !(1 << cursor);
        }
        state = i.next_state;
        steps += 1;
        if i.move_right {
            cursor += 1;
            if cursor == len {
                move_right = true;
                break;
            }
        } else if cursor == 0 {
            break;
        } else {
            cursor -= 1;
        }
    }

    CompressedInstruction {
        write: new_tape,
        move_right,
        next_state: state,
        steps,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let blocks = input.split("\n\n").collect::<Vec<_>>();

    let steps = blocks[0]
        .split(' ')
        .nth(8)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    // parse instructions
    let mut instructions = vec![
        const {
            Instruction {
                write: false,
                move_right: false,
                next_state: 0,
            }
        };
        (blocks.len() - 1) * 2
    ];

    let n_states = blocks.len() - 1;
    for (state, b) in blocks.iter().skip(1).enumerate() {
        let mut i = b.lines().skip(1);
        for j in 0..2 {
            i.next().unwrap();
            let f_write = i
                .next()
                .unwrap()
                .strip_suffix('.')
                .unwrap()
                .rsplit_once(' ')
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            let f_move = i
                .next()
                .unwrap()
                .strip_suffix('.')
                .unwrap()
                .rsplit_once(' ')
                .unwrap()
                .1;
            let f_continue = i
                .next()
                .unwrap()
                .strip_suffix('.')
                .unwrap()
                .rsplit_once(' ')
                .unwrap()
                .1;
            instructions[state * 2 + j] = Instruction {
                write: f_write == 1,
                move_right: f_move == "right",
                next_state: (f_continue.as_bytes()[0] - b'A') as usize,
            };
        }
    }

    // a cache for compressed instructions
    let mut compressed_instructions_from_left = vec![None; (1 << WORD_LEN) * n_states];
    let mut compressed_instructions_from_right = vec![None; (1 << WORD_LEN) * n_states];

    // Simulate Turing machine on a compressed tape until we cannot take any
    // more steps. Running the machine with compressed instructions is basically
    // the same as running it normally, with the difference that we not only
    // write 0's and 1's but whole words of length `WORD_LEN` (so our alphabet
    // is larger), and also, one step on the compressed tape corresponds to
    // several steps on the normal one. This significantly speeds up the whole
    // simulation.
    let mut compressed_left = Vec::with_capacity(2048);
    let mut compressed_right = Vec::with_capacity(2048);
    let mut current = 0;
    let mut from_left = true;
    let mut state = 0;
    let mut steps_taken = 0;

    while steps_taken < steps {
        // get compressed instruction
        let i = if from_left {
            let o = &mut compressed_instructions_from_left[current as usize * n_states + state];
            if o.is_none() {
                // compress original instructions on demand
                *o = Some(compress(state, 0, current, WORD_LEN, &instructions));
            }
            o.unwrap()
        } else {
            let o = &mut compressed_instructions_from_right[current as usize * n_states + state];
            if o.is_none() {
                // compress original instructions on demand
                *o = Some(compress(
                    state,
                    WORD_LEN - 1,
                    current,
                    WORD_LEN,
                    &instructions,
                ));
            }
            o.unwrap()
        };

        // check if we are able to execute this compressed instruction without
        // exceeding `steps`
        if steps_taken + i.steps > steps {
            break;
        }

        // execute compressed instruction
        if i.move_right {
            compressed_left.push(i.write);
            current = compressed_right.pop().unwrap_or_default();
            from_left = true;
        } else {
            compressed_right.push(i.write);
            current = compressed_left.pop().unwrap_or_default();
            from_left = false;
        }

        state = i.next_state;
        steps_taken += i.steps;
    }

    // convert compressed tape to normal tape
    let mut left = Vec::with_capacity(2048);
    let mut right = Vec::with_capacity(2048);
    for t in compressed_left.into_iter() {
        for i in 0..WORD_LEN {
            left.push(t & (1 << i) > 0);
        }
    }
    for t in compressed_right.into_iter() {
        // whenever we push to the right stack, we need to iterate in reverse
        for i in (0..WORD_LEN).rev() {
            right.push(t & (1 << i) > 0);
        }
    }
    let mut current = if from_left {
        // whenever we push to the right stack, we need to iterate in reverse
        for i in (1..WORD_LEN).rev() {
            right.push(current & (1 << i) > 0);
        }
        current & 1 > 0 // first bit
    } else {
        for i in 0..WORD_LEN - 1 {
            left.push(current & (1 << i) > 0);
        }
        current & (1 << (WORD_LEN - 1)) > 0 // last bit
    };

    // perform rest of simulation normally
    while steps_taken < steps {
        let i = instructions[state * 2 + (current as usize)];
        if i.move_right {
            left.push(i.write);
            current = right.pop().unwrap_or_default();
        } else {
            right.push(i.write);
            current = left.pop().unwrap_or_default();
        }
        state = i.next_state;
        steps_taken += 1;
    }

    // compute checksum
    let sum = left.into_iter().map(|v| v as usize).sum::<usize>()
        + current as usize
        + right.into_iter().map(|v| v as usize).sum::<usize>();
    println!("{sum}");
}
