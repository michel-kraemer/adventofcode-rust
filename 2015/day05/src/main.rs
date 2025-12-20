use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut nice = 0;
        for l in input.lines() {
            if part1 {
                let mut vowels = 0;
                let mut last_letter = ' ';
                let mut contains_repeated = false;
                let mut contains_bad = false;
                for c in l.chars() {
                    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                        vowels += 1;
                    }
                    if c == last_letter {
                        contains_repeated = true;
                    }
                    if (last_letter == 'a' && c == 'b')
                        || (last_letter == 'c' && c == 'd')
                        || (last_letter == 'p' && c == 'q')
                        || (last_letter == 'x' && c == 'y')
                    {
                        contains_bad = true;
                    }
                    last_letter = c;
                }

                if vowels >= 3 && contains_repeated && !contains_bad {
                    nice += 1;
                }
            } else {
                let mut repeat_pair = false;
                for i in 0..l.len() - 2 {
                    let c1 = &l[i..i + 2];
                    if l[i + 2..].contains(c1) {
                        repeat_pair = true;
                        break;
                    }
                }

                let mut repeat_char = false;
                for i in 0..l.len() - 2 {
                    if l.as_bytes()[i] == l.as_bytes()[i + 2] {
                        repeat_char = true;
                        break;
                    }
                }

                if repeat_pair && repeat_char {
                    nice += 1;
                }
            }
        }

        println!("{nice}");
    }
}
