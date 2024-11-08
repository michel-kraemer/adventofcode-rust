use std::fs;

fn parse_line(line: &str, part1: bool) -> Vec<u64> {
    if part1 {
        line.split_whitespace()
            .skip(1)
            .map(|t| t.parse().unwrap())
            .collect()
    } else {
        let (_, rhs) = line.split_once(":").unwrap();
        vec![rhs.replace(" ", "").parse().unwrap()]
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let (times, records) = input.trim().split_once("\n").unwrap();

        let times: Vec<u64> = parse_line(times, part1);
        let records: Vec<u64> = parse_line(records, part1);

        let mut product = 1;
        for i in 0..times.len() {
            let time = times[i];
            let record = records[i];
            let mut wins = 0;
            for duration in 0..=time {
                if duration * (time - duration) > record {
                    wins += 1;
                }
            }
            product *= wins;
        }

        println!("{}", product);
    }
}
