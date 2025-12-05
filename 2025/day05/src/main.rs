use std::fs;

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

    let mut ids = block2
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a.start().cmp(b.start()).then(b.end().cmp(a.start())));
    ids.sort_unstable();

    // merge ranges (and keep track of range lengths for part 2 already)
    let mut merged_ranges = Vec::new();
    let mut i = 0;
    let mut total2 = 0;
    while i < ranges.len() {
        let mut r = ranges[i].clone();
        i += 1;
        while i < ranges.len() && r.contains(ranges[i].start()) {
            r = *r.start()..=*r.end().max(ranges[i].end());
            i += 1;
        }
        total2 += r.end() - r.start() + 1;
        merged_ranges.push(r);
    }

    // part 1
    let mut ri = merged_ranges.into_iter();
    let mut current_range = ri.next().unwrap();
    let mut current_id = 0;
    let mut total1 = 0;
    while current_id < ids.len() {
        // skip to the next range
        if *current_range.end() < ids[current_id] {
            let Some(cr) = ri.find(|r| *r.end() >= ids[current_id]) else {
                break;
            };
            current_range = cr;
        }

        // skip to the next ID
        while current_id < ids.len() && ids[current_id] < *current_range.start() {
            current_id += 1;
        }

        // count all IDs that lie inside the current range
        while current_id < ids.len() && current_range.contains(&ids[current_id]) {
            total1 += 1;
            current_id += 1;
        }
    }

    println!("{total1}");
    println!("{total2}");
}
