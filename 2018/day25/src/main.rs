use std::{fs, str::Bytes};

struct Point {
    x: u16,
    y: u16,
    z: u16,
    t: u16,
}

fn split(x: u16) -> u64 {
    let mut x = x as u64;
    x = (x ^ (x << 24)) & 0x000000ff000000ff;
    x = (x ^ (x << 12)) & 0x000f000f000f000f;
    x = (x ^ (x << 6)) & 0x0303030303030303;
    x = (x ^ (x << 3)) & 0x1111111111111111;
    x
}

fn morton(p: &Point) -> u64 {
    (split(p.x) << 3) + (split(p.y) << 2) + (split(p.z) << 1) + split(p.t)
}

/// This is faster than splitting the lines and then using parse()
fn parse_next_number(bytes: &mut Bytes) -> Option<i32> {
    let mut b = bytes.next()?;
    while b != b'-' && !b.is_ascii_digit() {
        b = bytes.next()?;
    }

    let negative = if b == b'-' {
        b = bytes.next()?;
        true
    } else {
        false
    };

    let mut r = 0;
    while b.is_ascii_digit() {
        r *= 10;
        r += (b - b'0') as i32;
        b = bytes.next()?;
    }

    Some(if negative { -r } else { r })
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
    let mut bytes = input.bytes();

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;
    let mut min_t = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_z = i32::MIN;
    let mut max_t = i32::MIN;
    let mut points = Vec::new();
    while let Some(x) = parse_next_number(&mut bytes) {
        let y = parse_next_number(&mut bytes).unwrap();
        let z = parse_next_number(&mut bytes).unwrap();
        let t = parse_next_number(&mut bytes).unwrap();
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        min_z = min_z.min(z);
        min_t = min_z.min(t);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        max_z = max_z.max(z);
        max_t = max_z.max(t);
        points.push((x, y, z, t));
    }

    // Our morton code function can only handle u16. This should be more than enough.
    // It seems the puzzle input is always in the range (-8, -8, -8, -8) - (8, 8, 8, 8)
    assert!(max_x - min_x < u16::MAX as i32);
    assert!(max_y - min_y < u16::MAX as i32);
    assert!(max_z - min_z < u16::MAX as i32);
    assert!(max_t - min_t < u16::MAX as i32);

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

    points.sort_unstable_by_key(|p| p.0);

    // DBSCAN: process point by point and search for neighbors
    let mut constellations = 0;
    let mut q = Vec::with_capacity(1000);
    while let Some(p) = points.pop() {
        q.push(p.1);

        while let Some(pq) = q.pop() {
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
                let d = pq.x.abs_diff(points[i].1.x)
                    + pq.y.abs_diff(points[i].1.y)
                    + pq.z.abs_diff(points[i].1.z)
                    + pq.t.abs_diff(points[i].1.t);
                if d <= MAX_DISTANCE {
                    // add matching point to queue to search for further neighbors
                    q.push(points.remove(i).1);
                } else {
                    i += 1;
                }
            }
        }

        constellations += 1;
    }

    println!("{constellations}");
}
