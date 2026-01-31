use std::{fs, str::Bytes};

use rustc_hash::FxHashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    /// Compute Manhattan distance to another point
    fn dist(&self, other: &Point) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs()
    }
}

struct Bot {
    center: Point,
    range: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    min: Point,
    max: Point,
}

impl Node {
    /// Subdivide this octree node into its 8 children
    fn subdivide(&self) -> [Node; 8] {
        let cx = self.max.x - self.min.x;
        let cy = self.max.y - self.min.y;
        let cz = self.max.z - self.min.z;
        let hx = self.min.x + cx / 2;
        let hy = self.min.y + cy / 2;
        let hz = self.min.z + cz / 2;
        [
            Node {
                min: Point {
                    x: self.min.x,
                    y: self.min.y,
                    z: self.min.z,
                },
                max: Point {
                    x: hx,
                    y: hy,
                    z: hz,
                },
            },
            Node {
                min: Point {
                    x: hx,
                    y: self.min.y,
                    z: self.min.z,
                },
                max: Point {
                    x: self.max.x,
                    y: hy,
                    z: hz,
                },
            },
            Node {
                min: Point {
                    x: self.min.x,
                    y: hy,
                    z: self.min.z,
                },
                max: Point {
                    x: hx,
                    y: self.max.y,
                    z: hz,
                },
            },
            Node {
                min: Point {
                    x: hx,
                    y: hy,
                    z: self.min.z,
                },
                max: Point {
                    x: self.max.x,
                    y: self.max.y,
                    z: hz,
                },
            },
            Node {
                min: Point {
                    x: self.min.x,
                    y: self.min.y,
                    z: hz,
                },
                max: Point {
                    x: hx,
                    y: hy,
                    z: self.max.z,
                },
            },
            Node {
                min: Point {
                    x: hx,
                    y: self.min.y,
                    z: hz,
                },
                max: Point {
                    x: self.max.x,
                    y: hy,
                    z: self.max.z,
                },
            },
            Node {
                min: Point {
                    x: self.min.x,
                    y: hy,
                    z: hz,
                },
                max: Point {
                    x: hx,
                    y: self.max.y,
                    z: self.max.z,
                },
            },
            Node {
                min: Point {
                    x: hx,
                    y: hy,
                    z: hz,
                },
                max: Point {
                    x: self.max.x,
                    y: self.max.y,
                    z: self.max.z,
                },
            },
        ]
    }

    /// Check if this node has a volume of 0
    fn is_empty(&self) -> bool {
        self.min.x == self.max.x || self.min.y == self.max.y || self.min.z == self.max.z
    }

    /// Check if this node represents a single cell (i.e. if its width, height,
    /// and depth are all 1)
    fn is_single_cell(&self) -> bool {
        self.max.x - self.min.x == 1 && self.max.y - self.min.y == 1 && self.max.z - self.min.z == 1
    }

    /// Check if this any cell within this node is in the range of the given bot
    fn intersects(&self, bot: &Bot) -> bool {
        let nearest_point = Point {
            x: self.min.x.max(bot.center.x.min(self.max.x - 1)),
            y: self.min.y.max(bot.center.y.min(self.max.y - 1)),
            z: self.min.z.max(bot.center.z.min(self.max.z - 1)),
        };
        bot.center.dist(&nearest_point) <= bot.range
    }
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
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut bytes = input.bytes();
    let mut bots = Vec::new();
    loop {
        let Some(x) = parse_next_number(&mut bytes) else {
            break;
        };
        let y = parse_next_number(&mut bytes).unwrap();
        let z = parse_next_number(&mut bytes).unwrap();
        let range = parse_next_number(&mut bytes).unwrap();
        bots.push(Bot {
            center: Point { x, y, z },
            range,
        });
    }

    // part 1
    let strongest_bot = bots.iter().max_by_key(|b| b.range).unwrap();
    let in_range_of_strongest = bots
        .iter()
        .filter(|b| b.center.dist(&strongest_bot.center) <= strongest_bot.range)
        .count();
    println!("{in_range_of_strongest}");

    // part 2 ...
    // compute bounding box
    let min_x = bots.iter().map(|b| b.center.x - b.range).min().unwrap();
    let min_y = bots.iter().map(|b| b.center.y - b.range).min().unwrap();
    let min_z = bots.iter().map(|b| b.center.z - b.range).min().unwrap();
    let max_x = bots.iter().map(|b| b.center.x + b.range).max().unwrap() + 1;
    let max_y = bots.iter().map(|b| b.center.y + b.range).max().unwrap() + 1;
    let max_z = bots.iter().map(|b| b.center.z + b.range).max().unwrap() + 1;

    let mut cache: FxHashMap<Node, usize> = FxHashMap::default();

    'outer: for min_intersections in (0..bots.len()).rev() {
        // create octree root node
        let mut nodes = vec![Node {
            min: Point {
                x: min_x,
                y: min_y,
                z: min_z,
            },
            max: Point {
                x: max_x,
                y: max_y,
                z: max_z,
            },
        }];

        // Recursively look for nodes that are a single cell and which intersect
        // with exactly `min_intersections` bots. This can only happen if their
        // parent nodes intersection with at least `min_intersections` bots.
        // Otherwise, we don't need to go deeper.
        while !nodes.is_empty() {
            let mut new_nodes = Vec::new();
            for n in &nodes {
                for child in n.subdivide().into_iter().filter(|c| !c.is_empty()) {
                    let n_intersections = *cache
                        .entry(child)
                        .or_insert_with(|| bots.iter().filter(|b| child.intersects(b)).count());
                    if n_intersections >= min_intersections {
                        new_nodes.push(child);
                    }
                }
            }
            nodes = new_nodes;

            // find single-cell nodes
            let singles = nodes
                .iter()
                .filter(|n| n.is_single_cell())
                .collect::<Vec<_>>();
            if !singles.is_empty() {
                // find cell that is closest to (0,0,0)
                let closest = singles
                    .into_iter()
                    .min_by_key(|s| s.min.x + s.min.y + s.min.z)
                    .unwrap();
                println!("{}", closest.min.x + closest.min.y + closest.min.z);
                break 'outer;
            }
        }
    }
}
