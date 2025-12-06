use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // part 1
    let mut rows = Vec::new();
    for l in lines.iter().take(lines.len() - 1) {
        rows.push(
            l.split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let ops = lines[lines.len() - 1].split_ascii_whitespace();

    let mut total1 = 0;
    for (i, op) in ops.enumerate() {
        let j = rows.iter().map(|r| r[i]);
        total1 += if op == "+" {
            j.sum::<i64>()
        } else {
            j.product::<i64>()
        };
    }
    println!("{total1}");

    // part 2
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let mut total2 = 0;
    let mut current_val = 0;
    let mut current_op = 0;
    for x in 0..width {
        let mut n = 0;
        for y in 0..height {
            let c = grid[y * width + x];
            if c.is_ascii_digit() {
                n *= 10;
                n += (c - b'0') as i64;
            } else if c != b' ' {
                current_op = c;
                if current_op == b'+' {
                    current_val = 0;
                } else {
                    current_val = 1;
                }
            }
        }

        if n == 0 {
            total2 += current_val;
        } else if current_op == b'+' {
            current_val += n;
        } else {
            current_val *= n;
        }
    }
    total2 += current_val;

    println!("{total2}");
}
