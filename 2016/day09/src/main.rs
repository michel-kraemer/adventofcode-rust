use std::fs;

fn decompress(s: &[u8]) -> (usize, usize) {
    let mut i = 0;
    let mut result1 = 0;
    let mut result2 = 0;

    while i < s.len() {
        if s[i] == b'(' {
            let mut e = i + 1;

            let mut j = 0;
            while e < s.len() && s[e] != b'x' {
                j *= 10;
                j += (s[e] - b'0') as usize;
                e += 1;
            }
            e += 1;

            let mut r = 0;
            while e < s.len() && s[e] != b')' {
                r *= 10;
                r += (s[e] - b'0') as usize;
                e += 1;
            }
            e += 1;

            i = e + j;
            result1 += j * r;
            result2 += decompress(&s[e..i]).1 * r;
        } else {
            i += 1;
            result1 += 1;
            result2 += 1;
        }
    }

    (result1, result2)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (total1, total2) = decompress(input.trim().as_bytes());
    println!("{total1}");
    println!("{total2}");
}
