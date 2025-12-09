use std::fs;

/// A 2D point
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u64,
    y: u64,
}

/// An horizontal or vertical edge consisting of 2 points, sorted by x or y
#[derive(Clone, Copy)]
struct Edge {
    min: Point,
    max: Point,
}

impl Edge {
    fn new(a: Point, b: Point) -> Self {
        if a < b {
            Self { min: a, max: b }
        } else {
            Self { min: b, max: a }
        }
    }
}

fn area(a: Point, b: Point) -> u64 {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

/// Check if a point `p` lies inside the polygon made up of the horizontal edges
/// `hedges` and the vertical edges `vedges`
fn is_inside(p: Point, hedges: &[Edge], vedges: &[Edge]) -> bool {
    let mut left_edges = 0;
    let mut right_edges = 0;
    let mut crossed_edges = 0;

    let mut i = hedges.partition_point(|e| e.min.y < p.y);
    while i < hedges.len() && hedges[i].min.y == p.y {
        if (hedges[i].min.x..=hedges[i].max.x).contains(&p.x) {
            // we've hit a horizontal edge, so we're inside the polygon
            return true;
        }
        i += 1;
    }
    if i == hedges.len() {
        return false;
    }

    let mut j = vedges.partition_point(|e| e.min.x < p.x);
    while j < vedges.len() && vedges[j].min.x == p.x {
        if (vedges[j].min.y..=vedges[j].max.y).contains(&p.y) {
            // we've hit a horizontal edge, so we're inside the polygon
            return true;
        }
        j += 1;
    }
    if j == vedges.len() {
        return false;
    }

    for e in hedges.iter().skip(i) {
        if (e.min.x..=e.max.x).contains(&p.x) {
            if p.x == e.min.x {
                // hit a corner
                if e.max.x > p.x {
                    right_edges += 1;
                } else {
                    left_edges += 1;
                }
                if right_edges == left_edges {
                    // We've crossed as many right-pointing as left-pointing
                    // edges. Increase the total number of edges crossed.
                    crossed_edges += 1;
                }
            } else if p.x == e.max.x {
                // hit a corner
                if e.min.x > p.x {
                    right_edges += 1;
                } else {
                    left_edges += 1;
                }
                if right_edges == left_edges {
                    // We've crossed as many right-pointing as left-pointing
                    // edges. Increase the total number of edges crossed.
                    crossed_edges += 1;
                }
            } else {
                // hit the inside of the edge
                crossed_edges += 1;
            }
        }
        i += 1;
    }

    // we're inside the polygon if we've crossed an odd number of edges
    crossed_edges % 2 != 0
}

/// Check if two edges cross each other, i.e. there is an intersection point but
/// this point is not a start or end point of one of the edges
fn cross_edges(vert: Edge, horiz: Edge) -> bool {
    vert.min.x > horiz.min.x
        && vert.min.x < horiz.max.x
        && vert.min.y < horiz.min.y
        && vert.max.y > horiz.min.y
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // parse coordinates
    let mut coords = Vec::new();
    for l in input.lines() {
        let (x, y) = l.split_once(',').unwrap();
        let x = x.parse::<u64>().unwrap();
        let y = y.parse::<u64>().unwrap();
        coords.push(Point { x, y });
    }

    // part 1: calculate maximum area
    let mut max = 0;
    for a in 0..coords.len() {
        for b in a + 1..coords.len() {
            max = max.max(area(coords[a], coords[b]));
        }
    }
    println!("{max}");

    // part 2 ...
    // get a list of all horizontal and vertical edges
    let mut hedges = Vec::new();
    let mut vedges = Vec::new();
    for i in 0..coords.len() {
        let p1 = coords[i];
        let p2 = coords[(i + 1) % coords.len()];
        if p1.y == p2.y {
            hedges.push(Edge::new(p1, p2));
        } else {
            vedges.push(Edge::new(p1, p2));
        }
    }
    hedges.sort_by_key(|e| e.min.y);
    vedges.sort_by_key(|e| e.min.x);

    let mut max = 0;
    for a in 0..coords.len() {
        for b in a + 1..coords.len() {
            // construct rectangle
            let left = coords[a].x.min(coords[b].x);
            let right = coords[a].x.max(coords[b].x);
            let top = coords[a].y.min(coords[b].y);
            let bottom = coords[a].y.max(coords[b].y);

            let top_left = Point { x: left, y: top };
            let bottom_left = Point { x: left, y: bottom };
            let top_right = Point { x: right, y: top };
            let bottom_right = Point {
                x: right,
                y: bottom,
            };

            // check if any of the polygon points lies inside the rectangle
            if coords
                .iter()
                .any(|c| c.x > left && c.x < right && c.y > top && c.y < bottom)
            {
                continue;
            }

            // check if any of the rectangle's corners lies outside the polygon
            if !is_inside(top_left, &hedges, &vedges)
                || !is_inside(bottom_left, &hedges, &vedges)
                || !is_inside(top_right, &hedges, &vedges)
                || !is_inside(bottom_right, &hedges, &vedges)
            {
                continue;
            }

            // check if any of the rectangle edges crosses any of the polygon edges
            if vedges.iter().any(|&v| {
                cross_edges(v, Edge::new(top_left, top_right))
                    || cross_edges(v, Edge::new(bottom_left, bottom_right))
            }) {
                continue;
            }
            if hedges.iter().any(|&h| {
                cross_edges(Edge::new(top_left, bottom_left), h)
                    || cross_edges(Edge::new(top_right, bottom_right), h)
            }) {
                continue;
            }

            max = max.max(area(coords[a], coords[b]));
        }
    }
    println!("{max}");
}
