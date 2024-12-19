use std::fs;

use trie::Trie;

mod trie;

fn dfs(design: &[u8], patterns: &Trie, cache: &mut [usize]) -> usize {
    if design.is_empty() {
        return 1;
    }

    let c = cache[design.len() - 1];
    if c != usize::MAX {
        return c;
    }

    let mut r = 0;
    // optimization: try the longest prefixes first
    for l in patterns.common_prefix_lengths(design).into_iter().rev() {
        r += dfs(&design[l..], patterns, cache);
    }

    cache[design.len() - 1] = r;
    r
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let patterns = lines[0]
        .split(", ")
        .map(|p| p.as_bytes())
        .collect::<Vec<_>>();
    let designs = &lines[2..];

    // create index structure that allows us to quickly search for common
    // prefix lengths
    let mut trie = Trie::default();
    for p in &patterns {
        trie.insert(p);
    }

    let mut total1 = 0;
    let mut total2 = 0;
    let mut cache = vec![usize::MAX; 100];
    for d in designs {
        cache.fill(usize::MAX);
        cache.resize(cache.len().max(d.len()), usize::MAX);
        let c = dfs(d.as_bytes(), &trie, &mut cache);
        if c > 0 {
            total1 += 1;
            total2 += c;
        }
    }

    println!("{}", total1);
    println!("{}", total2);
}
