use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let s = input.trim().chars().collect::<Vec<_>>();

        let mut sum = 0;
        for i in 0..s.len() {
            let n = if part1 {
                (i + 1) % s.len()
            } else {
                (i + s.len() / 2) % s.len()
            };
            if s[i].is_ascii_digit() && s[i] == s[n] {
                sum += s[i].to_digit(10).unwrap();
            }
        }

        println!("{}", sum);
    }
}
