use std::{collections::HashMap, fs};

fn get_orbits<'a>(
    n: &'a str,
    map: &HashMap<&'a str, &'a str>,
    orbits: &mut HashMap<&'a str, Vec<&'a str>>,
) {
    if let Some(&prev) = map.get(n) {
        if !orbits.contains_key(prev) {
            get_orbits(prev, map, orbits);
        }
        let mut c = orbits.get(prev).unwrap().clone();
        c.push(n);
        orbits.insert(n, c);
    } else {
        orbits.insert(n, vec![n]);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let map = input
        .lines()
        .map(|l| {
            let r = l.split_once(')').unwrap();
            (r.1, r.0)
        })
        .collect::<HashMap<_, _>>();

    let mut orbits = HashMap::new();
    for o in map.keys() {
        get_orbits(o, &map, &mut orbits);
    }

    // part 1
    println!("{}", orbits.values().map(|l| l.len() - 1).sum::<usize>());

    // part 2
    let y = &orbits["YOU"];
    let s = &orbits["SAN"];
    for i in (0..y.len()).rev() {
        if let Some(j) = s.iter().rev().position(|&n| n == y[i]) {
            println!("{}", y.len() - i + j - 3);
            break;
        }
    }
}
