use std::fs;

fn is_valid(x: i32, y: i32, grid: &[Vec<char>], d: &(i32, i32)) -> bool {
    if x < 0 || y < 0 || (x as usize) >= grid[0].len() || (y as usize) >= grid.len() {
        return false;
    }
    let c = grid[y as usize][x as usize];
    c.is_ascii_uppercase()
        || c == '+'
        || match d {
            (1, 0) | (-1, 0) => c == '-',
            (0, 1) | (0, -1) => c == '|',
            _ => unreachable!(),
        }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let s = grid[0].iter().position(|c| *c == '|').unwrap();

    let mut x = s as i32;
    let mut y = 0i32;
    let mut dir = (0i32, 1i32);

    let mut steps = 1;
    let mut letters = Vec::new();
    loop {
        let c = grid[y as usize][x as usize];
        if c.is_ascii_uppercase() {
            letters.push(c);
        }

        if c == '+' {
            for d in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if d == dir || d == (-dir.0, -dir.1) {
                    continue;
                }
                let nx = x + d.0;
                let ny = y + d.1;
                if is_valid(nx, ny, &grid, &d) {
                    x = nx;
                    y = ny;
                    dir = d;
                    break;
                }
            }
        } else {
            let nx = x + dir.0;
            let ny = y + dir.1;
            let nx2 = nx + dir.0;
            let ny2 = ny + dir.1;
            if is_valid(nx, ny, &grid, &dir) || is_valid(nx2, ny2, &grid, &dir) {
                x = nx;
                y = ny;
            } else {
                break;
            }
        }
        steps += 1;
    }

    // part 1
    println!("{}", String::from_iter(letters));

    // part 2
    println!("{}", steps);
}
