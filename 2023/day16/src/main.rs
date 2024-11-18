use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn energize(start: (i32, i32, i32, i32), grid: &[Vec<char>]) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut seen = HashSet::new();
    let mut visited = HashSet::new();

    'outer: while let Some(mut b) = queue.pop_front() {
        visited.insert((b.0, b.1));

        // performance optimization shortcut: continue straight as far as possible
        while grid[b.1 as usize][b.0 as usize] == '.' {
            b.0 += b.2;
            b.1 += b.3;
            if b.0 < 0 || b.1 < 0 || b.0 == grid[0].len() as i32 || b.1 == grid.len() as i32 {
                continue 'outer;
            }
            visited.insert((b.0, b.1));
        }

        let (nb1, nb2) = match grid[b.1 as usize][b.0 as usize] {
            '.' => (Some(b), None),
            '/' => (Some((b.0, b.1, -b.3, -b.2)), None),
            '\\' => (Some((b.0, b.1, b.3, b.2)), None),
            '|' => {
                if b.2 != 0 {
                    (Some((b.0, b.1, 0, -1)), Some((b.0, b.1, 0, 1)))
                } else {
                    (Some(b), None)
                }
            }
            '-' => {
                if b.3 != 0 {
                    (Some((b.0, b.1, -1, 0)), Some((b.0, b.1, 1, 0)))
                } else {
                    (Some(b), None)
                }
            }
            _ => unreachable!(),
        };

        for mut nb in [nb1, nb2].into_iter().flatten() {
            nb.0 += nb.2;
            nb.1 += nb.3;
            if !seen.contains(&nb)
                && nb.0 >= 0
                && nb.1 >= 0
                && nb.0 < grid[0].len() as i32
                && nb.1 < grid.len() as i32
            {
                seen.insert(nb);
                queue.push_back(nb);
            }
        }
    }

    visited.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let total = energize((0, 0, 1, 0), &grid);
    println!("{}", total);

    let mut max = 0;
    for x in 0..grid[0].len() as i32 {
        max = max.max(energize((x, 0, 0, 1), &grid));
        max = max.max(energize((x, grid.len() as i32 - 1, 0, -1), &grid));
    }
    for y in 0..grid.len() as i32 {
        max = max.max(energize((0, y, 1, 0), &grid));
        max = max.max(energize((grid[0].len() as i32 - 1, y, -1, 0), &grid));
    }

    println!("{}", max);
}
