use std::fs;

fn get_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No start found");
}

fn find_loop(
    grid: &Vec<Vec<char>>,
    start: &(usize, usize),
    dir: (i8, i8),
) -> Option<Vec<(usize, usize)>> {
    let mut x = start.0;
    let mut y = start.1;
    let mut dir = dir;
    let mut l = Vec::new();

    loop {
        match dir {
            (0, 1) => y += 1,
            (0, -1) => y -= 1,
            (1, 0) => x += 1,
            (-1, 0) => x -= 1,
            _ => panic!("Unknown direction: {:?}", dir),
        }
        l.push((x, y));

        match grid[y][x] {
            '|' => {
                if dir.1 == 0 {
                    return None;
                }
            }
            '-' => {
                if dir.0 == 0 {
                    return None;
                }
            }
            'L' => match dir {
                (0, 1) => dir = (1, 0),
                (-1, 0) => dir = (0, -1),
                _ => return None,
            },
            'J' => match dir {
                (0, 1) => dir = (-1, 0),
                (1, 0) => dir = (0, -1),
                _ => return None,
            },
            '7' => match dir {
                (0, -1) => dir = (-1, 0),
                (1, 0) => dir = (0, 1),
                _ => return None,
            },
            'F' => match dir {
                (0, -1) => dir = (1, 0),
                (-1, 0) => dir = (0, 1),
                _ => return None,
            },
            '.' => return None,
            'S' => return Some(l),
            _ => panic!("Unknown tile: {}", grid[y][x]),
        }
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let start = get_start(&grid);

        for dir in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
            if let Some(l) = find_loop(&grid, &start, dir) {
                if part1 {
                    println!("{}", l.len() / 2);
                    break;
                }

                let mut grid_copy = grid
                    .iter()
                    .map(|row| row.iter().map(|s| (*s, false)).collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                for (x, y) in l {
                    grid_copy[y][x].1 = true;
                }

                let mut tiles = 0;
                for row in grid_copy {
                    let mut inside = false;
                    let mut x = 0;
                    while x < row.len() {
                        let s = row[x];
                        if s.1 {
                            match s.0 {
                                '|' => inside = !inside,
                                'F' => {
                                    while !(row[x].0 == 'J' || row[x].0 == '7') {
                                        x += 1
                                    }
                                    if row[x].0 == 'J' {
                                        inside = !inside;
                                    }
                                }
                                'L' => {
                                    while !(row[x].0 == 'J' || row[x].0 == '7') {
                                        x += 1
                                    }
                                    if row[x].0 == '7' {
                                        inside = !inside;
                                    }
                                }
                                _ => {}
                            }
                        } else if inside {
                            tiles += 1;
                        }
                        x += 1;
                    }
                }
                println!("{}", tiles);
                break;
            }
        }
    }
}
