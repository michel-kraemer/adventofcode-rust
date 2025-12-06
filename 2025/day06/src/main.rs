use std::fs;

/// Transpose the given list of lines so that columns become rows
fn transpose(lines: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();
    for x in 0..lines[0].len() {
        let mut col = String::new();
        for l in &lines {
            col.push(l.as_bytes()[x] as char);
        }
        result.push(col);
    }
    result
}

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
    let lines = transpose(lines);
    let mut total2 = 0;
    let mut current = 0;
    let mut current_op = "";
    for l in &lines {
        // whenever we find an empty line, update `total2` and reset `current`
        // and `current_op`
        if l.trim_ascii().is_empty() {
            total2 += current;
            current = 0;
            current_op = "";
            continue;
        }

        // At the beginning and after an empty line, `current_op` is empty. The
        // current line should contain a new operator.
        let (n, op) = l.split_at(l.len() - 1);
        if current_op.is_empty() {
            current_op = op;
            if op == "+" {
                current = 0;
            } else {
                current = 1;
            }
        }

        // update current value
        let n = n.trim_ascii().parse::<i64>().unwrap();
        if current_op == "+" {
            current += n;
        } else {
            current *= n;
        }
    }
    total2 += current; // add the final value

    println!("{total2}");
}
