use std::fs;

fn dfs(port: usize, bridges: &[u64; 64], seen: &mut [u64; 64]) -> (usize, usize, usize) {
    let mut strength = 0;
    let mut maxlen = 0;
    let mut maxlen_strength = 0;

    let mut km = bridges[port];
    while km > 0 {
        // select LSB and reset it
        let k = km.trailing_zeros() as usize;
        km &= km - 1;

        if seen[port] & (1 << k) > 0 {
            continue;
        }
        seen[port] |= 1 << k;
        seen[k] |= 1 << port;
        let (s, ml, mls) = dfs(k, bridges, seen);
        seen[port] &= !(1 << k);
        seen[k] &= !(1 << port);

        strength = strength.max(port + k + s);
        if ml + 1 > maxlen {
            maxlen = ml + 1;
            maxlen_strength = port + k + mls;
        } else if ml + 1 == maxlen {
            maxlen_strength = maxlen_strength.max(port + k + mls);
        }
    }

    (strength, maxlen, maxlen_strength)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // no port in the input is greater than 64
    let mut bridges: [u64; 64] = [0; 64];

    for l in input.lines() {
        let p = l.split_once('/').unwrap();
        let from = p.0.parse::<usize>().unwrap();
        let to = p.1.parse::<usize>().unwrap();
        bridges[from] |= 1 << to;
        bridges[to] |= 1 << from;
    }

    let mut seen = [0; 64];
    let (max_strength, _, maxlen_strength) = dfs(0, &bridges, &mut seen);

    // part 1
    println!("{max_strength}");

    // part 2
    println!("{maxlen_strength}");
}
