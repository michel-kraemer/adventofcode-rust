use std::{cmp::max, collections::VecDeque, fs};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut ranges = input
            .lines()
            .map(|l| {
                let p = l.split_once('-').unwrap();
                p.0.parse::<usize>().unwrap()..=p.1.parse::<usize>().unwrap()
            })
            .collect::<Vec<_>>();

        ranges.sort_by(|a, b| a.start().cmp(b.start()));

        let mut ranges = ranges.into_iter().collect::<VecDeque<_>>();

        let mut len = 0;
        while !ranges.is_empty() {
            let r = ranges.pop_front().unwrap();
            if ranges.is_empty() || *ranges[0].start() > r.end() + 1 {
                if part1 {
                    ranges.push_front(r);
                    break;
                }
                len += r.end() - r.start() + 1;
            } else if *ranges[0].start() <= r.end() + 1 {
                let n = ranges.pop_front().unwrap();
                ranges.push_front(*r.start()..=max(*r.end(), *n.end()));
            }
        }

        if part1 {
            println!("{}", ranges[0].end() + 1);
        } else {
            println!("{}", u32::MAX - len as u32 + 1);
        }
    }
}
