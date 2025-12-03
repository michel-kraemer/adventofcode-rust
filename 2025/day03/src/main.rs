use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut total1 = 0;
    let mut total2 = 0;
    let mut dp = vec![0; 13];
    for l in input.lines() {
        let bytes = l.as_bytes();
        if (bytes.len() + 1) * 13 > dp.len() {
            dp.resize((bytes.len() + 1) * 13, 0);
        } else {
            dp.fill(0);
        }
        let mut mul = 1;
        for len in 1..=12 {
            let mut max = 0;
            for (i, &b) in bytes.iter().enumerate().take(bytes.len() - len + 1).rev() {
                let d = (b - b'0') as u64;
                max = max.max(d * mul + dp[(len - 1) * 13 + i + 1]);
                dp[len * 13 + i] = max;
            }
            mul *= 10;
        }
        total1 += dp[2 * 13];
        total2 += dp[12 * 13];
    }

    println!("{total1}");
    println!("{total2}");
}
