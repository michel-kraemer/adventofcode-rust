use std::fs;

/// Performs the extended Euclidean algorithm. Based on the pseudo-code from
/// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm but we're only
/// interested in `y`.
fn extended_gcd(a: i64, b: i64) -> i64 {
    let (mut old_r, mut r) = (a, b);
    let (mut old_y, mut y) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_y, y) = (y, old_y - quotient * y);
    }

    old_y
}

/// Chinese remainder theorem
///
/// Solve a system of congruences:
///
/// ```text
/// x ≡ remainders[0] mod moduli[0]
/// x ≡ remainders[1] mod moduli[1]
/// ...
/// x ≡ remainders[k] mod moduli[k]
/// ```
pub fn chinese_remainder(remainders: &[i64], moduli: &[i64]) -> i64 {
    // 1. multiply all moduli
    let n = moduli.iter().product();

    // 2. divide this product by the moduli
    let m = moduli.iter().map(|&v| n / v);

    // 3. apply the extended Euclidean algorithm and multiply y_i by m_i
    // to obtain e_i
    let e = moduli.iter().zip(m).map(|(&r, m)| extended_gcd(r, m) * m);

    // 4. calculate the sum of the products of remainders_i and e_i
    let result = remainders.iter().zip(e).map(|(&r, e)| r * e).sum::<i64>();

    // 5. make the result positive and as small as possible
    result.rem_euclid(n)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let discs = input
        .lines()
        .map(|l| {
            let p = l.split(' ').collect::<Vec<_>>();
            (
                // number of positions (= disc size)
                p[3].parse::<i64>().unwrap(),
                // initial position at t=0
                p[11].strip_suffix('.').unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    for part1 in [true, false] {
        let mut discs = discs.clone();

        // part 2: add another disc with size 11 starting at 0
        if !part1 {
            discs.push((11, 0));
        }

        // for each individual disc, calculate when we would need to push the
        // button so that the capsule falls through the disc's slot when it
        // arrives at the disc.
        let remainders = discs
            .iter()
            .enumerate()
            .map(|(i, d)| (d.0 - d.1 - (i as i64 + 1)).rem_euclid(d.0))
            .collect::<Vec<_>>();

        // get the disc sizes
        let moduli = discs.iter().map(|d| d.0).collect::<Vec<_>>();

        println!("{}", chinese_remainder(&remainders, &moduli));
    }
}
