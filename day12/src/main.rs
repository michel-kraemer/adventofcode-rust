use std::{fs, collections::HashMap};

fn does_match(a: &str, springs: usize) -> bool {
    let a = a.as_bytes();
    let mut i = 0;

    let mut springs = springs;
    while springs > 0 {
        if a[i] != b'#' && a[i] != b'?' {
            return false;
        }
        i += 1;
        springs -= 1;
        if springs > 0 && i == a.len() {
            return false;
        }
    }

    true
}

fn count_matches(si: usize, groups: &Vec<usize>, gi: usize, max_len: usize,
        conditions: &str, cache: &mut HashMap<String, usize>) -> usize {
    let mut key = String::from(&conditions[si..]);
    key.push('|');
    key.push_str(groups.iter().skip(gi).map(|g| g.to_string()).collect::<Vec<_>>().join(",").as_str());

    if let Some(c) = cache.get(&key) {
        return *c;
    }

    let min_spaces = if gi == 0 { 0 } else { 1 };
    let mut result: usize = 0;
    for spaces in min_spaces..max_len {
        if si + spaces > max_len {
            cache.insert(key, result);
            return result;
        }
        if spaces > 0 {
            if conditions.as_bytes()[si + spaces - 1] == b'#' {
                cache.insert(key, result);
                return result;
            }
        }
        if si + spaces + groups[gi] > max_len {
            cache.insert(key, result);
            return result;
        }
        if !does_match(&conditions[(si + spaces)..(si + spaces + groups[gi])], groups[gi]) {
            continue;
        }
        if gi < groups.len() - 1 {
            let next = count_matches(si + spaces + groups[gi], groups, gi + 1, max_len, &conditions, cache);
            result += next;
        } else {
            if !conditions[(si + spaces + groups[gi])..].contains('#') {
                result += 1;
            }
        }
    }

    cache.insert(key, result);

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut cache: HashMap<String, usize> = HashMap::new();

    let matches: usize = input.lines().map(|line| {
        let (conditions, groups) = line.split_once(" ").unwrap();
        let groups = groups.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();

        let mut unfolded_conditions = String::new();
        for i in 0..5 {
            if i > 0 {
                unfolded_conditions.push('?');
            }
            unfolded_conditions.push_str(conditions);
        }

        let mut unfolded_groups = Vec::new();
        for _ in 0..5 {
            for g in &groups {
                unfolded_groups.push(*g);
            }
        }

        count_matches(0, &unfolded_groups, 0,
            unfolded_conditions.len(), &unfolded_conditions, &mut cache)
    }).sum();

    println!("{}", matches);
}
