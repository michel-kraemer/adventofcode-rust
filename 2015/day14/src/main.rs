use std::fs;

#[inline]
fn dist(i: usize, r: (&str, usize, usize, usize)) -> usize {
    let f = r.2 + r.3;
    r.1 * ((i / f) * r.2 + (i % f).min(r.2))
}

fn main() {
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

    let mut max = 0;
    for i in 1..=2503 {
        max = 0;
        for (r, d) in reindeer.iter().zip(distances.iter_mut()) {
            let v = dist(i, *r);
            *d = v;
            max = max.max(v);
        }
        for (d, s) in distances.iter().zip(scores.iter_mut()) {
            if *d == max {
                *s += 1;
            }
        }
    }

    println!("{max}");
    println!("{}", scores.iter().max().unwrap());
}
