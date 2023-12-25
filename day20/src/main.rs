// While I was able to solve the first part on my own, I needed some help with the
// second part. Thanks go to HyperNeutrino (https://www.youtube.com/@hyper-neutrino /
// https://github.com/hyper-neutrino/advent-of-code/). See this video for more
// information: https://www.youtube.com/watch?v=lxm6i21O83k

use num::integer::lcm;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug)]
enum Module<'a> {
    Broadcaster {
        dests: Vec<&'a str>,
    },
    FlipFlop {
        dests: Vec<&'a str>,
        on: bool,
    },
    Conjunction {
        dests: Vec<&'a str>,
        inputs: HashMap<&'a str, bool>,
    },
}

fn record_pulse<'a>(
    pulse: (&'a str, &'a str, bool),
    lows: &mut usize,
    highs: &mut usize,
) -> (&'a str, &'a str, bool) {
    if pulse.2 {
        *highs += 1;
    } else {
        *lows += 1;
    }
    pulse
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut modules = input
            .lines()
            .map(|line| {
                let (name, destinations) = line.split_once(" -> ").unwrap();
                let destinations = destinations.split(", ").collect::<Vec<_>>();
                if name == "broadcaster" {
                    (
                        name,
                        Module::Broadcaster {
                            dests: destinations,
                        },
                    )
                } else if name.chars().nth(0).unwrap() == '%' {
                    (
                        &name[1..],
                        Module::FlipFlop {
                            dests: destinations,
                            on: false,
                        },
                    )
                } else {
                    (
                        &name[1..],
                        Module::Conjunction {
                            dests: destinations,
                            inputs: HashMap::new(),
                        },
                    )
                }
            })
            .collect::<HashMap<_, _>>();

        // for each node m, find all its predecessors
        let mut incoming_connections: HashMap<&str, Vec<&str>> = HashMap::new();
        for (name, m) in &modules {
            let dests = match m {
                Module::Broadcaster { dests } => dests,
                Module::FlipFlop { dests, on: _ } => dests,
                Module::Conjunction { dests, inputs: _ } => dests,
            };
            dests
                .iter()
                .for_each(|d| incoming_connections.entry(d).or_default().push(*name));
        }

        // make predecessors of conjuctions their inputs
        for (name, ic) in incoming_connections {
            modules.get_mut(name).map(|m| match m {
                Module::Conjunction { dests: _, inputs } => ic.iter().for_each(|n| {
                    inputs.insert(n, false);
                }),
                _ => {}
            });
        }

        // assumption for part 2: 'rx' has only one input and its a conjunction
        let mut input_of_rx = None;
        if !part1 {
            for (name, m) in &modules {
                match m {
                    Module::Conjunction { dests, inputs } => {
                        if dests.iter().any(|d| *d == "rx") {
                            input_of_rx = Some((*name, inputs.len()));
                        }
                    }
                    _ => {}
                };
            }
        }

        // number of low and high pulses
        let mut lows = 0;
        let mut highs = 0;

        // number of button presses
        let mut button_presses = 0;

        // the frequency at which each input of `input_of_rx` receives a high pulse
        let mut cycle_lengths: HashMap<&str, usize> = HashMap::new();

        'outer: loop {
            button_presses += 1;

            let mut pulses: VecDeque<(&str, &str, bool)> = VecDeque::new();
            pulses.push_back(("", "broadcaster", false));
            lows += 1;

            while !pulses.is_empty() {
                let (src, dest, pulse) = pulses.pop_front().unwrap();

                // Once an input of 'input_of_rx' has received a high pulse,
                // we assume we know the regular interval at which it will
                // receive high pulses. Record this interval for each input.
                // As soon as we've recorded the intervals for all inputs,
                // we can use lcm to calculate how many button presses we need
                // until all of them receive a high pulse at the same time.
                if let Some(input_of_rx) = input_of_rx {
                    if dest == input_of_rx.0 && pulse {
                        cycle_lengths.insert(src, button_presses);

                        if cycle_lengths.len() == input_of_rx.1 {
                            break 'outer;
                        }
                    }
                }

                // process machine logic
                modules.get_mut(dest).map(|m| match m {
                    Module::Broadcaster { dests } => {
                        for d in dests {
                            pulses.push_back(record_pulse((dest, d, pulse), &mut lows, &mut highs));
                        }
                    }
                    Module::FlipFlop { dests, on } => {
                        if !pulse {
                            *on = !*on;
                            for d in dests {
                                pulses.push_back(record_pulse(
                                    (dest, d, *on),
                                    &mut lows,
                                    &mut highs,
                                ));
                            }
                        }
                    }
                    Module::Conjunction { dests, inputs } => {
                        inputs.insert(src, pulse).unwrap();
                        let all_high = inputs.iter().all(|i| *i.1);
                        for d in dests {
                            pulses.push_back(record_pulse(
                                (dest, d, !all_high),
                                &mut lows,
                                &mut highs,
                            ));
                        }
                    }
                });
            }

            if part1 && button_presses == 1000 {
                break;
            }
        }

        if part1 {
            println!("{}", lows * highs);
        } else {
            let r = cycle_lengths
                .values()
                .into_iter()
                .map(|i| *i)
                .reduce(|a, b| lcm(a, b))
                .unwrap();
            println!("{}", r);
        }
    }
}
