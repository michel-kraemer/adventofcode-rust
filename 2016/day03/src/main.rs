use std::fs;

fn is_triangle(a: u64, b: u64, c: u64) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut total1 = 0;
    let mut total2 = 0;

    let mut c1a = 0;
    let mut c1b = 0;

    let mut c2a = 0;
    let mut c2b = 0;

    let mut c3a = 0;
    let mut c3b = 0;

    let mut i = 0;
    for l in input.lines() {
        let mut parts = l
            .split_ascii_whitespace()
            .map(|v| v.parse::<u64>().unwrap());
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        let c = parts.next().unwrap();

        if is_triangle(a, b, c) {
            total1 += 1;
        }

        if i == 0 {
            c1a = a;
            c2a = b;
            c3a = c;
            i += 1;
        } else if i == 1 {
            c1b = a;
            c2b = b;
            c3b = c;
            i += 1;
        } else {
            total2 += if is_triangle(c1a, c1b, a) { 1 } else { 0 };
            total2 += if is_triangle(c2a, c2b, b) { 1 } else { 0 };
            total2 += if is_triangle(c3a, c3b, c) { 1 } else { 0 };
            i = 0;
        }
    }

    println!("{total1}");
    println!("{total2}");
}
