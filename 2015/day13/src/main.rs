use std::{collections::HashMap, fs};

fn permutations(mut people: Vec<&str>) -> Vec<Vec<&str>> {
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
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut scores = input
            .lines()
            .map(|l| {
                let mut parts = l.split_ascii_whitespace();
                let name = parts.next().unwrap();
                let gainlose = parts.nth(1).unwrap();
                let mut points = parts.next().unwrap().parse::<i64>().unwrap();
                let other = parts.nth(6).unwrap();
                if gainlose == "lose" {
                    points = -points;
                }
                ((name, &other[..other.len() - 1]), points)
            })
            .collect::<HashMap<_, _>>();

        let mut people = scores.keys().map(|s| s.0).collect::<Vec<_>>();
        people.sort_unstable();
        people.dedup();

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

        println!("{max_happiness}");
    }
}
