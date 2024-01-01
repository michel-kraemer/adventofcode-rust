use std::fs;

use num::integer::Integer;

fn main() {
    for part1 in [true, false] {
        // parse input
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut discs = input
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

        // add another disc to part 2 with size 11 starting at 0
        if !part1 {
            discs.push((11, 0));
        }

        // for each individual disc, calculate when we would need to push the
        // button so that the capsule falls through the disc's slot when it
        // arrives at the disc.
        let a = discs
            .iter()
            .enumerate()
            .map(|(i, d)| (d.0 - d.1 - (i as i64 + 1)).rem_euclid(d.0))
            .collect::<Vec<_>>();

        // get the disc sizes so we can create a system of congruences such that:
        // x ≡ a[0] mod r[0]
        // x ≡ a[1] mod r[1]
        // ...
        // x ≡ a[k] mod r[k]
        let r = discs.iter().map(|d| d.0).collect::<Vec<_>>();

        // Chinese Remainder theorem:
        // 1. multiple all remainders (i.e. disc sizes)
        let n = r.iter().copied().reduce(|a, b| a * b).unwrap();

        // 2. divide this product by the disc sizes
        let m = r.iter().map(|v| n / v).collect::<Vec<_>>();

        // 3. apply the extended Eucledian algorithm and multiply y_i by m_i
        // to obtain e_i
        let e = r
            .iter()
            .zip(m.iter())
            .map(|(r, m)| i64::extended_gcd(r, m).y * m)
            .collect::<Vec<_>>();

        // 4. calculate the sum of the products of a_i and e_i
        let mut result = a.iter().zip(e.iter()).map(|(a, e)| a * e).sum::<i64>();

        // 5. make the result positive
        while result < 0 {
            result += n;
        }

        println!("{}", result);
    }
}
