use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut bytes = input.trim().bytes();
    let mut len = 0;
    let mut i0 = 0_u64;
    for _ in 0..64 {
        i0 <<= 1;
        if bytes.next().unwrap() == b'^' {
            i0 += 1;
        }
        len += 1;
    }
    let mut i1 = 0_u64;
    for b in bytes {
        i1 <<= 1;
        if b == b'^' {
            i1 += 1;
        }
        len += 1;
    }

    let trailing_zeroes = 64 - (len - 64);
    i1 <<= trailing_zeroes;
    let mask1 = (!0) << trailing_zeroes;

    for part1 in [true, false] {
        let mut current_row = (i0, i1);
        let mut total = 0;

        let rows = if part1 { 40 } else { 400000 };
        for _ in 0..rows {
            total += current_row.0.count_zeros() + current_row.1.count_zeros() - trailing_zeroes;

            // The complex rules from the problem statement are a red herring.
            // We just need to check that the left tile does not equal the right
            // one. Shift 1 bit to the left, shift one bit to the right, and
            // then XOR.
            let l0 = (current_row.0 << 1) | (current_row.1 >> 63);
            let r0 = current_row.0 >> 1;
            let n0 = l0 ^ r0;

            let l1 = current_row.1 << 1;
            let r1 = (current_row.1 >> 1) | ((current_row.0 & 1) << 63);
            let n1 = (l1 ^ r1) & mask1; // mask out trailing zeroes

            current_row = (n0, n1);
        }

        println!("{total}");
    }
}
