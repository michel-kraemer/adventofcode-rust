use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut points = input
        .lines()
        .map(|l| {
            let p = l.trim().split(',').collect::<Vec<_>>();
            (
                p[0].parse::<i64>().unwrap(),
                p[1].parse::<i64>().unwrap(),
                p[2].parse::<i64>().unwrap(),
                p[3].parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut constellations = 0;
    while !points.is_empty() {
        let mut constellation = vec![points.swap_remove(0)];

        let mut i = 0;
        while i < points.len() {
            if constellation.iter().any(|l| {
                (l.0 - points[i].0).abs()
                    + (l.1 - points[i].1).abs()
                    + (l.2 - points[i].2).abs()
                    + (l.3 - points[i].3).abs()
                    <= 3
            }) {
                constellation.push(points[i]);
                points.swap_remove(i);
                i = 0;
            } else {
                i += 1;
            }
        }

        constellations += 1;
    }

    println!("{}", constellations);
}
