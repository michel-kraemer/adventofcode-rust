use std::fs;

use z3::{
    ast::{self, Ast},
    Config, Context, SatResult, Solver,
};

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
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let hailstones = input
            .lines()
            .map(|l| l.split_once(" @ ").unwrap())
            .map(|p| {
                let p0: Vec<f64> = p.0.split(", ").map(|v| v.trim().parse().unwrap()).collect();
                let p1: Vec<f64> = p.1.split(", ").map(|v| v.trim().parse().unwrap()).collect();
                (
                    Point {
                        x: p0[0],
                        y: p0[1],
                        z: p0[2],
                    },
                    Point {
                        x: p1[0],
                        y: p1[1],
                        z: p1[2],
                    },
                )
            })
            .collect::<Vec<_>>();

        if part1 {
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
        } else {
            let cfg = Config::new();
            let ctx = Context::new(&cfg);
            let solver = Solver::new(&ctx);

            let rxs = ast::Int::new_const(&ctx, "rxs");
            let rys = ast::Int::new_const(&ctx, "rys");
            let rzs = ast::Int::new_const(&ctx, "rzs");
            let rxv = ast::Int::new_const(&ctx, "rxv");
            let ryv = ast::Int::new_const(&ctx, "ryv");
            let rzv = ast::Int::new_const(&ctx, "rzv");
            let zero = ast::Int::from_i64(&ctx, 0);

            // (rxs - hxs) * (hyv - ryv) - (rys - hys) * (hxv - rxv) = 0
            // (rzs - hzs) * (hyv - ryv) - (rys - hys) * (hzv - rzv) = 0
            for h in hailstones {
                let hxs = ast::Int::from_i64(&ctx, h.0.x as i64);
                let hys = ast::Int::from_i64(&ctx, h.0.y as i64);
                let hzs = ast::Int::from_i64(&ctx, h.0.z as i64);
                let hxv = ast::Int::from_i64(&ctx, h.1.x as i64);
                let hyv = ast::Int::from_i64(&ctx, h.1.y as i64);
                let hzv = ast::Int::from_i64(&ctx, h.1.z as i64);
                let eq1 =
                    ((&rxs - &hxs) * (&hyv - &ryv) - (&rys - &hys) * (&hxv - &rxv))._eq(&zero);
                let eq2 =
                    ((&rzs - &hzs) * (&hyv - &ryv) - (&rys - &hys) * (&hzv - &rzv))._eq(&zero);
                solver.assert(&eq1);
                solver.assert(&eq2);
            }

            if let SatResult::Sat = solver.check() {
                let model = solver.get_model().unwrap();
                let rxs_v = model.eval(&rxs, true).unwrap().as_i64().unwrap();
                let rys_v = model.eval(&rys, true).unwrap().as_i64().unwrap();
                let rzs_v = model.eval(&rzs, true).unwrap().as_i64().unwrap();
                // let rxv_v = model.eval(&rxv, true).unwrap().as_i64().unwrap();
                // let ryv_v = model.eval(&ryv, true).unwrap().as_i64().unwrap();
                // let rzv_v = model.eval(&rzv, true).unwrap().as_i64().unwrap();
                // println!("{} {} {} / {} {} {}", rxs_v, rys_v, rzs_v, rxv_v, ryv_v, rzv_v);
                println!("{}", rxs_v + rys_v + rzs_v);
            } else {
                panic!("No solution!");
            }
        }
    }
}
