use std::fs;

/// Nodes always have three characters and all of them are lowercase letters
/// between 'a' and 'z', so we can compute a perfect hash
fn index(node: &str) -> usize {
    let bytes = node.as_bytes();
    (bytes[0] - b'a') as usize * 26 * 26
        + (bytes[1] - b'a') as usize * 26
        + (bytes[2] - b'a') as usize
}

/// Count the number of paths from the node `pos` to `end`
fn count(pos: usize, end: usize, edges: &Vec<Vec<usize>>, cache: &mut Vec<u64>) -> u64 {
    if pos == end {
        return 1;
    }
    let c = cache[pos];
    if c != u64::MAX {
        return c;
    }

    let mut result = 0;
    for n in &edges[pos] {
        result += count(*n, end, edges, cache);
    }

    cache[pos] = result;

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let you = index("you");
    let out = index("out");
    let svr = index("svr");
    let fft = index("fft");
    let dac = index("dac");

    let max_len = 26 * 26 * 26;

    let mut edges = vec![Vec::new(); max_len];
    for l in input.lines() {
        let (from, to) = l.split_once(": ").unwrap();
        for t in to.split_ascii_whitespace() {
            edges[index(from)].push(index(t));
        }
    }

    // part 1
    println!("{}", count(you, out, &edges, &mut vec![u64::MAX; max_len]));

    // part 2 ...
    // count the paths from svr to fft, fft to dac, dac to out
    let svr_to_fft = count(svr, fft, &edges, &mut vec![u64::MAX; max_len]);
    let fft_to_dac = count(fft, dac, &edges, &mut vec![u64::MAX; max_len]);
    let dac_to_out = count(dac, out, &edges, &mut vec![u64::MAX; max_len]);

    // count the paths from svr to dac, dac to fft, fft to out
    let svr_to_dac = count(svr, dac, &edges, &mut vec![u64::MAX; max_len]);
    let dac_to_fft = count(dac, fft, &edges, &mut vec![u64::MAX; max_len]);
    let fft_to_out = count(fft, out, &edges, &mut vec![u64::MAX; max_len]);

    // calculate total number of paths
    println!(
        "{}",
        svr_to_fft * fft_to_dac * dac_to_out + svr_to_dac * dac_to_fft * fft_to_out
    );
}
