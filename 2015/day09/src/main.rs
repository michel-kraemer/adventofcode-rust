use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn dfs<'a>(
    distances: &HashMap<&'a str, HashMap<&'a str, usize>>,
    city: &'a str,
    seen: &mut HashSet<&'a str>,
    part1: bool,
) -> Option<usize> {
    if seen.len() == distances.len() - 1 {
        return Some(0);
    }

    seen.insert(city);

    let mut result = None;
    for (dest, dist) in &distances[&city] {
        if !seen.contains(dest) {
            if let Some(r) = dfs(distances, dest, seen, part1) {
                if part1 {
                    result = Some(result.unwrap_or(usize::MAX).min(dist + r));
                } else {
                    result = Some(result.unwrap_or(usize::MIN).max(dist + r));
                }
            }
        }
    }

    seen.remove(city);

    result
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let distances_list = input
            .lines()
            .map(|l| l.split_once(" = ").unwrap())
            .map(|p| {
                let locs = p.0.split_once(" to ").unwrap();
                (locs.0, locs.1, p.1.parse::<usize>().unwrap())
            })
            .collect::<Vec<_>>();

        let mut distances: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
        for d in distances_list {
            distances.entry(d.0).or_default().insert(d.1, d.2);
            distances.entry(d.1).or_default().insert(d.0, d.2);
        }

        let mut seen: HashSet<&str> = HashSet::new();
        let r = distances
            .iter()
            .map(|(city, _)| dfs(&distances, city, &mut seen, part1).unwrap());
        let r = if part1 {
            r.min().unwrap()
        } else {
            r.max().unwrap()
        };

        println!("{}", r);
    }
}
