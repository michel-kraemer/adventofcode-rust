use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Asteroid {
    x: i32,
    y: i32,
}

fn in_sight(asteroids: &Vec<Asteroid>, from: Asteroid) -> Vec<Asteroid> {
    let mut result = Vec::new();

    for &to in asteroids {
        if from == to {
            continue;
        }

        let mut blocked = false;
        for &between in asteroids {
            if between == from || between == to {
                continue;
            }

            let dx = to.x - from.x;
            let dy = to.y - from.y;

            let dxb = between.x - from.x;
            let dyb = between.y - from.y;

            let cross = dxb * dy - dyb * dx;
            if cross == 0 {
                let b = if dx.abs() >= dy.abs() {
                    if dx > 0 {
                        from.x <= between.x && between.x <= to.x
                    } else {
                        to.x <= between.x && between.x <= from.x
                    }
                } else if dy > 0 {
                    from.y <= between.y && between.y <= to.y
                } else {
                    to.y <= between.y && between.y <= from.y
                };

                if b {
                    blocked = true;
                    break;
                }
            }
        }

        if !blocked {
            result.push(to);
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Asteroid {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // part 1
    let (station, max_in_sight) = asteroids
        .iter()
        .map(|&a| (a, in_sight(&asteroids, a)))
        .max_by_key(|a| a.1.len())
        .unwrap();
    println!("{}", max_in_sight.len());

    // part 2
    let mut nd = 0;
    let result: Asteroid;
    'outer: loop {
        let is = in_sight(&asteroids, station);
        let mut angles = is
            .into_iter()
            .map(|i| {
                let delta_x = i.x - station.x;
                let delta_y = i.y - station.y;
                let r = (delta_x as f64).atan2(delta_y as f64);
                (r, i)
            })
            .collect::<Vec<_>>();
        angles.sort_by(|a, b| b.0.total_cmp(&a.0));

        assert!(!angles.is_empty(), "less than 200 asteroids");

        for (_, d) in angles {
            nd += 1;
            if nd == 200 {
                result = d;
                break 'outer;
            }
            let j = asteroids.iter().position(|&p| p == d).unwrap();
            asteroids.remove(j);
        }
    }
    println!("{}", result.x * 100 + result.y);
}
