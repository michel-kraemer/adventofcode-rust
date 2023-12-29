use std::fs;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let block_size = 10000;

    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let input = input.trim();

        let mut result = vec![' '; 8];
        let mut i = 0;
        let mut ci = 0;
        while ci < 8 {
            let block = (i..i + block_size)
                .into_par_iter()
                .map(|i| {
                    let pw = format!("{}{}", input, i);
                    let digest = md5::compute(pw);
                    format!("{:x}", digest)
                })
                .filter(|h| h.starts_with("00000"))
                .collect::<Vec<_>>();
            i += block_size;

            for hex in block {
                let c = hex.as_bytes()[5];
                if part1 {
                    result[ci] = c as char;
                    ci += 1;
                } else if c.is_ascii_digit() {
                    let pos = c - b'0';
                    if pos < 8 && result[pos as usize] == ' ' {
                        result[pos as usize] = hex.as_bytes()[6] as char;
                        ci += 1;
                    }
                }
            }
        }

        println!("{}", String::from_iter(result));
    }
}
