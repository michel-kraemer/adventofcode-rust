use std::fs;

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, -1),
    (-1, 1),
    (1, 1),
    (-1, -1),
];

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            // part 1
            if grid[y][x] == WORD[0] {
                for d in &DIRS {
                    let mut found = true;
                    for (j, w) in WORD.iter().skip(1).enumerate() {
                        let nx = x as i32 + d.0 * (j as i32 + 1);
                        let ny = y as i32 + d.1 * (j as i32 + 1);
                        if nx < 0
                            || ny < 0
                            || nx >= grid[0].len() as i32
                            || ny >= grid.len() as i32
                            || grid[ny as usize][nx as usize] != *w
                        {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        part1 += 1;
                    }
                }
            }

            // part 2
            if grid[y][x] == 'A' && x > 0 && y > 0 && x < grid[0].len() - 1 && y < grid.len() - 1 {
                let c1 = grid[y - 1][x - 1];
                let c2 = grid[y + 1][x + 1];
                let c3 = grid[y - 1][x + 1];
                let c4 = grid[y + 1][x - 1];
                if ((c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M'))
                    && ((c3 == 'M' && c4 == 'S') || (c3 == 'S' && c4 == 'M'))
                {
                    part2 += 1;
                }
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
