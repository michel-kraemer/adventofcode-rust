use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Particle {
    position: Point,
    velocity: Point,
    acceleration: Point,
}

/// Calculate the displacement `d` of a particle with velocity `v` and
/// acceleration `a` at time `t`, such that `p_0 + d = p_t`, where `p_0` is the
/// particle's position at time 0, and `p_t` is its position at time `t`.
///
/// Looking at the example from the problem statement, we can see that the
/// velocity of a particle changes over time in an arithmetic progression. `p_t`
/// equals `p_0` plus the sum of the values in the arithmetic progression. We
/// start with formula for this sum:
///
/// ```plain
/// S = (v2 - v1 + 1)(v1 + v2) / 2
/// ```
///
/// To get the sum of the values with a distance of `a.abs()`, we need to divide
/// `v1` and `v2` by `a.abs()` (scale down) and then multiply again (scale up):
///
/// ```plain
/// S = ((v2/a.abs() - v1/a.abs() + 1)(v1/a.abs() + v2/a.abs()) / 2) * a.abs()
///
/// where v2 = v + a * t, v1 = v + a   if a < 0
///    or v2 = v + a, v1 = v + a * t   if a > 0
/// ```
///
/// If `a == 0`, then simply `S = v * t`
///
/// By combining these formulae and rearranging, we obtain the following generic
/// formula for the displacement:
///
/// ```plain
/// (a * t * t + (2 * v + a) * t) / 2
/// ```
fn get_displacement(v: i64, a: i64, t: i64) -> i64 {
    (a * t * t + (2 * v + a) * t) / 2
}

/// Calculate the position of a `particle` at time `t`
fn get_position(particle: Particle, t: i64) -> Point {
    let displacement = Point {
        x: get_displacement(particle.velocity.x, particle.acceleration.x, t),
        y: get_displacement(particle.velocity.y, particle.acceleration.y, t),
        z: get_displacement(particle.velocity.z, particle.acceleration.z, t),
    };
    Point {
        x: particle.position.x + displacement.x,
        y: particle.position.y + displacement.y,
        z: particle.position.z + displacement.z,
    }
}

/// Check if two particles will collide at a time `t > 0`
///
/// For this, we equate the formula for the position of the two particles at a
/// time `t`:
///
/// ```plain
/// (a_1 * t * t + (2 * v_1 + a_1) * t) / 2 + p_1
///     = (a_2 * t * t + (2 * v_2 + a_2) * t) / 2 + p_2
/// ```
///
/// and solve it for `t`. See [Wolfram|Alpha]
///
/// We obtain the following formulae:
///
/// ```plain
/// t = (sqrt((a1 - a2 + 2 * v1 - 2 * v2)^2 - 8 * (a1 - a2) * (p1 - p2))
///         - (a1 - a2 + 2 * v1 - 2 * v2)) / (2 * (a1 - a2))
///   or
/// t = (sqrt((a1 - a2 + 2 * v1 - 2 * v2)^2 - 8 * (a1 - a2) * (p1 - p2))
///         + (a1 - a2 + 2 * v1 - 2 * v2)) / (2 * (a2 - a1))
///   if a1 != a2
///
/// and
///
/// t = (p2 - p1)/(v1 - v2)   if a1 == a2 and v1 != v2
/// ```
///
/// In all other cases, there is no solution (i.e. the particles do not collide)
///
/// [Wolfram|Alpha]: https://www.wolframalpha.com/input?i=solve+%28a_1+*+t+*+t+%2B+%282+*+v_1+%2B+a_1%29+*+t%29+%2F+2+%2B+p_1+%3D+%28a_2+*+t+*+t+%2B+%282+*+v_2+%2B+a_2%29+*+t%29+%2F+2+%2B+p_2+for+t
fn do_collide(particle1: Particle, particle2: Particle) -> bool {
    let p1 = particle1.position.x;
    let p2 = particle2.position.x;
    let v1 = particle1.velocity.x;
    let v2 = particle2.velocity.x;
    let a1 = particle1.acceleration.x;
    let a2 = particle2.acceleration.x;

    if a1 == a2 && v1 == v2 {
        // no solution
        return false;
    }

    if a1 == a2 && v1 != v2 {
        if (p2 - p1) % (v1 - v2) != 0 {
            // no integer solution
            return false;
        }
        let t = (p2 - p1) / (v1 - v2);
        if t > 0 {
            let d1 = get_position(particle1, t);
            let d2 = get_position(particle2, t);
            return d1 == d2;
        }
        return false;
    }

    let r1 = a1 - a2 + 2 * v1 - 2 * v2;
    let r2 = 8 * (a1 - a2) * (p1 - p2);
    let r3 = r1 * r1 - r2;
    if r3 < 0 {
        // no solution
        return false;
    }
    let s = r3.isqrt();
    if s * s != r3 {
        // no integer solution
        return false;
    }

    if (s - r1) % (2 * (a1 - a2)) == 0 {
        let t = (s - r1) / (2 * (a1 - a2));
        if t > 0 {
            let d1 = get_position(particle1, t);
            let d2 = get_position(particle2, t);
            if d1 == d2 {
                return true;
            }
        }
    }
    if (s + r1) % (2 * (a2 - a1)) == 0 {
        let t = (s + r1) / (2 * (a2 - a1));
        if t > 0 {
            let d1 = get_position(particle1, t);
            let d2 = get_position(particle2, t);
            if d1 == d2 {
                return true;
            }
        }
    };

    false
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut particles = input
        .lines()
        .map(|l| {
            let mut parts = l.split(", ").map(|v| {
                v[3..v.len() - 1]
                    .split(',')
                    .map(|i| i.parse::<i64>().unwrap())
            });
            let mut position = parts.next().unwrap();
            let position = Point {
                x: position.next().unwrap(),
                y: position.next().unwrap(),
                z: position.next().unwrap(),
            };
            let mut velocity = parts.next().unwrap();
            let velocity = Point {
                x: velocity.next().unwrap(),
                y: velocity.next().unwrap(),
                z: velocity.next().unwrap(),
            };
            let mut acceleration = parts.next().unwrap();
            let acceleration = Point {
                x: acceleration.next().unwrap(),
                y: acceleration.next().unwrap(),
                z: acceleration.next().unwrap(),
            };
            Particle {
                position,
                velocity,
                acceleration,
            }
        })
        .collect::<Vec<_>>();

    // part 1 - determine the position of all particles in the very distant
    // future (at time t=1_000_000) and look for the particle that is closest to
    // the origin
    println!(
        "{}",
        particles
            .iter()
            .enumerate()
            .min_by_key(|&(_, p)| {
                let pos = get_position(*p, 1_000_000);
                pos.x.abs() + pos.y.abs() + pos.z.abs()
            })
            .unwrap()
            .0
    );

    // part 2 - iteratively filter out particles that collide and count those
    // that don't at the same time
    let mut good = 0;
    while let Some(p1) = particles.pop() {
        let old_len = particles.len();
        particles.retain(|p2| !do_collide(p1, *p2));
        if particles.len() == old_len {
            good += 1;
        }
    }
    println!("{good}");
}
