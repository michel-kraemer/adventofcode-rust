use std::{cmp::Ordering, fs};

fn main() {
    // parse input
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (block1, block2) = input.split_once("\n\n").unwrap();

    let mut ranges = block1
        .lines()
        .map(|l| {
            let (start, end) = l.split_once("-").unwrap();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    let ids = block2
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a.start().cmp(b.start()).then(b.end().cmp(a.start())));

    // merge ranges
    let mut merged_ranges = Vec::new();
    let mut i = 0;
    while i < ranges.len() {
        let mut r = ranges[i].clone();
        i += 1;
        while i < ranges.len() && r.contains(ranges[i].start()) {
            r = *r.start()..=*r.end().max(ranges[i].end());
            i += 1;
        }
        merged_ranges.push(r);
    }

    // part 1
    println!(
        "{}",
        ids.into_iter()
            .filter_map(|id| {
                merged_ranges
                    .binary_search_by(|r| {
                        if *r.start() > id {
                            Ordering::Greater
                        } else if *r.end() < id {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    })
                    .ok()
            })
            .count()
    );

    // part 2
    println!(
        "{}",
        merged_ranges
            .into_iter()
            .map(|r| r.end() - r.start() + 1)
            .sum::<i64>()
    );
}
