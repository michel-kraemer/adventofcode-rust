use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let range = input.trim().split_once('-').unwrap();
        let range = range.0.parse::<usize>().unwrap()..=range.1.parse::<usize>().unwrap();

        let mut total = 0;
        for mut i in range {
            if !(100000..=999999).contains(&i) {
                continue;
            }

            let mut increasing = true;
            let mut last_digit = usize::MAX;
            let mut same = [0; 10];
            let mut any_same = false;
            while i > 0 {
                let digit = i % 10;
                i /= 10;

                // we're parsing from right to left!
                if digit > last_digit {
                    increasing = false;
                    break;
                }

                if digit == last_digit {
                    same[digit] += 1;
                    any_same = true;
                }
                last_digit = digit;
            }

            let ok = increasing
                && match part1 {
                    true => any_same,
                    false => same.into_iter().any(|v| v == 1),
                };
            if ok {
                total += 1;
            }
        }

        println!("{}", total);
    }
}
