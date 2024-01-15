use std::{
    collections::{HashMap, HashSet},
    fs,
};

enum Event {
    Guard(usize),
    Asleep(usize),
    Wakeup(usize),
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // sort by date and time
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort();

    // parse
    let mut all_guards = HashSet::new();
    let events = lines
        .into_iter()
        .map(|l| {
            let p = l.split(' ').collect::<Vec<_>>();
            let time = &p[1][..p[1].len() - 1];
            let (_, minute) = time.split_once(':').unwrap();
            let minute = minute.parse::<usize>().unwrap();

            if p[3].starts_with('#') {
                let g = p[3][1..].parse::<usize>().unwrap();
                all_guards.insert(g);
                Event::Guard(g)
            } else if p[3] == "asleep" {
                Event::Asleep(minute)
            } else {
                Event::Wakeup(minute)
            }
        })
        .collect::<Vec<_>>();

    let mut hours = all_guards
        .iter()
        .map(|&g| (g, vec![0; 60]))
        .collect::<HashMap<_, _>>();

    let mut current_guard = 0;
    let mut start_sleep = 0;
    for e in events {
        match e {
            Event::Guard(g) => {
                current_guard = g;
                all_guards.insert(g);
            }
            Event::Asleep(minute) => start_sleep = minute,
            Event::Wakeup(minute) => {
                if let Some(h) = hours.get_mut(&current_guard) {
                    (start_sleep..minute).for_each(|i| {
                        h[i] += 1;
                    });
                }
            }
        }
    }

    // part 1
    let guard_with_max_sleep = hours
        .iter()
        .map(|(g, h)| (*g, h.iter().sum::<usize>()))
        .max_by_key(|g| g.1)
        .unwrap()
        .0;
    let max_minute_of_guard_with_max_sleep = hours
        .get(&guard_with_max_sleep)
        .unwrap()
        .iter()
        .enumerate()
        .max_by_key(|h| *h.1)
        .unwrap()
        .0;
    println!(
        "{}",
        guard_with_max_sleep * max_minute_of_guard_with_max_sleep
    );

    // part 2
    let guard_most_asleep = hours
        .iter()
        .map(|(g, h)| (*g, h.iter().enumerate().max_by_key(|v| v.1).unwrap()))
        .max_by_key(|g| g.1 .1)
        .unwrap();
    println!("{}", guard_most_asleep.0 * guard_most_asleep.1 .0);
}
