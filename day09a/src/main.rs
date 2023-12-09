use std::fs;

fn recurse(numbers: &[i32]) -> i32 {
    let nn = numbers.windows(2).map(|p| p[1] - p[0]).collect::<Vec<i32>>();
    if !nn.iter().all(|n| *n == 0) {
        *nn.last().unwrap() + recurse(&nn)
    } else {
        0
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines();

    let mut sum = 0;
    for l in lines {
        let parts = l.split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        sum += *parts.last().unwrap() + recurse(&parts);
    }

    println!("{}", sum);
}
