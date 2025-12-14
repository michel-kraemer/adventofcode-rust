//! Optimized solution using coordinate compression and prefix sums. Works for
//! all test cases I could find on Reddit:
//! * https://www.reddit.com/r/adventofcode/comments/1pi5rqn/2025_day_9_part_2_check_your_solution_with_this/
//! * https://www.reddit.com/r/adventofcode/comments/1piet8x/2025_day_9_part_2_a_simple_cursed_input/
//! * https://www.reddit.com/r/adventofcode/comments/1pi36pq/2025_day_9_part_2_more_examples_to_soften_your/
//! * https://www.reddit.com/r/adventofcode/comments/1pit2un/2029_day_9_part_2_i_solved_this_one_but_my_code/
//! * https://www.reddit.com/r/adventofcode/comments/1piqgc2/2025_day_9_check_your_code_with_this_test_input/

use std::{
    collections::{HashMap, VecDeque},
    fs,
};

// Right, Down, Left, Up
const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// Check if a polygon is oriented clockwise
fn is_clockwise(coords: &[(usize, usize)]) -> bool {
    coords
        .windows(2)
        .map(|c| (c[1].0 as i64 - c[0].0 as i64) * (c[1].1 as i64 + c[0].1 as i64))
        .sum::<i64>()
        < 0
}

/// Compress the given coordinates. Make sure there is space between the
/// compressed values if necessary
fn compress(mut orig: Vec<usize>) -> HashMap<usize, usize> {
    orig.sort_unstable();
    orig.dedup();

    let mut result = HashMap::new();
    let mut new = 1; // starting at 1 makes it easier to calculate prefix sums later
    let mut last_orig = orig[0];
    for v in orig {
        if v - last_orig > 1 {
            // only leave space if there was space in the original coordinates too
            new += 1;
        }
        result.insert(v, new);
        last_orig = v;
        new += 1;
    }

    result
}

/// Translate an original coordinate to a compressed one
fn get_compressed(
    c: (usize, usize),
    compressed_x: &HashMap<usize, usize>,
    compressed_y: &HashMap<usize, usize>,
) -> (usize, usize) {
    (
        *compressed_x.get(&c.0).unwrap(),
        *compressed_y.get(&c.1).unwrap(),
    )
}

/// Perform a flood fill at the given position
fn fill(pos: (usize, usize), grid: &mut [Vec<u64>], width: usize, height: usize) {
    if grid[pos.1][pos.0] > 0 {
        // nothing to do
        return;
    }

    let mut queue = VecDeque::new();
    queue.push_back(pos);
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in DIRS {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if nx >= 0
                && (nx as usize) < width
                && ny >= 0
                && (ny as usize) < height
                && grid[ny as usize][nx as usize] == 0
            {
                grid[ny as usize][nx as usize] = 1;
                queue.push_back((nx as usize, ny as usize));
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // parse coordinates
    let mut coords = Vec::new();
    for l in input.lines() {
        let (x, y) = l.split_once(',').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        coords.push((x, y));
    }

    // close the polygon - make the code a bit simpler
    coords.push(coords[0]);

    // make sure the polygon is oriented clockwise
    if !is_clockwise(&coords) {
        coords.reverse();
    }

    // compress x and y coordinates
    let compressed_x = compress(coords.iter().map(|c| c.0).collect());
    let compressed_y = compress(coords.iter().map(|c| c.1).collect());

    // convert coordinates to list of tuples of original and compressed
    // coordinates
    let coords = coords
        .into_iter()
        .map(|c| (c, get_compressed(c, &compressed_x, &compressed_y)))
        .collect::<Vec<_>>();

    // draw compressed polygon into a grid
    let width = compressed_x.values().max().unwrap() + 1;
    let height = compressed_y.values().max().unwrap() + 1;

    let mut grid = vec![vec![0; width]; height];
    for c in coords.windows(2) {
        let a = c[0].1;
        let b = c[1].1;
        if a.0 == b.0 {
            // vertical edge
            for cell in grid.iter_mut().take(a.1.max(b.1) + 1).skip(a.1.min(b.1)) {
                cell[a.0] = 1;
            }
        } else {
            // horizontal edge
            let minx = a.0.min(b.0);
            let maxx = a.0.max(b.0);
            grid[a.1][minx..=maxx].fill(1);
        }
    }

    // flood-fill everything inside the polygon
    for c in coords.windows(2) {
        let a = c[0].1;
        let b = c[1].1;
        if a.0 == b.0 {
            // vertical edge
            if a.1 < b.1 {
                // down
                fill((a.0 - 1, a.1 + 1), &mut grid, width, height);
            } else {
                // up
                fill((a.0 + 1, a.1 - 1), &mut grid, width, height);
            }
        } else {
            // horizontal edge
            if a.0 < b.0 {
                // right
                fill((a.0 + 1, a.1 + 1), &mut grid, width, height);
            } else {
                // left
                fill((a.0 - 1, a.1 - 1), &mut grid, width, height);
            }
        }
    }

    // Compute prefix sums. At a given position (x,y), this gives us the
    // number of occupied cells in the rectangle [0,0,x,y].
    for y in 1..height {
        for x in 1..width {
            grid[y][x] += grid[y][x - 1] + grid[y - 1][x] - grid[y - 1][x - 1];
        }
    }

    // iterate over all possible rectangles
    let mut max1 = 0;
    let mut max2 = 0;
    for (i, a) in coords.iter().enumerate() {
        for b in coords.iter().skip(i + 1) {
            // compute area using original coordinates
            let area = (a.0.0.abs_diff(b.0.0) + 1) * (a.0.1.abs_diff(b.0.1) + 1);

            // part 1: compute the maximum area of all rectangles
            max1 = max1.max(area);

            // part 2: check if the rectangle is completely inside the polygon
            // (only do this if necessary, i.e. if the area is larger than the
            // largest one already found)
            if area > max2 {
                let minx = a.1.0.min(b.1.0);
                let maxx = a.1.0.max(b.1.0);
                let miny = a.1.1.min(b.1.1);
                let maxy = a.1.1.max(b.1.1);

                // calculate how many occupied cells we expect
                let expected_area = ((maxx - minx + 1) * (maxy - miny + 1)) as u64;

                // get the number of actually occupied cells
                let prefix_area = grid[maxy][maxx] - grid[miny - 1][maxx] - grid[maxy][minx - 1]
                    + grid[miny - 1][minx - 1];

                if prefix_area == expected_area {
                    // the rectangle is completely inside the polygon
                    max2 = max2.max(area);
                }
            }
        }
    }

    println!("{max1}");
    println!("{max2}");
}
