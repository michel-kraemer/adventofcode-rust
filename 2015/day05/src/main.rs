use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut nice = 0;
        for l in input.lines() {
            if part1 {
                let mut vowels = 0;
                let mut last_letter = b' ';
                let mut contains_repeated = false;
                let mut contains_bad = false;
                for &c in l.as_bytes() {
                    if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' {
                        vowels += 1;
                    }
                    if c == last_letter {
                        contains_repeated = true;
                    }
                    if (last_letter == b'a' && c == b'b')
                        || (last_letter == b'c' && c == b'd')
                        || (last_letter == b'p' && c == b'q')
                        || (last_letter == b'x' && c == b'y')
                    {
                        contains_bad = true;
                        break;
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
