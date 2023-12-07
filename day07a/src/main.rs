use std::{fs, collections::HashMap};

fn get_type(s: &str) -> i8 {
    let mut counts: HashMap<char, usize> = HashMap::new();
    s.chars().take(5).for_each(|item| *counts.entry(item).or_default() += 1);
    if counts.len() == 1 {
        7
    } else if counts.len() == 2 {
        if counts.values().any(|&x| x == 4) {
            6
        } else {
            5
        }
    } else if counts.len() == 3 {
        if counts.values().any(|&x| x == 3) {
            4
        } else {
            3
        }
    } else if counts.len() == 4 {
        2
    } else {
        1
    }
}

fn get_ord(s: &str) -> String {
    s.chars().take(5).map(|c| {
        match c {
            'A' => 'E',
            'K' => 'D',
            'Q' => 'C',
            'J' => 'B',
            'T' => 'A',
            '2'..='9' => c,
            _ => panic!("Invalid card")
        }
    }).collect::<String>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut lines = input.lines().collect::<Vec<_>>();

    lines.sort_unstable_by(|a, b| {
        let a_type = get_type(a);
        let b_type = get_type(b);
        if a_type != b_type {
            return a_type.cmp(&b_type);
        }
        let a_ord = get_ord(a);
        let b_ord = get_ord(b);
        a_ord.cmp(&b_ord)
    });

    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        let bid = line.split(" ").last().unwrap().parse::<u32>().unwrap();
        sum += bid * (i + 1) as u32;
    }

    println!("{}", sum);
}
