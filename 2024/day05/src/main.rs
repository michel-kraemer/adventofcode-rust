use std::{collections::HashMap, fs};

fn followed_by_all(page: u32, other_pages: &[u32], succ: &HashMap<u32, Vec<u32>>) -> bool {
    if let Some(sc) = succ.get(&page) {
        other_pages.iter().all(|x| sc.contains(x))
    } else {
        true
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut succ: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule in rules.lines() {
        let (first, second) = rule.split_once("|").unwrap();
        let first = first.parse::<u32>().unwrap();
        let second = second.parse::<u32>().unwrap();
        succ.entry(first).or_default().push(second);
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for update in updates.lines() {
        let mut pages = update
            .split(",")
            .map(|p| p.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut ok = true;
        for (i, page) in pages.iter().enumerate() {
            if !followed_by_all(*page, &pages[i + 1..], &succ) {
                ok = false;
                break;
            }
        }

        if ok {
            part1 += pages[pages.len() / 2];
        } else {
            let mut last = 0;
            for _ in 0..pages.len() / 2 + 1 {
                for (i, p) in pages.iter().enumerate() {
                    let mut remaining_pages = pages.clone();
                    remaining_pages.swap_remove(i);
                    if followed_by_all(*p, &remaining_pages, &succ) {
                        last = *p;
                        pages = remaining_pages;
                        break;
                    }
                }
            }
            part2 += last;
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
