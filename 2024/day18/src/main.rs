use std::collections::VecDeque;
use std::fs;

fn bfs(grid: &mut [bool], max_bytes: usize, bytes: &[(i32, i32)]) -> Option<usize> {
    grid.fill(false);

    for i in 0..max_bytes {
        grid[bytes[i].1 as usize * 71 + bytes[i].0 as usize] = true;
    }

    let pos = (0, 0, 0);
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    grid[0] = true;

    while let Some((x, y, steps)) = queue.pop_front() {
        if x == 70 && y == 70 {
            return Some(steps);
        }
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;
            let si = ny as usize * 71 + nx as usize;
            if nx >= 0 && ny >= 0 && nx < 71 && ny < 71 && !grid[si] {
                grid[si] = true;
                queue.push_back((nx, ny, steps + 1));
            }
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let bytes = input
        .lines()
        .map(|l| {
            l.split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect::<Vec<(i32, i32)>>();

    let mut grid = vec![false; 71 * 71];

    // part 1
    let total1 = bfs(&mut grid, 1024, &bytes).unwrap();
    println!("{}", total1);

    // part 2
    let mut min = 1025;
    let mut max = bytes.len();
    while min != max {
        let mid = (min + max) / 2;
        if bfs(&mut grid, mid, &bytes).is_some() {
            min = mid + 1;
        } else {
            max = mid;
        }
    }

    println!("{},{}", bytes[max - 1].0, bytes[max - 1].1);
}
