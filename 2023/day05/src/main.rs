use std::{fs, ops::Range};

fn parse_mapping(b: &str) -> Vec<(Range<usize>, Range<usize>)> {
    b.trim()
        .lines()
        .skip(1)
        .map(|line| {
            let mut p = line.split(" ");
            let dest_start = p.next().unwrap().parse().unwrap();
            let src_start = p.next().unwrap().parse().unwrap();
            let len = p.next().unwrap().parse::<usize>().unwrap();
            (dest_start..dest_start + len, src_start..src_start + len)
        })
        .collect()
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut blocks = input.split("\n\n");

        let mut ranges = Vec::new();
        let mut seeds = blocks.next().unwrap().split(" ").skip(1);
        if part1 {
            for s in seeds {
                let s = s.parse::<usize>().unwrap();
                ranges.push(s..s + 1);
            }
        } else {
            while let Some(start) = seeds.next() {
                let start = start.parse::<usize>().unwrap();
                let len = seeds.next().unwrap().parse::<usize>().unwrap();
                ranges.push(start..start + len);
            }
        }

        let maps = blocks.map(parse_mapping).collect::<Vec<_>>();

        for m in &maps {
            let mut new_ranges = Vec::new();
            while !ranges.is_empty() {
                let r = ranges.swap_remove(0);
                let mut found = false;
                for mapping in m {
                    if mapping.1.start < r.end && r.start < mapping.1.end {
                        // add intersection to new_ranges
                        // see https://scicomp.stackexchange.com/a/26260
                        let os = r.start.max(mapping.1.start) - mapping.1.start + mapping.0.start;
                        let oe = r.end.min(mapping.1.end) - mapping.1.start + mapping.0.start;
                        new_ranges.push(os..oe);

                        // queue remainder of this range
                        if r.start < mapping.1.start {
                            ranges.push(r.start..mapping.1.start);
                        }
                        if r.end > mapping.1.end {
                            ranges.push(mapping.1.end..r.end);
                        }

                        found = true;
                        break;
                    }
                }
                if !found {
                    // no mapping was found - keep the range as is
                    new_ranges.push(r);
                }
            }
            ranges = new_ranges;
        }

        let result = ranges.into_iter().map(|r| r.start).min().unwrap();

        println!("{}", result);
    }
}
