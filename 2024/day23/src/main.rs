use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for l in lines {
        let (a, b) = l.split_once("-").unwrap();
        map.entry(a).or_default().insert(b);
        map.entry(b).or_default().insert(a);
    }

    let mut clusters: Vec<Vec<&str>> = Vec::new();
    for &start in map.keys() {
        let mut queue = VecDeque::new();
        queue.push_back((start, vec![start]));

        while let Some((computer, seen)) = queue.pop_front() {
            for &n in &map[computer] {
                if n < computer {
                    continue;
                }
                let nn = &map[n];
                if seen.iter().any(|&s| !nn.contains(&s)) {
                    continue;
                }

                let mut new_seen = seen.clone();
                new_seen.push(n);

                clusters.push(new_seen.clone());
                queue.push_back((n, new_seen));
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
