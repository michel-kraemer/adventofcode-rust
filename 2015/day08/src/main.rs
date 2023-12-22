use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let lines = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut sum = 0;
        for l in lines {
            if part1 {
                // decode
                let mut sl = 0;
                let mut i = 1;
                while i < l.len() - 1 {
                    let c = l[i];
                    if c == '\\' {
                        i += 1;
                        let c2 = l[i];
                        if c2 == '\\' {
                            sl += 1;
                        } else if c2 == '"' {
                            sl += 1;
                        } else if c2 == 'x' {
                            sl += 1;
                            i += 2;
                        }
                    } else {
                        sl += 1;
                    }
                    i += 1;
                }
                sum += l.len() - sl;
            } else {
                // encode
                let mut sl = 2;
                let mut i = 0;
                while i < l.len() {
                    let c = l[i];
                    if c == '"' {
                        sl += 2;
                    } else if c == '\\' {
                        sl += 2;
                    } else {
                        sl += 1;
                    }
                    i += 1;
                }
                sum += sl - l.len();
            }
        }

        println!("{sum}");
    }
}
