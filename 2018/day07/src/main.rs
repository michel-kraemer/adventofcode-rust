use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut all_steps = HashSet::new();
        let instructions = input
            .lines()
            .map(|l| {
                let p = l.split(' ').collect::<Vec<_>>();
                let p1 = p[1].chars().next().unwrap();
                let p7 = p[7].chars().next().unwrap();
                all_steps.insert(p1);
                all_steps.insert(p7);
                (p1, p7)
            })
            .collect::<Vec<_>>();

        let num_steps = all_steps.len();

        // initialize dependencies for all known steps
        let mut deps = all_steps
            .into_iter()
            .map(|a| (a, Vec::new()))
            .collect::<HashMap<_, _>>();

        // set dependencies for all steps
        for i in &instructions {
            deps.entry(i.1).or_default().push(i.0);
        }

        // convert dependencies to vector and sort alphabetically
        let mut deps = deps.into_iter().collect::<Vec<_>>();
        deps.sort_by_key(|d| d.0);

        let mut result = String::new();
        let mut workers: Vec<Option<(char, usize)>> = vec![None; if part1 { 1 } else { 4 }];
        let mut time = 0;
        loop {
            // check which steps are finished
            for w in workers.iter_mut() {
                if let Some((step, remaining)) = w {
                    *remaining -= 1;
                    if *remaining == 0 {
                        result.push(*step);
                        *w = None;
                    }
                }
            }

            // for every idle worker, find a step to work on
            for w in workers.iter_mut().filter(|w| w.is_none()) {
                for i in 0..deps.len() {
                    let d = &deps[i];
                    if d.1.iter().all(|d| result.contains(*d)) {
                        // we found a new item to work on
                        *w = Some((d.0, (d.0 as u8 - b'A') as usize + 1 + 60));
                        deps.remove(i);
                        break;
                    }
                }
            }

            if result.len() == num_steps {
                // all items are finished
                break;
            }

            time += 1;
        }

        if part1 {
            println!("{}", result);
        } else {
            println!("{}", time);
        }
    }
}
