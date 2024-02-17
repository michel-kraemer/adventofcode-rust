use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut input = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect::<Vec<_>>();

        if part1 {
            let base: [i64; 4] = [0, 1, 0, -1];
            for _ in 0..100 {
                let mut output = Vec::with_capacity(input.len());
                for o in 0..input.len() {
                    let mut sum = 0;
                    for (i, v) in input.iter().enumerate().skip(o) {
                        let bi = ((i + 1) / (o + 1)) % base.len();
                        sum += v * base[bi];
                    }
                    output.push(sum.abs() % 10);
                }
                input = output;
            }
            println!(
                "{}",
                input[0..8]
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<String>()
            );
        } else {
            let input_orig = input;
            let mut input: Vec<i64> = Vec::new();
            for _ in 0..10000 {
                input.extend(&input_orig);
            }

            let mut offset = 0usize;
            for &i in &input[0..7] {
                offset *= 10;
                offset += i as usize;
            }

            // Part 2 has a very simple solution that only works if the offset
            // is in the second half of the input. In this case, we just have
            // to sum up numbers from the end until the offset. This is because
            // the base pattern has a 0 at the first position and a 1 at the
            // second, which means that once we reach output position o_i with
            // i == input.len() / 2, the 0 will be repeated i times and the
            // rest will be filled with 1s (we can ignore the other two digits
            // of the base pattern), which means we can just use a running
            // sum from this point on.
            assert!(offset > input.len() / 2);

            for _ in 0..100 {
                let mut o = input.len() - 1;
                let mut sum = 0;
                while o >= offset {
                    sum = (sum + input[o]).abs() % 10;
                    input[o] = sum;
                    o -= 1;
                }
            }

            println!(
                "{}",
                input[offset..offset + 8]
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<String>()
            );
        }
    }
}
