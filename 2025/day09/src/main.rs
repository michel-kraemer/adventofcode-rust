use std::{collections::HashMap, fs};

type Point = (i64, i64);

fn is_inside(
    p: Point,
    edges_by_y: &HashMap<i64, Vec<(Point, Point)>>,
    vertical_edges_by_x: &HashMap<i64, Vec<(Point, Point)>>,
) -> bool {
    let x = p.0;
    let mut y = p.1;

    let max_y = 100000;

    let mut left_edges = 0;
    let mut right_edges = 0;
    let mut insides = 0;

    if edges_by_y
        .get(&y)
        .and_then(|es| {
            es.iter().find(|e| {
                let min = e.0.0.min(e.1.0);
                let max = e.0.0.max(e.1.0);
                (min..=max).contains(&x)
            })
        })
        .is_some()
    {
        return true;
    }

    if vertical_edges_by_x
        .get(&x)
        .and_then(|es| {
            es.iter().find(|e| {
                let min = e.0.1.min(e.1.1);
                let max = e.0.1.max(e.1.1);
                (min..=max).contains(&y)
            })
        })
        .is_some()
    {
        return true;
    }

    while y < max_y {
        y += 1;
        let e = edges_by_y.get(&y).and_then(|es| {
            es.iter().find(|e| {
                let min = e.0.0.min(e.1.0);
                let max = e.0.0.max(e.1.0);
                (min..=max).contains(&x)
            })
        });
        if let Some(e) = e {
            if x == e.0.0 && y == e.0.1 {
                // hit a corner
                if e.1.0 > x {
                    right_edges += 1;
                } else {
                    left_edges += 1;
                }
                if right_edges == left_edges {
                    insides += 1;
                }
            } else if x == e.1.0 && y == e.1.1 {
                // hit a corner
                if e.0.0 > x {
                    right_edges += 1;
                } else {
                    left_edges += 1;
                }
                if right_edges == left_edges {
                    insides += 1;
                }
            } else {
                // hit the inside of the edge
                // is_inside = !is_inside;
                insides += 1;
            }
        }
    }

    insides % 2 != 0
}

fn intersect_edges(vert: &(Point, Point), horiz: &(Point, Point)) -> bool {
    let vmin = vert.0.1.min(vert.1.1);
    let vmax = vert.0.1.max(vert.1.1);
    let hmin = horiz.0.0.min(horiz.1.0);
    let hmax = horiz.0.0.max(horiz.1.0);
    if vert.0.0 <= hmin || vert.0.0 >= hmax {
        return false;
    }
    vmin < horiz.0.1 && vmax > horiz.0.1
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut coords = Vec::new();
    for l in input.lines() {
        let (x, y) = l.split_once(',').unwrap();
        let x = x.parse::<i64>().unwrap();
        let y = y.parse::<i64>().unwrap();
        coords.push((x, y));
    }

    let mut max = 0;
    for a in 0..coords.len() {
        for b in a + 1..coords.len() {
            let area =
                (coords[a].0.abs_diff(coords[b].0) + 1) * (coords[a].1.abs_diff(coords[b].1) + 1);
            if area > max {
                max = max.max(area);
            }
        }
    }
    println!("{max}");

    let mut edges = Vec::new();
    let mut vertical_edges = Vec::new();
    let mut vertical_edges_by_x: HashMap<i64, Vec<(Point, Point)>> = HashMap::new();
    let mut edges_by_y: HashMap<i64, Vec<(Point, Point)>> = HashMap::new();
    for i in 0..coords.len() {
        let p1 = coords[i];
        let p2 = coords[(i + 1) % coords.len()];
        if p1.1 == p2.1 {
            edges.push((p1, p2));
            edges_by_y.entry(p1.1).or_default().push((p1, p2));
        } else {
            vertical_edges.push((p1, p2));
            vertical_edges_by_x.entry(p1.0).or_default().push((p1, p2));
        }
    }

    let mut max = 0;
    for a in 0..coords.len() {
        for b in a + 2..coords.len() {
            let left = coords[a].0.min(coords[b].0);
            let right = coords[a].0.max(coords[b].0);
            let top = coords[a].1.min(coords[b].1);
            let bottom = coords[a].1.max(coords[b].1);

            let tl = (left, top);
            let bl = (left, bottom);
            let tr = (right, top);
            let br = (right, bottom);

            let mut good = true;
            for c in &coords {
                if c.0 > left && c.0 < right && c.1 > top && c.1 < bottom {
                    good = false;
                    break;
                }
            }
            if !good {
                continue;
            }

            if !is_inside(tl, &edges_by_y, &vertical_edges_by_x)
                || !is_inside(bl, &edges_by_y, &vertical_edges_by_x)
                || !is_inside(tr, &edges_by_y, &vertical_edges_by_x)
                || !is_inside(br, &edges_by_y, &vertical_edges_by_x)
            {
                continue;
            }

            for v in &vertical_edges {
                if intersect_edges(v, &(tl, tr)) {
                    good = false;
                    break;
                }
            }
            if !good {
                continue;
            }
            for v in &vertical_edges {
                if intersect_edges(v, &(bl, br)) {
                    good = false;
                    break;
                }
            }
            if !good {
                continue;
            }

            for v in &edges {
                if intersect_edges(&(tl, bl), v) {
                    good = false;
                    break;
                }
            }
            if !good {
                continue;
            }
            for v in &edges {
                if intersect_edges(&(tr, br), v) {
                    good = false;
                    break;
                }
            }
            if !good {
                continue;
            }

            let area =
                (coords[a].0.abs_diff(coords[b].0) + 1) * (coords[a].1.abs_diff(coords[b].1) + 1);
            if area > max {
                max = max.max(area);
            }
        }
    }
    println!("{max}");
}
