//! # Part 1
//!
//! In order to calculate if the paths of the hailstones intersect, we write two
//! paths/lines in parametric form and equate them:
//!
//! ```text
//! p1.x + t1 * v1.x = sx
//! p1.y + t1 * v1.y = sy
//! p2.x + t2 * v2.x = sx
//! p2.y + t2 * v2.y = sy
//! ```
//!
//! Equate:
//! ```text
//! => p1.x + t1 * v1.x = p2.x + t2 * v2.x
//! => p1.y + t1 * v1.y = p2.y + t2 * v2.y
//! ```
//!
//! Solve for t1 and t2:
//! ```text
//! => t1 = (p2.x + t2 * v2.x - p1.x) / v1.x
//! => t2 = (p1.y + t1 * v1.y - p2.y) / v2.y
//! ```
//!
//! Substitute t2 for t1 and rearrange:
//! ```text
//! => t1 = (p2.x + ((p1.y + t1 * v1.y - p2.y) / v2.y) * v2.x - p1.x) / v1.x
//! => t1 = (v2.x * (p1.y - p2.y) + v2.y * (p2.x - p1.x)) / (v1.x * v2.y - v2.x * v1.y)
//! ```
//!
//! Substitute t1 for t2 and rearrange:
//! ```text
//! => t2 = (p1.y + ((p2.x + t2 * v2.x - p1.x) / v1.x) * v1.y - p2.y) / v2.y
//! => t2 = (v1.x * (p1.y - p2.y) + v1.y * (p2.x - p1.x)) / (v1.x * v2.y - v2.x * v1.y)
//! ```
//!
//! Simplify:
//! ```text
//! d = v1.x * v2.y - v2.x * v1.y     <- determinant
//! cx = p2.x - p1.x
//! cy = p1.y - p2.y
//!
//! => t1 = (v2.x * cy + v2.y * cx) / d
//! => t2 = (v1.x * cy + v1.y * cx) / d
//! ```
//!
//! This approach is implemented in the [line_intersect] method.
//!
//! # Part 2
//!
//! To calculate the initial position of our rock and its velocity, we need to
//! solve a system of linear equations. In order for this system to be solvable,
//! we need three hailstones. We start with writing the equation of our rock in
//! parametric form:
//!
//! ```text
//! pr.x + t1 * vr.x = s1.x
//! pr.y + t1 * vr.y = s1.y
//! pr.z + t1 * vr.z = s1.z
//!
//! pr.x + t2 * vr.x = s2.x
//! pr.y + t2 * vr.y = s2.y
//! pr.z + t2 * vr.z = s2.z
//!
//! pr.x + t3 * vr.x = s3.x
//! pr.y + t3 * vr.y = s3.y
//! pr.z + t3 * vr.z = s3.z
//! ```
//!
//! `pr` is our rock's initial position and `vr` is its velocity. The points
//! `s1`, `s2`, and `s3` are the intersection points with three hailstones and
//! `t1`, `t2`, `t3` are the points in time our rocks hits these hailstones.
//!
//! Now, we write the equations of the hailstones in parametric form:
//!
//! ```text
//! p1.x + t1 * v1.x = s1.x
//! p1.y + t1 * v1.y = s1.y
//! p1.z + t1 * v1.z = s1.z
//!
//! p2.x + t2 * v2.x = s2.x
//! p2.y + t2 * v2.y = s2.y
//! p2.z + t2 * v2.z = s2.z
//!
//! p3.x + t3 * v3.x = s3.x
//! p3.y + t3 * v3.y = s3.y
//! p3.z + t3 * v3.z = s3.z
//! ```
//!
//! Equate:
//!
//! ```text
//! => pr.x + t1 * vr.x = p1.x + t1 * v1.x
//! => pr.y + t1 * vr.y = p1.y + t1 * v1.y
//! => pr.z + t1 * vr.z = p1.z + t1 * v1.z
//!
//! => pr.x + t2 * vr.x = p2.x + t2 * v2.x
//! => pr.y + t2 * vr.y = p2.y + t2 * v2.y
//! => pr.z + t2 * vr.z = p2.z + t2 * v2.z
//!
//! => pr.x + t3 * vr.x = p3.x + t3 * v3.x
//! => pr.y + t3 * vr.y = p3.y + t3 * v3.y
//! => pr,z + t3 * vr.z = p3.z + t3 * v3.z
//! ```
//!
//! Rearrange:
//!
//! ```text
//! => pr.x + t1 * vr.x - p1.x - t1 * v1.x = 0     (1)
//! => pr.y + t1 * vr.y - p1.y - t1 * v1.y = 0     (2)
//! => pr.z + t1 * vr.z - p1.z - t1 * v1.z = 0     (3)
//!
//! => pr.x + t2 * vr.x - p2.x - t2 * v2.x = 0     (4)
//! => pr.y + t2 * vr.y - p2.y - t2 * v2.y = 0     (5)
//! => pr.z + t2 * vr.z - p2.z - t2 * v2.z = 0     (6)
//!
//! => pr.x + t3 * vr.x - p3.x - t3 * v3.x = 0     (7)
//! => pr.y + t3 * vr.y - p3.y - t3 * v3.y = 0     (8)
//! => pr.z + t3 * vr.z - p3.z - t3 * v3.z = 0     (9)
//! ```
//!
//! Now, we've got 9 equations with 9 unknown variables (see [equations]). We
//! use Newton–Raphson to solve it numerically
//! (<https://en.wikipedia.org/wiki/Newton%27s_method>).
//!
//! We start with the Jacobian matrix
//! (<https://en.wikipedia.org/wiki/Jacobian_matrix_and_determinant>). For this,
//! we need to find the partial derivative of each equation with respect to each
//! unknown variable. For a system of equations with 9 unknowns and 9 equations,
//! we therefore need to create a 9x9 matrix. Normally, this means we need to
//! find 81 partial derivatives. However, in our case, the equations are simpler
//! and most coefficients are 0. For example, in equation (1), we only have the
//! unknowns `pr.x`, `t1`, and `vr.x`. The other unknowns have a coefficient of
//! 0. The full equation (in terms of the system of equations) would actually
//! look like this:
//!
//! ```text
//! 1 * pr.x + 0 * pr.y + 0 * pr.z + 1 * t1 * vr.x + 0 * t1 * vr.y ... and so on
//! ```
//!
//! But most coefficients are 0, so we only need to create 9 partial derivates
//! in total. The rest of the Jacobian matrix is just 0.
//!
//! Let's start with the first row (the partial derivates of the first equation
//! with respect to the unknowns `pr.x`, `t1`, `vr.x`):
//!
//! ```text
//! ∂pr.x => 1
//! ∂t1   => vr.x - v1.x
//! ∂vr.x => t1
//! ```
//!
// Second equation:
//!
//! ```text
//! ∂pr.y => 1
//! ∂t1   => vr.y - v1.y
//! ∂vr.y => t1
//! ```
//!
//! ... and so on.
//!
//! The final Jacobian matrix looks like this (see [jacobian]):
//!
//! ```text
//!     ∂pr.x  ∂pr.y  ∂pr.z      ∂t1          ∂t2          ∂t3      ∂vr.x  ∂vr.y  ∂vr.z
//! (1)   1      0      0    vr.x - v1.x       0            0         t1      0      0
//! (2)   0      1      0    vr.y - v1.y       0            0          0     t1      0
//! (3)   0      0      1    vr.z - v1.z       0            0          0      0     t1
//! (4)   1      0      0         0       vr.x - v2.x       0         t2      0      0
//! (5)   0      1      0         0       vr.y - v2.y       0          0     t2      0
//! (6)   0      0      1         0       vr.z - v2.z       0          0      0     t2
//! (7)   1      0      0         0            0       vr.x - v3.x    t3      0      0
//! (8)   0      1      0         0            0       vr.y - v3.y     0     t3      0
//! (9)   0      0      1         0            0       vr.z - v3.z     0      0     t3
//! ```
//!
//! Finally, we perform Newton-Raphson using some random (but reasonable) start
//! values for `pr`, `vr`, `t1`, `t2`, `t3` (see main function).

use std::fs;

use crate::gauss::solve;

mod gauss;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Intersect two lines and returns the intersection point (if there is one)
fn line_intersect(p1: Point, v1: Point, p2: Point, v2: Point) -> Option<Point> {
    let d = v1.x * v2.y - v2.x * v1.y;
    if d == 0.0 {
        // lines are parallel
        return None;
    }

    let cx = p2.x - p1.x;
    let cy = p1.y - p2.y;

    let t1 = (v2.x * cy + v2.y * cx) / d;
    let t2 = (v1.x * cy + v1.y * cx) / d;

    if t1 < 0.0 || t2 < 0.0 {
        // lines do not intersect
        return None;
    }

    let sx = p1.x + t1 * v1.x;
    let sy = p1.y + t1 * v1.y;

    Some(Point::new(sx, sy, 0.0))
}

/// The system of equations to solve
#[allow(clippy::too_many_arguments)]
fn equations(
    pr: Point,
    t1: f64,
    t2: f64,
    t3: f64,
    vr: Point,
    p1: Point,
    p2: Point,
    p3: Point,
    v1: Point,
    v2: Point,
    v3: Point,
) -> [f64; 9] {
    [
        pr.x + t1 * vr.x - p1.x - t1 * v1.x,
        pr.y + t1 * vr.y - p1.y - t1 * v1.y,
        pr.z + t1 * vr.z - p1.z - t1 * v1.z,
        pr.x + t2 * vr.x - p2.x - t2 * v2.x,
        pr.y + t2 * vr.y - p2.y - t2 * v2.y,
        pr.z + t2 * vr.z - p2.z - t2 * v2.z,
        pr.x + t3 * vr.x - p3.x - t3 * v3.x,
        pr.y + t3 * vr.y - p3.y - t3 * v3.y,
        pr.z + t3 * vr.z - p3.z - t3 * v3.z,
    ]
}

/// The Jacobian matrix of our system of equations. It contains the partial
/// derivatives of each equation (rows) with respect to each unknown variable
/// (columns).
fn jacobian(
    t1: f64,
    t2: f64,
    t3: f64,
    vr: Point,
    v1: Point,
    v2: Point,
    v3: Point,
) -> [[f64; 9]; 9] {
    [
        [1.0, 0.0, 0.0, vr.x - v1.x, 0.0, 0.0, t1, 0.0, 0.0],
        [0.0, 1.0, 0.0, vr.y - v1.y, 0.0, 0.0, 0.0, t1, 0.0],
        [0.0, 0.0, 1.0, vr.z - v1.z, 0.0, 0.0, 0.0, 0.0, t1],
        [1.0, 0.0, 0.0, 0.0, vr.x - v2.x, 0.0, t2, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0, vr.y - v2.y, 0.0, 0.0, t2, 0.0],
        [0.0, 0.0, 1.0, 0.0, vr.z - v2.z, 0.0, 0.0, 0.0, t2],
        [1.0, 0.0, 0.0, 0.0, 0.0, vr.x - v3.x, t3, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0, 0.0, vr.y - v3.y, 0.0, t3, 0.0],
        [0.0, 0.0, 1.0, 0.0, 0.0, vr.z - v3.z, 0.0, 0.0, t3],
    ]
}

/// Perform Newton-Raphson and find values for `pr`, `t1`, `t2`, `t3`, and `vr`.
/// Use the given values as starting guesses.
#[allow(clippy::too_many_arguments)]
fn newton(
    mut pr: Point,
    mut t1: f64,
    mut t2: f64,
    mut t3: f64,
    mut vr: Point,
    p1: Point,
    p2: Point,
    p3: Point,
    v1: Point,
    v2: Point,
    v3: Point,
) -> Option<Point> {
    const MAX_ITERATIONS: usize = 1000;

    for _ in 0..MAX_ITERATIONS {
        if pr.x.is_nan()
            || pr.y.is_nan()
            || pr.z.is_nan()
            || t1.is_nan()
            || t2.is_nan()
            || t3.is_nan()
            || vr.x.is_nan()
            || vr.y.is_nan()
            || vr.z.is_nan()
        {
            // The guesses have diverged. Try again with another set of initial
            // guesses!
            return None;
        }

        // use the current guesses and check how far we're off
        let f = equations(pr, t1, t2, t3, vr, p1, p2, p3, v1, v2, v3);
        let dist = f.iter().map(|v| v * v).sum::<f64>().sqrt();
        if dist < 1e-6 {
            // error is small enough
            return Some(pr);
        }

        // calculate the values of the Jacobian matrix for the current guesses
        let j = jacobian(t1, t2, t3, vr, v1, v2, v3);

        // equate the Jacobian matrix (left side) with the current values of
        // our equations (right side)
        let mut m = [[0.0; 10]; 9];
        for y in 0..9 {
            for x in 0..9 {
                m[y][x] = j[y][x];
            }
        }
        for y in 0..9 {
            // `solve` assumes a matrix of only the left side of the system of
            // equations (the right side is just 0). We use the negative sign
            // here to move the current values of our equations from the right
            // to the left side
            m[y][9] = -f[y];
        }

        // solve the equation system to get the deltas for all variables
        let r = solve(&mut m);

        // modify guesses according to the calculated deltas
        pr.x += r[0];
        pr.y += r[1];
        pr.z += r[2];
        t1 += r[3];
        t2 += r[4];
        t3 += r[5];
        vr.x += r[6];
        vr.y += r[7];
        vr.z += r[8];
    }

    // no solution found within MAX_ITERATIONS
    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let mut points = Vec::new();
    for l in lines {
        let (coordinates, velocities) = l.split_once(" @ ").unwrap();
        let coordinates = coordinates
            .split(", ")
            .map(|c| c.parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        let velocities = velocities
            .split(", ")
            .map(|c| c.parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        points.push((
            Point::new(coordinates[0], coordinates[1], coordinates[2]),
            Point::new(velocities[0], velocities[1], velocities[2]),
        ));
    }

    let min = 200000000000000.0;
    let max = 400000000000000.0;

    // part 1 ...

    let mut total1 = 0;
    for (i, &(p1, v1)) in points.iter().enumerate() {
        for &(p2, v2) in points.iter().skip(i + 1) {
            if let Some(s) = line_intersect(p1, v1, p2, v2)
                && (min..=max).contains(&s.x)
                && (min..=max).contains(&s.y)
            {
                // intersection is in range
                total1 += 1;
            }
        }
    }
    println!("{}", total1);

    // part 2 ...

    // take three random points
    let (p1, v1) = points[0];
    let (p2, v2) = points[1];
    let (p3, v3) = points[2];

    // Perform Newton-Raphson until we find a solution. Use reasonable initial
    // guesses, to speed up the process (e.g. t1, t2, and t3 can never be
    // negative).
    'outer: for sign_px in [1.0, -1.0] {
        for sign_py in [1.0, -1.0] {
            for sign_pz in [1.0, -1.0] {
                for sign_vx in [1.0, -1.0] {
                    for sign_vy in [1.0, -1.0] {
                        for sign_vz in [1.0, -1.0] {
                            if let Some(r) = newton(
                                Point::new(100.0 * sign_px, 200.0 * sign_py, 300.0 * sign_pz),
                                400.0,
                                500.0,
                                600.0,
                                Point::new(700.0 * sign_vx, 800.0 * sign_vy, 900.0 * sign_vz),
                                p1,
                                p2,
                                p3,
                                v1,
                                v2,
                                v3,
                            ) {
                                // found solution
                                println!("{}", (r.x.round() + r.y.round() + r.z.round()) as u64);
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }
}
