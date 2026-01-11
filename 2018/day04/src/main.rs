use std::{cmp::Ordering, fs};

use rustc_hash::FxHashMap;

#[derive(PartialEq, Eq, Debug)]
enum EventType {
    Guard(usize),
    Wakeup,
    Asleep,
}

#[derive(PartialEq, Eq, Debug)]
struct Event {
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    event_type: EventType,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.month
            .cmp(&other.month)
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.minute.cmp(&other.minute))
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // parse events
    let mut guards: FxHashMap<usize, Vec<i64>> = FxHashMap::default();
    let mut events = Vec::new();
    for l in input.lines() {
        let event_type = match l.as_bytes()[19] {
            b'G' => {
                let (id, _) = l[26..].split_once(' ').unwrap();
                let id = id.parse::<usize>().unwrap();
                guards.entry(id).or_insert_with(|| vec![0; 60]);
                EventType::Guard(id)
            }
            b'w' => EventType::Wakeup,
            b'f' => EventType::Asleep,
            _ => unreachable!(),
        };

        // all entries are from the same year - skip parsing it
        let event = Event {
            month: l[6..8].parse().unwrap(),
            day: l[9..11].parse().unwrap(),
            hour: l[12..14].parse().unwrap(),
            minute: l[15..17].parse().unwrap(),
            event_type,
        };
        events.push(event);
    }

    // sort events by time
    events.sort_unstable();

    // Iterate through events and assign them to each guard's timetable.
    // Increase the value in the timetable if the guard falls asleep and
    // decrease it they wake up. This allows us later to iterate over the time
    // tables, compute a running sum, calculate the number of minutes asleep,
    // and find the minute the guard was asleep the most.
    let mut current_guard = guards.iter_mut().next().unwrap().1;
    for e in events {
        match e.event_type {
            EventType::Guard(id) => current_guard = guards.get_mut(&id).unwrap(),
            EventType::Wakeup => current_guard[e.minute as usize] -= 1,
            EventType::Asleep => current_guard[e.minute as usize] += 1,
        }
    }

    let mut part1_max_minutes_asleep = 0;
    let mut total1 = 0;

    let mut part2_max = 0;
    let mut total2 = 0;

    for (id, minutes) in guards {
        // Iterate over all minutes and compute total minutes asleep. Also find
        // the one minute where the guard was asleep the most. Since the problem
        // statement clearly says that we're looking for a single minute, we
        // only need to consider the minutes when the guard falls asleep and
        // when they wake up again.
        let mut minutes_asleep = 0;
        let mut last_minute = 0;
        let mut sum = 0;
        let mut max = 0;
        let mut max_minute = 0;
        for (minute, v) in minutes.into_iter().enumerate() {
            if v != 0 {
                minutes_asleep += (minute - last_minute) as i64 * sum;
                sum += v;
                if sum > max {
                    max = sum;
                    max_minute = minute;
                }
                last_minute = minute;
            }
        }

        // part 1
        if minutes_asleep > part1_max_minutes_asleep {
            total1 = id * max_minute;
            part1_max_minutes_asleep = minutes_asleep;
        }

        // part 2
        if max > part2_max {
            total2 = id * max_minute;
            part2_max = max;
        }
    }

    println!("{total1}");
    println!("{total2}");
}
