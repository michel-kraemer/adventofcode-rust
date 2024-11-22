use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug)]
enum Mod<'a> {
    Broadcaster {
        dests: Vec<&'a str>,
    },
    FlipFlop {
        on: bool,
        dests: Vec<&'a str>,
    },
    Conjunction {
        input_states: RefCell<HashMap<&'a str, bool>>,
        dests: Vec<&'a str>,
    },
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // get all modules and their destinations
    let mut modules = HashMap::new();
    for l in &lines {
        let (from, to) = l.split_once(" -> ").unwrap();
        let dests = to.split(", ").collect::<Vec<_>>();
        if from == "broadcaster" {
            modules.insert("broadcaster", Mod::Broadcaster { dests });
        } else if let Some(name) = from.strip_prefix("%") {
            modules.insert(name, Mod::FlipFlop { on: false, dests });
        } else if let Some(name) = from.strip_prefix("&") {
            modules.insert(
                name,
                Mod::Conjunction {
                    input_states: Default::default(),
                    dests,
                },
            );
        }
    }

    // get predecessors of each conjunction
    for (name, m) in &modules {
        let dests = match m {
            Mod::Broadcaster { dests } => dests,
            Mod::FlipFlop { dests, .. } => dests,
            Mod::Conjunction { dests, .. } => dests,
        };
        for dest in dests {
            if let Some(Mod::Conjunction { input_states, .. }) = modules.get(dest) {
                input_states.borrow_mut().insert(name, false);
            }
        }
    }

    // assumption for part 2: 'rx' has only one input and its a conjunction
    let mut input_of_rx = None;
    for (name, m) in &modules {
        if let Mod::Conjunction {
            input_states,
            dests,
        } = m
        {
            if dests.iter().any(|d| *d == "rx") {
                input_of_rx = Some((*name, input_states.borrow().len()));
                break;
            }
        };
    }

    // Record cycle lengths of all inputs of 'rx'. Assumption for part 2: the
    // cycles all start at step 1.
    let mut cycle_lengths: HashMap<&str, usize> = HashMap::new();

    let mut lows: usize = 0;
    let mut highs: usize = 0;
    let mut steps: usize = 0;
    let mut part1_total: usize = 0;
    loop {
        steps += 1;

        let mut q = VecDeque::new();
        q.push_back(("", "broadcaster", false));

        while let Some((from, to, pulse)) = q.pop_front() {
            // count high and low pulses
            if pulse {
                highs += 1;
            } else {
                lows += 1;
            }

            // apply module logic
            if let Some(m) = modules.get_mut(to) {
                match m {
                    Mod::Broadcaster { dests } => {
                        for next in dests {
                            q.push_back((to, next, pulse));
                        }
                    }

                    Mod::FlipFlop { on, dests } => {
                        if !pulse {
                            *on = !*on;
                            for next in dests {
                                q.push_back((to, next, *on));
                            }
                        }
                    }

                    Mod::Conjunction {
                        input_states,
                        dests,
                    } => {
                        *input_states.borrow_mut().get_mut(from).unwrap() = pulse;

                        // Once an input of 'input_of_rx' has received a high pulse,
                        // we assume we know the regular interval at which it will
                        // receive high pulses. Record this interval for each input.
                        // As soon as we've recorded the intervals for all inputs,
                        // we can use lcm to calculate how many button presses we need
                        // until all of them receive a high pulse at the same time.
                        if pulse && to == input_of_rx.unwrap().0 {
                            cycle_lengths.entry(from).or_insert(steps);
                        }

                        let new_pulse = !input_states.borrow().values().all(|v| *v);
                        for next in dests {
                            q.push_back((to, next, new_pulse));
                        }
                    }
                }
            }
        }

        if steps == 1000 {
            part1_total = lows * highs;
        }

        if steps >= 1000 && cycle_lengths.len() == input_of_rx.unwrap().1 {
            break;
        }
    }

    // part 1
    println!("{}", part1_total);

    // part 2
    let l = cycle_lengths
        .values()
        .copied()
        .reduce(num::integer::lcm)
        .unwrap();
    println!("{}", l);
}
