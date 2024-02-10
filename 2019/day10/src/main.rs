use std::{collections::BTreeMap, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Asteroid {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, PartialEq)]
struct Angle(f64);

impl Eq for Angle {}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.total_cmp(&self.0)
    }
}

fn in_sight(asteroids: &[Asteroid], from: Asteroid) -> BTreeMap<Angle, Asteroid> {
    let mut result: BTreeMap<Angle, Asteroid> = BTreeMap::new();
    for &to in asteroids {
        if from == to {
            continue;
        }

        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let a = Angle((dx as f64).atan2(dy as f64));
        let l = dx.abs() + dy.abs();
        result
            .entry(a)
            .and_modify(|e| {
                let l2 = (e.x - from.x).abs() + (e.y - from.y).abs();
                if l < l2 {
                    *e = to;
                }
            })
            .or_insert(to);
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
        let angles = in_sight(&asteroids, station);

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
