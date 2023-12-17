use hashbag::HashBag;
use std::{
    cmp::min,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    for part1 in [true, false] {
        let f = File::open("input.txt").expect("Could not open file");
        let reader = BufReader::new(f);

        let lines: Vec<_> = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .split(&[':', '|'])
                    .skip(1)
                    .map(|s| {
                        s.split(" ")
                            .filter(|x| !x.is_empty())
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect::<HashBag<i32>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut sum = if part1 { 0 } else { lines.len() };
        let mut queue: VecDeque<_> = lines.iter().enumerate().collect();
        while !queue.is_empty() {
            let (i, parts) = queue.pop_front().unwrap();

            let numbers_on_card = &parts[0];
            let numbers_i_have = &parts[1];

            let mut matches = 0;
            for c in numbers_on_card {
                if numbers_i_have.contains(&c) > 0 {
                    matches += 1;
                }
            }

            if part1 {
                if matches > 0 {
                    sum += 1 << matches - 1;
                }
            } else {
                let start = i + 1;
                let end = min(lines.len(), start + matches);
                for j in start..end {
                    sum += 1;
                    queue.push_back((j, &lines[j]));
                }
            }
        }

        println!("{sum}");
    }
}
