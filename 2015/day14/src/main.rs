use std::fs;

use regex::Regex;

fn main() {
    let r = Regex::new(
        r"([a-zA-Z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let reindeer = input
            .lines()
            .map(|l| {
                let c = r.captures(l).unwrap();
                (
                    c.get(1).unwrap().as_str(),
                    c[2].parse::<usize>().unwrap(),
                    c[3].parse::<usize>().unwrap(),
                    c[4].parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let mut distances = vec![0; reindeer.len()];
        let mut scores = vec![0; reindeer.len()];

        let s = 2503;
        for i in 0..s {
            for (j, r) in reindeer.iter().enumerate() {
                let f = r.2 + r.3;
                let running = (i % f) < r.2;
                if running {
                    distances[j] += r.1;
                }
            }
            let max = distances.iter().max().unwrap();
            for (j, _) in reindeer.iter().enumerate() {
                if distances[j] == *max {
                    scores[j] += 1;
                }
            }
        }

        if part1 {
            let lead = distances.iter().enumerate().max_by_key(|e| *e.1).unwrap();
            println!("{}", lead.1);
        } else {
            println!("{}", scores.iter().max().unwrap());
        }
    }
}
