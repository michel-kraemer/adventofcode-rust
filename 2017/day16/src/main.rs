use std::fs;

/// Multiply `a` with `b` and store the result in `dst`
fn mul(a: &[usize; 16], b: &[usize; 16], dst: &mut [usize; 16]) {
    for (d, &s) in b.iter().enumerate() {
        dst[d] = a[s];
    }
}

/// Binary exponentiation of `src`. Store the result in `dst`
fn pow(src: &[usize; 16], mut e: usize, dst: &mut [usize; 16]) {
    assert!(e > 0);
    e -= 1;
    let mut a = *src;
    let mut tmp = *src;
    dst.copy_from_slice(src);
    while e > 0 {
        if e & 1 > 0 {
            mul(dst, &a, &mut tmp);
            dst.copy_from_slice(&tmp);
        }
        mul(&a, &a, &mut tmp);
        a.copy_from_slice(&tmp);
        e >>= 1;
    }
}

fn to_order(moves: &[usize; 16], renames: &[usize; 16]) -> String {
    let mut a: [usize; 16] = [0; 16];
    for (i, m) in moves.iter().enumerate() {
        a[*m] = i;
    }

    let mut b: [char; 16] = [' '; 16];
    for (i, r) in renames.iter().enumerate() {
        let j = a.iter().position(|p| *p == i).unwrap();
        b[j] = ((*r as u8) + b'a') as char;
    }

    b.iter().collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // Perform instructions once but differentiate between moves and renames.
    // This allows us to use binary exponentiation later to get the order of the
    // programs after 1 billion dances.
    let mut programs: [usize; 16] = std::array::from_fn(|i| i);
    let mut swaps: [usize; 16] = std::array::from_fn(|i| i);

    let mut offset = 0;
    for instruction in input.trim().split(',') {
        if let Some(m) = instruction.strip_prefix('s') {
            offset = (offset + m.parse::<usize>().unwrap()) % 16;
        } else if let Some(m) = instruction.strip_prefix('x') {
            let (a, b) = m.split_once('/').unwrap();
            programs.swap(
                (a.parse::<usize>().unwrap() + 16 - offset) % 16,
                (b.parse::<usize>().unwrap() + 16 - offset) % 16,
            );
        } else if let Some(m) = instruction.strip_prefix('p') {
            let mut c = m.bytes();
            let a = c.next().unwrap();
            let b = c.nth(1).unwrap();
            swaps.swap((a - b'a') as usize, (b - b'a') as usize);
        } else {
            panic!("Unknown instruction: {instruction}");
        }
    }

    programs.rotate_right(offset);

    let mut moves = [0; 16];
    for (i, p) in programs.iter().enumerate() {
        moves[*p] = i;
    }
    let mut renames = [0; 16];
    for (i, s) in swaps.iter().enumerate() {
        renames[*s] = i;
    }

    // part 1
    println!("{}", to_order(&moves, &renames));

    // part 2
    let mut moves2 = [0; 16];
    pow(&moves, 1_000_000_000, &mut moves2);
    let mut renames2 = [0; 16];
    pow(&renames, 1_000_000_000, &mut renames2);

    println!("{}", to_order(&moves2, &renames2));
}
