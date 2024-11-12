use std::fs;

fn find_start(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

fn find_loop(
    grid: &[Vec<char>],
    start: &(usize, usize),
    dir: (i8, i8),
) -> Option<Vec<(usize, usize)>> {
    let mut x = start.0;
    let mut y = start.1;
    let mut dir = dir;
    let mut l = vec![(x, y)];

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
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let s = find_start(&grid).expect("No start found");

    for dir in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
        if let Some(l) = find_loop(&grid, &s, dir) {
            // part 1
            println!("{}", l.len() / 2);

            // part 2 ...

            // use shoelace formula to calculate polygon area
            let mut sum = 0;
            for i in 0..l.len() - 1 {
                let y = l[i].0 + l[i + 1].0;
                let x = l[i].1 - l[i + 1].1;
                sum += y * x;
            }
            let area = ((sum / 2) as i32).abs();

            // use Pick's theorem to calculate number of interior points from
            // area and number of boundary points b
            let b = (l.len() - 1) as i32;
            let i = -b / 2 + 1 + area;
            println!("{}", i);

            break;
        }
    }
}
