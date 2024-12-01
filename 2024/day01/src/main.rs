use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut m = HashMap::new();
    for line in lines {
        let mut s = line.split_whitespace();
        left.push(s.next().unwrap().parse::<i64>().unwrap());
        let r = s.next().unwrap().parse::<i64>().unwrap();
        right.push(r);
        *m.entry(r).or_insert(0i64) += 1;
    }
    left.sort();
    right.sort();

    // part 1
    let mut total1 = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        total1 += (r - l).abs();
    }
    println!("{}", total1);

    // part 2
    let mut total2 = 0;
    for l in left.iter() {
        total2 += l * m.get(l).copied().unwrap_or_default();
    }
    println!("{}", total2);
}
