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
    // we first find the right ring and then check which edge the number we are
    // are looking lies on
    let mut ring = 1;
    let mut n = 3;
    while n * n < input {
        ring += 1;
        n += 2;
    }
    let lower_right = n * n;
    let lower_left = lower_right - (n - 1);
    let upper_left = lower_left - (n - 1);
    let upper_right = upper_left - (n - 1);
    let prev = (n - 2) * (n - 2) + 1;
    let total1 = if (prev..=upper_right).contains(&input) {
        // right edge
        let steps_x = ring;
        let steps_y = ((prev - 1 + upper_right) / 2).abs_diff(input);
        steps_x + steps_y
    } else if (upper_right..=upper_left).contains(&input) {
        // top edge
        let steps_x = ((upper_right + upper_left) / 2).abs_diff(input);
        let steps_y = ring;
        steps_x + steps_y
    } else if (upper_left..=lower_left).contains(&input) {
        // left edge
        let steps_x = ring;
        let steps_y = ((upper_left + lower_left) / 2).abs_diff(input);
        steps_x + steps_y
    } else {
        // bottom edge
        let steps_x = ((lower_left + lower_right) / 2).abs_diff(input);
        let steps_y = ring;
        steps_x + steps_y
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
