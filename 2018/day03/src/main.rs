use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let claims = input
        .lines()
        .map(|l| {
            let (_, rest) = l.split_once(" @ ").unwrap();
            let (coords, size) = rest.split_once(": ").unwrap();
            let (x, y) = coords.split_once(',').unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            let (w, h) = size.split_once('x').unwrap();
            let w = w.parse::<usize>().unwrap();
            let h = h.parse::<usize>().unwrap();
            (x, y, w, h)
        })
        .enumerate()
        .collect::<Vec<_>>();

    let mut fabric: HashMap<(usize, usize), usize> = HashMap::new();

    // mark used squares
    for (_, c) in &claims {
        for x in c.0..(c.0 + c.2) {
            for y in c.1..(c.1 + c.3) {
                *fabric.entry((x, y)).or_default() += 1;
            }
        }
    }

    // part 1: count how many squares are used more than once
    let sum = fabric.values().filter(|&&c| c > 1).count();
    println!("{}", sum);

    // part 2: find the ID of the one claim that never overlaps
    for (i, c) in &claims {
        let mut only_ones = true;
        for x in c.0..(c.0 + c.2) {
            for y in c.1..(c.1 + c.3) {
                if *fabric.get(&(x, y)).unwrap() != 1 {
                    only_ones = false;
                    break;
                }
            }
        }
        if only_ones {
            println!("{}", i + 1);
            break;
        }
    }
}
