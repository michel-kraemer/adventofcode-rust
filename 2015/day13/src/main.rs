use std::{
    collections::{HashMap, HashSet},
    fs,
};

use regex::Regex;

fn permutations<'a>(mut people: Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut c = vec![0; people.len()];
    let mut result = Vec::new();

    result.push(people.clone());

    let mut i = 1;
    while i < people.len() {
        if c[i] < i {
            if i % 2 == 0 {
                people.swap(0, i);
            } else {
                people.swap(c[i], i);
            }

            result.push(people.clone());

            c[i] += 1;
            i = 1;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    result
}

fn main() {
    for part1 in [true, false] {
        let r = Regex::new(
            r"([a-zA-Z]+) would (gain|lose) (\d+) happiness units by sitting next to ([a-zA-Z]+).",
        )
        .unwrap();

        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut scores = input
            .lines()
            .map(|l| {
                let m = r.captures(l).unwrap();
                let mut points = m.get(3).unwrap().as_str().parse::<i64>().unwrap();
                if m.get(2).unwrap().as_str() == "lose" {
                    points = -points;
                }
                (
                    (m.get(1).unwrap().as_str(), m.get(4).unwrap().as_str()),
                    points,
                )
            })
            .collect::<HashMap<_, _>>();

        let mut people = scores
            .keys()
            .map(|s| s.0)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        if !part1 {
            for p in &people {
                scores.insert(("me", p), 0);
                scores.insert((p, "me"), 0);
            }
            people.push("me");
        }

        let perms = permutations(people);

        let max_happiness = perms
            .iter()
            .map(|p| {
                let mut happiness = 0i64;
                for w in p.windows(2) {
                    happiness += scores[&(w[0], w[1])];
                    happiness += scores[&(w[1], w[0])];
                }
                happiness += scores[&(p[p.len() - 1], p[0])];
                happiness += scores[&(p[0], p[p.len() - 1])];
                happiness
            })
            .max()
            .unwrap();

        println!("{}", max_happiness);
    }
}
