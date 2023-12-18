use std::fs;

fn recurse1(numbers: &[i32]) -> i32 {
    let nn = numbers.windows(2).map(|p| p[1] - p[0]).collect::<Vec<_>>();
    if !nn.iter().all(|n| *n == 0) {
        *nn.last().unwrap() + recurse1(&nn)
    } else {
        0
    }
}

fn recurse2(numbers: &[i32]) -> i32 {
    let nn = numbers.windows(2).map(|p| p[1] - p[0]).collect::<Vec<_>>();
    if !nn.iter().all(|n| *n == 0) {
        *nn.first().unwrap() - recurse2(&nn)
    } else {
        0
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines = input.lines();

        let mut sum = 0;
        for l in lines {
            let parts: Vec<i32> = l.split(" ").map(|s| s.parse().unwrap()).collect();
            if part1 {
                sum += *parts.last().unwrap() + recurse1(&parts);
            } else {
                sum += *parts.first().unwrap() - recurse2(&parts);
            }
        }

        println!("{}", sum);
    }
}
