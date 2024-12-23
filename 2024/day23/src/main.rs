use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in lines {
        let (a, b) = l.split_once("-").unwrap();
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
    }

    let mut clusters: HashSet<Vec<&str>> = HashSet::new();
    for &start in map.keys() {
        let mut queue = VecDeque::new();
        queue.push_back((start, vec![start]));

        while let Some((computer, seen)) = queue.pop_front() {
            for &n in &map[computer] {
                if seen.contains(&n) {
                    continue;
                }
                if !seen.iter().all(|&s| map[s].contains(&n)) {
                    continue;
                }

                let mut new_seen = seen.clone();
                let i = new_seen.partition_point(|o| o.cmp(&n) == Ordering::Less);
                new_seen.insert(i, n);

                if !clusters.contains(&new_seen) {
                    clusters.insert(new_seen.clone());
                    queue.push_back((n, new_seen));
                }
            }
        }
    }

    // part 1
    let total1 = clusters
        .iter()
        .filter(|c| c.len() == 3 && c.iter().any(|computer| computer.starts_with('t')))
        .count();
    println!("{}", total1);

    // part 2
    let largest = clusters.iter().max_by_key(|c| c.len()).unwrap();
    println!("{}", largest.join(","));
}
