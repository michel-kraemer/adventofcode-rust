use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let blocks = input.split("\n\n").collect::<Vec<_>>();

    let steps = blocks[0]
        .split(' ')
        .nth(8)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut instructions = Vec::new();
    for b in &blocks[1..] {
        let mut i = b.lines();
        let in_state = i
            .next()
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .split(' ')
            .next_back()
            .unwrap();
        for _ in 0..2 {
            let f = i
                .next()
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .split(' ')
                .next_back()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let f_write = i
                .next()
                .unwrap()
                .strip_suffix('.')
                .unwrap()
                .split(' ')
                .next_back()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let f_move = i
                .next()
                .unwrap()
                .strip_suffix('.')
                .unwrap()
                .split(' ')
                .next_back()
                .unwrap();
            let f_continue = i
                .next()
                .unwrap()
                .strip_suffix('.')
                .unwrap()
                .split(' ')
                .next_back()
                .unwrap();
            instructions.push((in_state, f, f_write, f_move, f_continue));
        }
    }

    let mut tape = HashMap::new();
    let mut cursor = 0;
    let mut state = "A";

    for _ in 0..steps {
        let current = tape.entry(cursor).or_insert(0);
        for i in &instructions {
            if state == i.0 && *current == i.1 {
                *current = i.2;
                if i.3 == "left" {
                    cursor -= 1;
                } else {
                    cursor += 1;
                }
                state = i.4;
                break;
            }
        }
    }

    let sum: usize = tape.values().sum();
    println!("{}", sum);
}
