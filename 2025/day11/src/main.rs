use std::collections::HashMap;
use std::fs;

fn count<'a>(
    pos: &'a str,
    end: &'a str,
    edges: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if pos == end {
        return 1;
    }
    if let Some(c) = cache.get(&pos) {
        return *c;
    }

    let mut result = 0;
    if let Some(neighbors) = edges.get(pos) {
        for n in neighbors {
            result += count(n, end, edges, cache);
        }
    }
    cache.insert(pos, result);
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in input.lines() {
        let (from, to) = l.split_once(": ").unwrap();
        for t in to.split_ascii_whitespace() {
            edges.entry(from).or_default().push(t);
        }
    }

    // part 1
    println!("{}", count("you", "out", &edges, &mut HashMap::new()));

    let svr_to_fft = count("svr", "fft", &edges, &mut HashMap::new());
    let fft_to_dac = count("fft", "dac", &edges, &mut HashMap::new());
    let dac_to_out = count("dac", "out", &edges, &mut HashMap::new());

    let svr_to_dac = count("svr", "dac", &edges, &mut HashMap::new());
    let dac_to_fft = count("dac", "fft", &edges, &mut HashMap::new());
    let fft_to_out = count("fft", "out", &edges, &mut HashMap::new());

    println!(
        "{}",
        svr_to_fft * fft_to_dac * dac_to_out + svr_to_dac * dac_to_fft * fft_to_out
    );
}
