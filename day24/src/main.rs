use std::fs;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

fn intersection2d(from1: &Point, to1: &Point, from2: &Point, to2: &Point) -> Option<Point> {
    let dx = to1.x - from1.x;
    let dy = to1.y - from1.y;

    let determinant = dx * (to2.y - from2.y) - (to2.x - from2.x) * dy;
    if determinant == 0. {
        return None;
    }

    let lambda = ((to2.y - from2.y) * (to2.x - from1.x) + (from2.x - to2.x) * (to2.y - from1.y))
        / determinant;
    let gamma = ((from1.y - to1.y) * (to2.x - from1.x) + dx * (to2.y - from1.y)) / determinant;

    // if !(0. <= lambda && lambda <= 1.) || !(0. <= gamma && gamma <= 1.) { return None; }

    if lambda <= 0. || gamma > 1. {
        return None;
    }

    return Some(Point {
        x: from1.x + lambda * dx,
        y: from1.y + lambda * dy,
        z: 0.,
    });
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let hailstones = input
        .lines()
        .map(|l| l.split_once(" @ ").unwrap())
        .map(|p| {
            let p0 = p.0.split(", ").collect::<Vec<_>>();
            let p1 = p.1.split(", ").collect::<Vec<_>>();
            (
                Point {
                    x: p0[0].trim().parse::<f64>().unwrap(),
                    y: p0[1].trim().parse::<f64>().unwrap(),
                    z: p0[2].trim().parse::<f64>().unwrap(),
                },
                Point {
                    x: p1[0].trim().parse::<f64>().unwrap(),
                    y: p1[1].trim().parse::<f64>().unwrap(),
                    z: p1[2].trim().parse::<f64>().unwrap(),
                },
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..hailstones.len() {
        let h1 = hailstones[i];
        for j in i + 1..hailstones.len() {
            let h2 = hailstones[j];

            let p1from = &h1.0;
            let p1to = Point {
                x: p1from.x + h1.1.x,
                y: p1from.y + h1.1.y,
                z: p1from.z + h1.1.z,
            };

            let p2from = &h2.0;
            let p2to = Point {
                x: p2from.x + h2.1.x,
                y: p2from.y + h2.1.y,
                z: p2from.z + h2.1.z,
            };

            if let Some(i) = intersection2d(&p1from, &p1to, &p2from, &p2to) {
                if i.x >= 200000000000000.
                    && i.x <= 400000000000000.
                    && i.y >= 200000000000000.
                    && i.y <= 400000000000000.
                {
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);
}
