use std::fs;

/// Calculate `a.pow(b) % m` using binary exponentiation
fn pow_mod(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut result = 1;
    while b > 0 {
        if b & 1 != 0 {
            result *= a;
            result %= m;
        }
        a *= a;
        a %= m;
        b >>= 1;
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut parts = input.split_ascii_whitespace();
    let row = parts.nth(15).unwrap();
    let row = row[..row.len() - 1].parse::<u64>().unwrap();
    let col = parts.nth(1).unwrap();
    let col = col[..col.len() - 1].parse::<u64>().unwrap();

    // determine the sequential (0-based) index of the table entry using
    // triangular numbers
    let dist = col - 1;
    let n = row + dist - 1;
    let i = n * (n + 1) / 2 + dist;

    let exp = pow_mod(252533, i, 33554393);
    println!("{}", (20151125 * exp) % 33554393);
}
