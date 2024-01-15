use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Range,
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
    let events = lines
        .into_iter()
        .map(|l| {
            let p = l.split(' ').collect::<Vec<_>>();
            let time = &p[1][..p[1].len() - 1];
            let (_, minute) = time.split_once(':').unwrap();
            let minute = minute.parse::<usize>().unwrap();

            if p[3].starts_with('#') {
                Event::Guard(p[3][1..].parse::<usize>().unwrap())
            } else if p[3] == "asleep" {
                Event::Asleep(minute)
            } else {
                Event::Wakeup(minute)
            }
        })
        .collect::<Vec<_>>();

    let mut current_guard = 0;
    let mut start_sleep = 0;
    let mut guard_to_cumulated_sleep: HashMap<usize, usize> = HashMap::new();
    let mut guard_to_ranges: HashMap<usize, Vec<Range<usize>>> = HashMap::new();
    let mut all_guards = HashSet::new();

    for e in events {
        match e {
            Event::Guard(g) => {
                current_guard = g;
                all_guards.insert(g);
            }
            Event::Asleep(minute) => start_sleep = minute,
            Event::Wakeup(minute) => {
                *guard_to_cumulated_sleep.entry(current_guard).or_default() += minute - start_sleep;
                guard_to_ranges
                    .entry(current_guard)
                    .or_default()
                    .push(start_sleep..minute);
            }
        }
    }

    // part 1
    let guard_with_max_sleep = guard_to_cumulated_sleep
        .into_iter()
        .max_by_key(|g| g.1)
        .unwrap()
        .0;
    let ranges_of_guard = guard_to_ranges.get(&guard_with_max_sleep).unwrap();
    let mut max_times_asleep = 0;
    let mut max_minute = 0;
    for i in 0..60 {
        let mut sum = 0;
        for r in ranges_of_guard {
            if i >= r.start && i < r.end {
                sum += 1;
            }
        }
        if sum > max_times_asleep {
            max_times_asleep = sum;
            max_minute = i;
        }
    }
    println!("{}", guard_with_max_sleep * max_minute);

    // part 2
    max_times_asleep = 0;
    max_minute = 0;
    let mut max_guard = 0;
    for i in 0..60 {
        for g in &all_guards {
            let mut sum = 0;
            if let Some(gr) = guard_to_ranges.get(g) {
                for r in gr {
                    if i >= r.start && i < r.end {
                        sum += 1;
                    }
                }
                if sum > max_times_asleep {
                    max_times_asleep = sum;
                    max_minute = i;
                    max_guard = *g;
                }
            }
        }
    }

    println!("{}", max_guard * max_minute);
}
