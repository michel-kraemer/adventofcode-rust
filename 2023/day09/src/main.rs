use std::fs;

fn extrapolate(n: &[i64], part1: bool) -> i64 {
    let mut children = Vec::new();
    for i in 0..n.len() - 1 {
        let d = n[i + 1] - n[i];
        children.push(d);
    }
    if children.iter().all(|x| *x == 0) {
        return n[0];
    }
    let cd = extrapolate(&children, part1);
    if part1 {
        n[n.len() - 1] + cd
    } else {
        n[0] - cd
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();
        let mut sum = 0;
        for l in lines {
            let n = l
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            sum += extrapolate(&n, part1);
        }
        println!("{}", sum);
    }
}
