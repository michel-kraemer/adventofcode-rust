use std::fs;

fn hash(s: &str) -> usize {
    let mut result = 0;
    for c in s.as_bytes() {
        result += *c as usize;
        result *= 17;
        result %= 256;
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.trim().split(",").collect::<Vec<_>>();

    let mut part1_total = 0;
    let mut boxes = [const { Vec::<(&str, usize)>::new() }; 256];
    for l in lines {
        part1_total += hash(l);

        if l.ends_with("-") {
            let label = &l[0..l.len() - 1];
            let h = hash(label);
            boxes[h].retain(|e| e.0 != label);
        } else {
            let (label, value) = l.split_once('=').unwrap();
            let value = value.parse().unwrap();
            let h = hash(label);
            if let Some(i) = boxes[h].iter().position(|e| e.0 == label) {
                boxes[h][i].1 = value;
            } else {
                boxes[h].push((label, value));
            }
        }
    }

    let mut part2_total = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, s) in b.iter().enumerate() {
            part2_total += (i + 1) * (j + 1) * s.1;
        }
    }

    println!("{}", part1_total);
    println!("{}", part2_total);
}
