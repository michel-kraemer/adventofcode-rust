use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut total1 = 0;
    let mut total2 = 0;
    for l in input.lines() {
        let mut p = l
            .split_ascii_whitespace()
            .map(|i| i.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        // sorting is faster than comparing individual elements against each other
        p.sort_unstable();

        total1 += p.last().unwrap() - p.first().unwrap();

        'outer: for (i, &a) in p.iter().enumerate() {
            for &b in p.iter().skip(i + 1) {
                if b % a == 0 {
                    total2 += b / a;
                    break 'outer;
                }
            }
        }
    }

    println!("{total1}");
    println!("{total2}");
}
