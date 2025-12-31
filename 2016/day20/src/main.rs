use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut ranges = input
        .lines()
        .map(|l| {
            let p = l.split_once('-').unwrap();
            p.0.parse::<usize>().unwrap()..=p.1.parse::<usize>().unwrap()
        })
        .collect::<Vec<_>>();
    ranges.sort_unstable_by(|a, b| b.start().cmp(a.start()));

    let mut total1 = usize::MAX;
    let mut len = 0;
    while let Some(r) = ranges.pop() {
        if ranges.is_empty() || *ranges.last().unwrap().start() > r.end() + 1 {
            if total1 == usize::MAX {
                total1 = r.end() + 1;
            }
            len += r.end() - r.start() + 1;
        } else {
            let next_range = ranges.last_mut().unwrap();
            *next_range = *r.start()..=*r.end().max(next_range.end());
        }
    }

    println!("{total1}");
    println!("{}", u32::MAX - len as u32 + 1);
}
