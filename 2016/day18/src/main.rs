use std::fs;

#[allow(clippy::overly_complex_bool_expr, clippy::nonminimal_bool)]
fn is_trap(left: bool, center: bool, right: bool) -> bool {
    (left && center && !right)
        || (center && right && !left)
        || (left && !center && !right)
        || (right && !center && !left)
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut current_row = input.trim().chars().map(|c| c == '^').collect::<Vec<_>>();
        let mut rows = 0;
        let mut result = 0;

        let max = if part1 { 40 } else { 400000 };

        while rows < max {
            let mut next_row = Vec::with_capacity(current_row.len());
            for i in 0..current_row.len() {
                if !current_row[i] {
                    result += 1;
                }
                let t = if i == 0 {
                    is_trap(false, current_row[i], current_row[i + 1])
                } else if i == current_row.len() - 1 {
                    is_trap(current_row[i - 1], current_row[i], false)
                } else {
                    is_trap(current_row[i - 1], current_row[i], current_row[i + 1])
                };
                next_row.push(t);
            }
            rows += 1;
            current_row = next_row;
        }

        println!("{}", result);
    }
}
