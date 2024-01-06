use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn get_group<'a>(pipes: &HashMap<&'a str, HashSet<&'a str>>, start: &str) -> HashSet<&'a str> {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut seen = HashSet::new();

    while !queue.is_empty() {
        let n = queue.pop_front().unwrap();
        let ds = &pipes[n];
        for d in ds {
            if !seen.contains(d) {
                seen.insert(*d);
                queue.push_back(*d);
            }
        }
    }

    seen
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut pipes: HashMap<&str, HashSet<&str>> = HashMap::new();

    for l in input.lines() {
        let (src, dsts) = l.split_once(" <-> ").unwrap();
        let dsts = dsts.split(", ").collect::<Vec<_>>();
        for d in dsts {
            pipes.entry(src).or_default().insert(d);
            pipes.entry(d).or_default().insert(src);
        }
    }

    let mut zero = get_group(&pipes, "0");

    // part 1
    println!("{}", zero.len());

    let mut groups = 1;
    loop {
        for z in &zero {
            pipes.remove(z);
        }
        if pipes.is_empty() {
            break;
        }
        groups += 1;
        zero = get_group(&pipes, pipes.iter().next().unwrap().0);
    }

    // part 2
    println!("{}", groups);
}
