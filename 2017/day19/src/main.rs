use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let s = grid[0].iter().position(|b| *b == b'|').unwrap();

    let mut x = s as i32;
    let mut y = 0i32;
    let mut dir = (0i32, 1i32);

    let mut steps = 0;
    let mut letters = Vec::new();
    loop {
        let c = grid[y as usize][x as usize];
        if c.is_ascii_uppercase() {
            letters.push(c as char);
        } else if c == b'+' {
            if dir.0 == 0 {
                // we're currently moving vertically
                if x > 0 && grid[y as usize][(x - 1) as usize] != b' ' {
                    dir = (-1, 0);
                } else if (x as usize) < grid[y as usize].len()
                    && grid[y as usize][(x + 1) as usize] != b' '
                {
                    dir = (1, 0);
                } else {
                    break;
                }
            } else {
                // we're currently moving horizontally
                if y > 0 && grid[(y - 1) as usize][x as usize] != b' ' {
                    dir = (0, -1);
                } else if (y as usize) < grid.len() && grid[(y + 1) as usize][x as usize] != b' ' {
                    dir = (0, 1);
                } else {
                    break;
                }
            }
        } else if c == b' ' {
            break;
        }

        x += dir.0;
        y += dir.1;
        steps += 1;
    }

    // part 1
    println!("{}", String::from_iter(letters));

    // part 2
    println!("{steps}");
}
