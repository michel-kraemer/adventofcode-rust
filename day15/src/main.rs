use std::fs;

fn h(s: &str) -> usize {
    let mut hash = 0usize;
    for c in s.chars() {
        hash += c as usize;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let values = input.trim().split(",");

        let mut sum = 0;
        if part1 {
            sum = values.map(|v| h(v)).sum();
        } else {
            let mut boxes = vec![Vec::new(); 256];

            for v in values {
                if v.contains("=") {
                    let (key, value) = v.split_once("=").unwrap();
                    let b = h(key);
                    let p = boxes[b].iter().position(|(k, _)| *k == key);
                    if let Some(p) = p {
                        boxes[b][p] = (key, value);
                    } else {
                        boxes[b].push((key, value));
                    }
                } else if v.ends_with("-") {
                    let (key, _) = v.split_once("-").unwrap();
                    let b = h(key);
                    boxes[b].iter().position(|(k, _)| *k == key)
                        .map(|i| boxes[b].remove(i));
                }
            }

            for (bi, b) in boxes.iter().enumerate() {
                for (si, (_, value)) in b.iter().enumerate() {
                    sum += (bi + 1) * (si + 1) * value.parse::<usize>().unwrap();
                }
            }
        }

        println!("{}", sum);
    }
}
