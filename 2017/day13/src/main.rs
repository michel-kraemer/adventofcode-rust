use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let layers = input
            .lines()
            .map(|l| {
                let (layer, len) = l.split_once(": ").unwrap();
                (
                    layer.parse::<usize>().unwrap(),
                    len.parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let mut delay = 0;
        loop {
            let mut severity = 0;
            for l in &layers {
                if (delay + l.0) % (l.1 * 2 - 2) == 0 {
                    if part1 {
                        severity += l.0 * l.1;
                    } else {
                        severity += 1;
                        break;
                    }
                }
            }

            if part1 {
                println!("{}", severity);
                break;
            } else if severity == 0 {
                println!("{}", delay);
                break;
            }

            delay += 1;
        }
    }
}
