use std::fs;

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn find_loop(
    grid: &[u8],
    width: usize,
    height: usize,
    mut pos: (i32, i32),
    mut route: Option<&mut Vec<usize>>,
    seen: &mut [u8],
    marked: &mut [bool],
) -> bool {
    // grid with all previous states
    seen.fill(0);

    if route.is_some() {
        // record previously visited positions so `route` will only
        // contain unique values
        marked.fill(false);
    }

    let mut dir = 0;
    loop {
        let i = pos.1 as usize * width + pos.0 as usize;

        // record guard's route
        if let Some(ref mut route) = route {
            if !marked[i] {
                route.push(i);
                marked[i] = true;
            }
        }

        let np = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);
        if np.0 < 0 || np.1 < 0 || np.0 >= width as i32 || np.1 >= height as i32 {
            // out of bounds
            return false;
        }

        if grid[np.1 as usize * width + np.0 as usize] == b'#' {
            // turn right
            dir = (dir + 1) % 4;

            // encode `dir` in a bit mask and check if guard has been in this
            // state before
            let mask = 1u8 << dir;
            if seen[i] & mask > 0 {
                // found loop
                return true;
            }
            seen[i] |= mask;
        } else {
            pos = np;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let mut grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    let w = lines[0].len();
    let h = lines.len();

    // find start position
    let mut start = (0, 0);
    for y in 0..h {
        for x in 0..w {
            if grid[y * w + x] == b'^' {
                start = (x as i32, y as i32);
                break;
            }
        }
    }

    let mut seen = vec![0u8; grid.len()];
    let mut marked = vec![false; grid.len()];

    // part 1
    let mut route = Vec::new();
    find_loop(&grid, w, h, start, Some(&mut route), &mut seen, &mut marked);
    println!("{}", route.len());

    // part 2
    let mut loops = 0;
    for &i in &route[1..] {
        grid[i] = b'#';
        if find_loop(&grid, w, h, start, None, &mut seen, &mut marked) {
            loops += 1;
        }
        grid[i] = b'.';
    }
    println!("{}", loops);
}
