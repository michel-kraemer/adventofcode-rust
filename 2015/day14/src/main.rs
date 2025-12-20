use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let reindeer = input
            .lines()
            .map(|l| {
                let mut parts = l.split_ascii_whitespace();
                let name = parts.next().unwrap();
                let speed = parts.nth(2).unwrap().parse::<usize>().unwrap();
                let seconds = parts.nth(2).unwrap().parse::<usize>().unwrap();
                let rest = parts.nth(6).unwrap().parse::<usize>().unwrap();
                (name, speed, seconds, rest)
            })
            .collect::<Vec<_>>();

        let mut distances = vec![0; reindeer.len()];
        let mut scores = vec![0; reindeer.len()];

        for i in 0..2503 {
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
