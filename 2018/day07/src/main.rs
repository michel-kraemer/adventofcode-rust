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
        let mut finished = HashSet::new();
        let mut in_work = HashSet::new();
        let mut workers: Vec<Option<(char, usize)>> = vec![None; if part1 { 1 } else { 4 }];
        let mut time = 0;
        loop {
            // check which steps are finished
            for w in workers.iter_mut() {
                if let Some((step, remaining)) = w {
                    *remaining -= 1;
                    if *remaining == 0 {
                        in_work.remove(step);
                        finished.insert(*step);
                        result.push(*step);
                        *w = None;
                    }
                }
            }

            // for every idle worker, find a step to work on
            for w in workers.iter_mut().filter(|w| w.is_none()) {
                for d in &deps {
                    if in_work.contains(&d.0) {
                        // this step is already in work
                        continue;
                    }
                    if finished.contains(&d.0) {
                        // this step is already finished
                        continue;
                    }
                    if d.1.iter().all(|d| finished.contains(d)) {
                        // we found a new item to work on
                        in_work.insert(d.0);
                        *w = Some((d.0, (d.0 as u8 - b'A') as usize + 1 + 60));
                        break;
                    }
                }
            }

            if finished.len() == deps.len() {
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
