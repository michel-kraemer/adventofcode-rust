use std::fs;

/// A table of distances between grid cells and input coordinates, calculated on
/// demand
struct Distances<'a> {
    /// The input coordinates
    coords: &'a [(i64, i64)],

    /// The minimum x-coordinate
    min_x: i64,

    /// The minimum y-coordinate
    min_y: i64,

    /// The width of the total area
    width: i64,

    /// A cache for already calculated minimum distances
    min_cache: Vec<Option<Option<usize>>>,
}

impl<'a> Distances<'a> {
    /// Create a new distances table
    fn new(min_x: i64, min_y: i64, max_x: i64, max_y: i64, coords: &'a [(i64, i64)]) -> Self {
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let min_cache = vec![None; (width * height) as usize];
        Self {
            coords,
            min_x,
            min_y,
            width,
            min_cache,
        }
    }

    /// Get the index of the coordinate with the minimum distance to the given
    /// grid cell
    fn get_min(&mut self, x: i64, y: i64) -> Option<usize> {
        let idx = ((y - self.min_y) * self.width + x - self.min_x) as usize;
        if let Some(c) = self.min_cache[idx] {
            return c;
        }

        let mut min = u64::MAX;
        let mut min_index = None;
        for (i, c) in self.coords.iter().enumerate() {
            let dist = c.0.abs_diff(x) + c.1.abs_diff(y);
            if dist < min {
                min = dist;
                min_index = Some(i);
            } else if dist == min {
                min_index = None;
            }
        }

        self.min_cache[idx] = Some(min_index);

        min_index
    }
}

/// Get the sum of all distances between the given grid cell and coordinates
fn get_sum(x: i64, y: i64, coords: &[(i64, i64)]) -> u64 {
    coords
        .iter()
        .map(|c| c.0.abs_diff(x) + c.1.abs_diff(y))
        .sum()
}

/// Scan a row `y` from `min_x` to `min_y` and collect the left and right edges
/// of found areas
fn scan(min_x: i64, max_x: i64, y: i64, edges: &mut Vec<(i64, usize)>, distances: &mut Distances) {
    let mut curr = None;
    for x in min_x..max_x {
        if let Some(min_index) = distances.get_min(x, y) {
            if let Some(c) = curr {
                if c != min_index {
                    edges.push((x - 1, c));
                    curr = Some(min_index);
                    edges.push((x, min_index));
                }
            } else {
                curr = Some(min_index);
                edges.push((x, min_index));
            }
        } else if let Some(c) = curr {
            edges.push((x - 1, c));
            curr = None;
        }
    }
    if let Some(c) = curr {
        edges.push((max_x - 1, c));
    }
}

/// Update the sizes of the given areas based on the given left and right edges
fn update_area_sizes(edges: &[(i64, usize)], areas: &mut [i64]) {
    for w in edges.chunks_exact(2) {
        areas[w[0].1] += w[1].0 - w[0].0 + 1;
    }
}

/// Find an approximate grid cell with a minimum distance sum. This is likely
/// the geometric mean of all coordinates (or at least it should be very close
/// to it).
fn find_minimum(
    coords: &[(i64, i64)],
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
) -> (i64, i64) {
    const DELTA: i64 = 10;

    // find centroid as a starting point
    let sum_x = coords.iter().map(|c| c.0).sum::<i64>();
    let sum_y = coords.iter().map(|c| c.1).sum::<i64>();
    let mut cx = sum_x / coords.len() as i64;
    let mut cy = sum_y / coords.len() as i64;

    // perform binary search in x-direction
    let mut lo_x = min_x;
    let mut hi_x = max_x;
    let mut min = u64::MAX;
    while lo_x < hi_x {
        let s1 = get_sum(cx - DELTA, cy, coords);
        let sc = get_sum(cx, cy, coords);
        let s2 = get_sum(cx + DELTA, cy, coords);
        if sc <= s1 && sc <= s2 {
            for x in cx - DELTA..=cx + DELTA {
                let s = get_sum(x, cy, coords);
                if s < min {
                    min = s;
                    cx = x;
                } else {
                    break;
                }
            }
            break;
        } else if s1 < s2 {
            hi_x = cx;
            cx = (lo_x + hi_x) / 2;
        } else {
            lo_x = cx + 1;
            cx = (lo_x + hi_x) / 2;
        }
    }

    // perform binary search in y-direction
    let mut lo_y = min_y;
    let mut hi_y = max_y;
    min = u64::MAX;
    while lo_y < hi_y {
        let s1 = get_sum(cx, cy - DELTA, coords);
        let sc = get_sum(cx, cy, coords);
        let s2 = get_sum(cx, cy + DELTA, coords);
        if sc <= s1 && sc <= s2 {
            for y in cy - DELTA..=cy + DELTA {
                let s = get_sum(cx, y, coords);
                if s < min {
                    min = s;
                    cy = y;
                } else {
                    break;
                }
            }
            break;
        } else if s1 < s2 {
            hi_y = cy;
            cy = (lo_y + hi_y) / 2;
        } else {
            lo_y = cy + 1;
            cy = (lo_y + hi_y) / 2;
        }
    }

    (cx, cy)
}

fn main() {
    // This solution is convoluted, but fast ;-)

    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    let coords = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(", ").unwrap();
            let c = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
            min_x = min_x.min(c.0);
            min_y = min_y.min(c.1);
            max_x = max_x.max(c.0 + 1);
            max_y = max_y.max(c.1 + 1);
            c
        })
        .collect::<Vec<_>>();

    // part 1 ...

    // area sizes
    let mut areas = vec![0; coords.len()];

    // whether areas are infinite or not
    let mut infinite = vec![false; coords.len()];

    // minimum distances per grid cell (calculated on demand)
    let mut distances = Distances::new(min_x, min_y, max_x, max_y, &coords);

    // scan first row to find left and right edges of areas
    let mut edges = Vec::new();
    scan(min_x, max_x, min_y, &mut edges, &mut distances);
    update_area_sizes(&edges, &mut areas);

    // all areas in the first row are infinite
    for &(_, i) in &edges {
        infinite[i] = true;
    }

    // scan all other rows and trace contours of areas
    for y in min_y + 1..max_y {
        let mut prev_right = min_x;
        let mut new_edges = Vec::new();
        for w in edges.chunks_exact(2) {
            let id = w[0].1;
            let old_left = w[0].0;
            let old_right = w[1].0;

            let min_left = distances.get_min(old_left, y);
            let new_left = match min_left {
                Some(min_left) if min_left == id => {
                    // still the same area - check if the edge has moved left
                    let mut x = old_left;
                    while x > min_x
                        && let Some(m2) = distances.get_min(x - 1, y)
                        && m2 == id
                    {
                        x -= 1;
                    }
                    x
                }

                _ => {
                    // another area or no man's land - check if the edge has moved right
                    let mut x = old_left + 1;
                    while x <= old_right && distances.get_min(x, y) != Some(id) {
                        x += 1;
                    }
                    if x == old_right + 1 {
                        // we've reached the end of the area - continue with next one
                        continue;
                    }
                    x
                }
            };

            let min_right = distances.get_min(old_right, y);
            let new_right = match min_right {
                Some(min_right) if min_right == id => {
                    // still the same area - check if the edge has moved right
                    let mut x = old_right;
                    while x < max_x - 1
                        && let Some(m2) = distances.get_min(x + 1, y)
                        && m2 == id
                    {
                        x += 1;
                    }
                    x
                }

                _ => {
                    // another area or no man's land - check if the edge has moved left
                    let mut x = old_right - 1;
                    while x >= old_left && distances.get_min(x, y) != Some(id) {
                        x -= 1;
                    }

                    // should have already been caught when tracing the left edge
                    assert!(
                        x != old_left - 1,
                        "Unexpected end of area {} at ({x},{y})",
                        id
                    );

                    x
                }
            };

            if new_left > prev_right + 1 {
                // there is a gap between the previous area's right edge and the
                // current area's left edge - look for new areas in between
                scan(prev_right, new_left, y, &mut new_edges, &mut distances);
            }

            prev_right = new_right + 1;

            new_edges.push((new_left, id));
            new_edges.push((new_right, id));
        }

        if prev_right < max_x {
            // there is a gap between the previous area's right edge and max_x
            // look for new areas
            scan(prev_right, max_x, y, &mut new_edges, &mut distances);
        }

        edges = new_edges;
        update_area_sizes(&edges, &mut areas);

        // all areas at the borders are infinite
        if edges[0].0 == min_x {
            infinite[edges[0].1] = true;
        }
        if edges[edges.len() - 1].0 == max_x - 1 {
            infinite[edges[edges.len() - 1].1] = true;
        }
    }

    // all areas in the last row are infinite
    for &(_, i) in &edges {
        infinite[i] = true;
    }

    // find the maximum size of all areas that are not infinite
    println!(
        "{}",
        areas
            .iter()
            .enumerate()
            .filter(|(i, _)| !infinite[*i])
            .map(|(_, a)| a)
            .max()
            .unwrap()
    );

    // part 2 ...

    // find a grid cell with a minimum distance sum
    let (mut left, mut y) = find_minimum(&coords, min_x, min_y, max_x, max_y);

    // find the upper edge of the safe region (using binary search)
    let mut lo_y = min_y;
    let mut hi_y = y;
    while lo_y < hi_y {
        let mid = (lo_y + hi_y) / 2;
        if get_sum(left, mid, &coords) < 10000 {
            hi_y = mid;
        } else {
            lo_y = mid + 1;
        }
    }
    y = lo_y;

    // find the left and right edges of the safe region - move the upper edge
    // further up if necessary
    let mut right;
    'outer: loop {
        right = left;
        while left > min_x && get_sum(left - 1, y, &coords) < 10000 {
            left -= 1;
            if y > min_y && get_sum(left, y - 1, &coords) < 10000 {
                y -= 1;
                continue 'outer;
            }
        }
        while right < max_x - 1 && get_sum(right + 1, y, &coords) < 10000 {
            right += 1;
            if y > min_y && get_sum(right, y - 1, &coords) < 10000 {
                left = right;
                y -= 1;
                continue 'outer;
            }
        }
        break;
    }

    // trace the safe region and sum up its size - similar to how we traced the
    // areas in part 1
    let mut total2 = right - left + 1;
    y += 1;
    while y < max_y {
        let left_sum = get_sum(left, y, &coords);
        if left_sum < 10000 {
            // we're still in the area - check if the edge has moved left
            while left > min_x && get_sum(left - 1, y, &coords) < 10000 {
                left -= 1;
            }
        } else {
            // we're outside - check if the edge has moved right
            left += 1;
            while left <= right && get_sum(left, y, &coords) >= 10000 {
                left += 1;
            }
        }
        if left > right {
            break;
        }

        let right_sum = get_sum(right, y, &coords);
        if right_sum < 10000 {
            // we're still in the area - check if the edge has moved right
            while right < max_x - 1 && get_sum(right + 1, y, &coords) < 10000 {
                right += 1;
            }
        } else {
            // we're outside - check if the edge has moved left
            right -= 1;
            while right >= left && get_sum(right, y, &coords) >= 10000 {
                right -= 1;
            }
        }

        total2 += right - left + 1;

        y += 1;
    }

    println!("{total2}");
}
