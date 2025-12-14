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
    let mut new = 0;
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
fn fill(pos: (usize, usize), grid: &mut [bool], width: usize, height: usize) {
    if grid[pos.1 * width + pos.0] {
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
                && !grid[ny as usize * width + nx as usize]
            {
                grid[ny as usize * width + nx as usize] = true;
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

    let mut grid = vec![false; width * height];
    for c in coords.windows(2) {
        let a = c[0].1;
        let b = c[1].1;
        if a.0 == b.0 {
            // vertical edge
            for y in a.1.min(b.1)..=a.1.max(b.1) {
                grid[y * width + a.0] = true;
            }
        } else {
            // horizontal edge
            let minx = a.0.min(b.0);
            let maxx = a.0.max(b.0);
            grid[(a.1 * width + minx)..=(a.1 * width + maxx)].fill(true);
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
                let mut ok = true;
                let minx = a.1.0.min(b.1.0);
                let maxx = a.1.0.max(b.1.0);
                let miny = a.1.1.min(b.1.1);
                let maxy = a.1.1.max(b.1.1);

                for y in miny..=maxy {
                    // performance optimization: check vertical edges first
                    if !grid[y * width + minx] || !grid[y * width + maxx] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    for y in miny..=maxy {
                        if grid
                            .iter()
                            .skip(y * width + minx)
                            .take(maxx - minx + 1)
                            .any(|b| !*b)
                        {
                            ok = false;
                            break;
                        }
                    }
                }

                if ok {
                    max2 = max2.max(area);
                }
            }
        }
    }

    println!("{max1}");
    println!("{max2}");
}
