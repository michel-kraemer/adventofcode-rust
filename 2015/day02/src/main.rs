use std::{cmp::min, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let presents = input
        .lines()
        .map(|line| {
            let vs = line
                .split("x")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (vs[0], vs[1], vs[2])
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut ribbon = 0;
    for p in presents {
        let s1 = p.0 * p.1;
        let s2 = p.1 * p.2;
        let s3 = p.2 * p.0;
        let m = min(s1, min(s2, s3));
        sum += 2 * s1;
        sum += 2 * s2;
        sum += 2 * s3;
        sum += m;

        let cubic = p.0 * p.1 * p.2;
        let mut v = vec![p.0, p.1, p.2];
        v.sort();
        let perimeter = 2 * v[0] + 2 * v[1];

        ribbon += perimeter + cubic;
    }

    println!("{}", sum);
    println!("{}", ribbon);
}
