use std::{collections::HashMap, fs};

type Dep<'a> = (i64, &'a str);

fn produce<'a>(
    what: &'a str,
    how_much: i64,
    reactions: &HashMap<&'a str, (i64, Vec<Dep<'a>>)>,
    produced: &mut HashMap<&'a str, i64>,
) -> i64 {
    let mut ore = 0;

    let r = reactions.get(what).unwrap();
    let f = (how_much + r.0 - 1) / r.0;
    let a = r.0 * f;
    let mut reqs = r.1.iter().rev().map(|d| (d.1, d.0 * f)).collect::<Vec<_>>();

    for (k, v) in &mut reqs {
        if let Some(p) = produced.get(k) {
            if p <= v {
                *v -= p;
                produced.remove(k);
            } else {
                produced.insert(k, *p - *v);
                *v = 0;
            }
        }
    }

    for (k, v) in reqs {
        if k == "ORE" {
            ore += v;
        } else {
            ore += produce(k, v, reactions, produced);
        }
    }

    *produced.entry(what).or_default() += a - how_much;

    ore
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let reactions = input
        .lines()
        .map(|l| {
            let p = l.split_once(" => ").unwrap();
            let left =
                p.0.split(", ")
                    .map(|le| {
                        let (a, w) = le.split_once(' ').unwrap();
                        (a.parse::<i64>().unwrap(), w)
                    })
                    .collect::<Vec<_>>();
            let r = p.1.split_once(' ').unwrap();
            let ra = r.0.parse::<i64>().unwrap();
            (r.1, (ra, left))
        })
        .collect::<HashMap<_, _>>();

    // part 1
    let mut produced: HashMap<&str, i64> = HashMap::new();
    let ore = produce("FUEL", 1, &reactions, &mut produced);
    println!("{}", ore);

    // part 2
    produced.clear();
    let mut total_ore = 1000000000000i64;
    let mut total_fuel = 0;
    let mut step = 1000000;
    loop {
        let mut tmp_produced = produced.clone();
        let ore = produce("FUEL", step, &reactions, &mut tmp_produced);
        if ore <= total_ore {
            produced = tmp_produced;
            total_fuel += step;
            total_ore -= ore;
        } else {
            if step == 1 {
                break;
            }
            step /= 2;
        }
    }
    println!("{}", total_fuel);
}
