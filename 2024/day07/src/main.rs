use std::fs;

fn concat(a: i64, b: i64) -> i64 {
    let concat = format!("{}{}", a, b);
    concat.parse::<i64>().unwrap()
}

fn check(result: i64, cur: i64, numbers: &[i64], part1: bool) -> bool {
    if numbers.len() == 1 {
        let nc = cur + numbers[0];
        if nc == result {
            return true;
        }
        let nc = cur * numbers[0];
        if nc == result {
            return true;
        }
        let nc = concat(cur, numbers[0]);
        if !part1 && nc == result {
            return true;
        }
        false
    } else {
        let nc = cur + numbers[0];
        if check(result, nc, &numbers[1..], part1) {
            return true;
        }
        let nc = cur * numbers[0];
        if check(result, nc, &numbers[1..], part1) {
            return true;
        }
        let nc = concat(cur, numbers[0]);
        if !part1 && check(result, nc, &numbers[1..], part1) {
            return true;
        }
        false
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();

        let mut total = 0;
        for l in lines {
            let (p, s) = l.split_once(": ").unwrap();
            let p = p.parse::<i64>().unwrap();
            let s = s
                .split_whitespace()
                .map(|p| p.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            if check(p, 0, &s, part1) {
                total += p;
            }
        }
        println!("{}", total);
    }
}
