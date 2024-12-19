use std::collections::HashMap;
use std::fs;

fn dfs<'a>(design: &'a str, patterns: &[&str], cache: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(c) = cache.get(design) {
        return *c;
    }

    let mut r = 0;
    for t in patterns {
        if let Some(rest) = design.strip_prefix(t) {
            r += dfs(rest, patterns, cache);
        }
    }

    cache.insert(design, r);
    r
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let patterns = lines[0].split(", ").collect::<Vec<_>>();
    let designs = &lines[2..];

    let mut seen = HashMap::new();
    let mut total1 = 0;
    let mut total2 = 0;
    for d in designs {
        let c = dfs(d, &patterns, &mut seen);
        if c > 0 {
            total1 += 1;
            total2 += c;
        }
    }

    println!("{}", total1);
    println!("{}", total2);
}
