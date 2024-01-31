use std::{collections::VecDeque, fs};

struct Point {
    x: u16,
    y: u16,
    z: u16,
    t: u16,
}

fn split(x: u16) -> u64 {
    let mut x: u64 = x as u64;
    x = (x ^ (x << 24)) & 0x000000ff000000ff;
    x = (x ^ (x << 12)) & 0x000f000f000f000f;
    x = (x ^ (x << 6)) & 0x0303030303030303;
    x = (x ^ (x << 3)) & 0x1111111111111111;
    x
}

fn morton(p: &Point) -> u64 {
    (split(p.x) << 3) + (split(p.y) << 2) + (split(p.z) << 1) + split(p.t)
}

fn main() {
    // HEADS UP: This solution is completely unnecessary for the puzzle! I
    // just wanted to have some fun, optimizing the program. :-) There is a
    // much shorter version using a naive brute-force algorithm in the Git
    // history. However, if you're interested in a faster solution, this
    // algorithm here uses a combination of DBSCAN and a spatial index based
    // on Morton code to achieve a runtime complexity of O(n * log n). The
    // naive algorithm is more than fast enough for the puzzle input but the
    // runtime explodes really quickly if the number of points grows. The
    // algorithm here is much more scalable.

    const MAX_DISTANCE: u16 = 3;

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut min_z = i64::MAX;
    let mut min_t = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    let mut max_z = i64::MIN;
    let mut max_t = i64::MIN;
    let points = input
        .lines()
        .map(|l| {
            let p = l.trim().split(',').collect::<Vec<_>>();
            let coords = (
                p[1].parse::<i64>().unwrap(),
                p[0].parse::<i64>().unwrap(),
                p[2].parse::<i64>().unwrap(),
                p[3].parse::<i64>().unwrap(),
            );
            min_x = min_x.min(coords.0);
            min_y = min_y.min(coords.1);
            min_z = min_z.min(coords.2);
            min_t = min_z.min(coords.3);
            max_x = max_x.max(coords.0);
            max_y = max_y.max(coords.1);
            max_z = max_z.max(coords.2);
            max_t = max_z.max(coords.3);
            coords
        })
        .collect::<Vec<_>>();

    // Our morton code function can only handle u16. This should be more than enough.
    // It seems the puzzle input is always in the range (-8, -8, -8, -8) - (8, 8, 8, 8)
    assert!(max_x - min_x < u16::MAX as i64);
    assert!(max_y - min_y < u16::MAX as i64);
    assert!(max_z - min_z < u16::MAX as i64);
    assert!(max_t - min_t < u16::MAX as i64);

    // build spatial index
    let mut points = points
        .into_iter()
        .map(|coords| {
            let p = Point {
                x: (coords.0 - min_x) as u16,
                y: (coords.1 - min_y) as u16,
                z: (coords.2 - min_z) as u16,
                t: (coords.3 - min_t) as u16,
            };
            let index = morton(&p);
            (index, p)
        })
        .collect::<Vec<_>>();

    points.sort_by_key(|p| p.0);

    // DBSCAN: process point by point and search for neighbors
    let mut constellations = 0;
    let mut q = VecDeque::new();
    while let Some(p) = points.pop() {
        q.push_back(p);

        while let Some((_, pq)) = q.pop_front() {
            // perform spatial search: find minimum and maximum morton index
            let min = morton(&Point {
                x: pq.x.saturating_sub(MAX_DISTANCE),
                y: pq.y.saturating_sub(MAX_DISTANCE),
                z: pq.z.saturating_sub(MAX_DISTANCE),
                t: pq.t.saturating_sub(MAX_DISTANCE),
            });
            let max = morton(&Point {
                x: pq.x.saturating_add(MAX_DISTANCE),
                y: pq.y.saturating_add(MAX_DISTANCE),
                z: pq.z.saturating_add(MAX_DISTANCE),
                t: pq.t.saturating_add(MAX_DISTANCE),
            });

            // find the position of the first candidate
            let mut i = points.partition_point(|i| i.0 < min);

            // iterate through candidates from `min` to `max` and add all
            // points that actually match
            while i < points.len() && points[i].0 <= max {
                let d = ((pq.x as i32) - (points[i].1.x as i32)).abs()
                    + ((pq.y as i32) - (points[i].1.y as i32)).abs()
                    + ((pq.z as i32) - (points[i].1.z as i32)).abs()
                    + ((pq.t as i32) - (points[i].1.t as i32)).abs();
                if d <= MAX_DISTANCE as i32 {
                    // add matching point to queue to search for further neighbors
                    q.push_back(points.remove(i));
                } else {
                    i += 1;
                }
            }
        }

        constellations += 1;
    }

    println!("{}", constellations);
}
