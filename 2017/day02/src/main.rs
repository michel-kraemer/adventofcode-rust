use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut sum = 0;
        for l in input.lines() {
            let p = l
                .split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if part1 {
                let max = p.iter().max().unwrap();
                let min = p.iter().min().unwrap();
                sum += max - min;
            } else {
                for i in 0..p.len() {
                    for j in i + 1..p.len() {
                        let (min, max) = if p[i] > p[j] {
                            (p[j], p[i])
                        } else {
                            (p[i], p[j])
                        };
                        if (max / min) as f64 == (max as f64 / min as f64) {
                            sum += max / min;
                        }
                    }
                }
            }
        }

        println!("{}", sum);
    }
}
