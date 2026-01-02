use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input.trim().parse::<u64>().unwrap();

    // part 1 - the numbers in the corners of each ring can be computed as
    // follows, where n is an odd number:
    // n * n                  # lower-right corner
    // n * n - 1 * (n - 1)    # lower-left corner
    // n * n - 2 * (n - 1)    # upper-left corner
    // n * n - 3 * (n - 1)    # upper-right corner
    // we iterate through all rings and check if the number we are looking for
    // lies on one of the four edges
    let mut ring = 1;
    let mut n = 3;
    let total1 = loop {
        let lower_right = n * n;
        let lower_left = lower_right - (n - 1);
        let upper_left = lower_left - (n - 1);
        let upper_right = upper_left - (n - 1);
        let prev = (n - 2) * (n - 2) + 1;

        // right edge
        if (prev..=upper_right).contains(&input) {
            let steps_x = ring;
            let steps_y = ((prev - 1 + upper_right) / 2).abs_diff(input);
            break steps_x + steps_y;
        }

        // top edge
        if (upper_right..=upper_left).contains(&input) {
            let steps_x = ((upper_right + upper_left) / 2).abs_diff(input);
            let steps_y = ring;
            break steps_x + steps_y;
        }

        // left edge
        if (upper_left..=lower_left).contains(&input) {
            let steps_x = ring;
            let steps_y = ((upper_left + lower_left) / 2).abs_diff(input);
            break steps_x + steps_y;
        }

        // bottom edge
        if (lower_left..=lower_right).contains(&input) {
            let steps_x = ((lower_left + lower_right) / 2).abs_diff(input);
            let steps_y = ring;
            break steps_x + steps_y;
        }

        ring += 1;
        n += 2;
    };
    println!("{total1}");

    // part 2 - the numbers grow very quickly, so we can just create a small
    // grid instead of a HashMap
    let size = 31; // this is more than enough for the whole u64 range
    let mut grid = vec![0; size * size];
    let center = (size as i32 / 2, size as i32 / 2);
    grid[center.1 as usize * size + center.0 as usize] = 1;
    let mut w = 1;
    let mut s = 2;
    let mut total2 = 0;
    'outer: while s <= input {
        let mut x: i32 = w / 2;
        let mut y: i32 = w / 2;
        for dir in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            for _ in 0..w - 1 {
                x += dir.0;
                y += dir.1;
                let mut sum = 0;
                for ny in y - 1..=y + 1 {
                    for nx in x - 1..=x + 1 {
                        sum += grid[(ny + center.1) as usize * size + (nx + center.0) as usize];
                    }
                }
                grid[(y + center.1) as usize * size + (x + center.0) as usize] = sum;
                if sum > input {
                    total2 = sum;
                    break 'outer;
                }
                s += 1;
            }
        }
        w += 2;
    }
    println!("{total2}");
}
