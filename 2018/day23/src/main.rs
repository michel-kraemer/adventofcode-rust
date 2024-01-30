use std::fs;

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

struct Bot {
    center: Point,
    range: f64,
}

struct Node {
    min: Point,
    max: Point,
}

impl Point {
    fn dist(&self, other: &Point) -> f64 {
        (other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs()
    }
}

fn intersects(bot: &Bot, node: &Node) -> bool {
    let nearest_point = Point {
        x: node.min.x.max(bot.center.x.min(node.max.x)),
        y: node.min.y.max(bot.center.y.min(node.max.y)),
        z: node.min.z.max(bot.center.z.min(node.max.z)),
    };
    bot.center.dist(&nearest_point) <= bot.range
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let bots = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(' ').unwrap();
            let left = left.split(&['<', '>']).collect::<Vec<_>>();
            let pos = left[1];
            let pos = pos
                .split(',')
                .map(|p| p.parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            let range = right[2..].parse::<f64>().unwrap();
            Bot {
                center: Point {
                    x: pos[0],
                    y: pos[1],
                    z: pos[2],
                },
                range,
            }
        })
        .collect::<Vec<_>>();

    // part 1
    let strongest_bot = bots
        .iter()
        .max_by(|b1, b2| b1.range.total_cmp(&b2.range))
        .unwrap();
    let in_range_of_strongest = bots
        .iter()
        .filter(|b| b.center.dist(&strongest_bot.center) <= strongest_bot.range)
        .count();
    println!("{}", in_range_of_strongest);

    // part 2 ...
    // calculate bounding box
    let min_x = bots
        .iter()
        .map(|b| b.center.x - b.range)
        .min_by(|a, b| a.total_cmp(b))
        .unwrap();
    let min_y = bots
        .iter()
        .map(|b| b.center.y - b.range)
        .min_by(|a, b| a.total_cmp(b))
        .unwrap();
    let min_z = bots
        .iter()
        .map(|b| b.center.z - b.range)
        .min_by(|a, b| a.total_cmp(b))
        .unwrap();
    let max_x = bots
        .iter()
        .map(|b| b.center.x + b.range)
        .max_by(|a, b| a.total_cmp(b))
        .unwrap() as f64;
    let max_y = bots
        .iter()
        .map(|b| b.center.y + b.range)
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();
    let max_z = bots
        .iter()
        .map(|b| b.center.z + b.range)
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();

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

    // current size of child nodes on this level
    let mut cx = (max_x - min_x) / 2.;
    let mut cy = (max_y - min_y) / 2.;
    let mut cz = (max_z - min_z) / 2.;

    // as long as it makes sense to go one level deeper ...
    while cx > 1. || cy > 1. || cz > 1. {
        // for all child nodes, check how many intersections they have with
        // bots and only keep those child nodes that have the highest number
        // of intersections
        let mut new_nodes = Vec::new();
        let mut min_n_intersections = 0;
        for n in nodes {
            for x in 0..2 {
                for y in 0..2 {
                    for z in 0..2 {
                        let child = Node {
                            min: Point {
                                x: n.min.x + cx * x as f64,
                                y: n.min.y + cy * y as f64,
                                z: n.min.z + cz * z as f64,
                            },
                            max: Point {
                                x: n.min.x + cx * (x + 1) as f64,
                                y: n.min.y + cy * (y + 1) as f64,
                                z: n.min.z + cz * (z + 1) as f64,
                            },
                        };

                        // check how many intersections this child node has
                        // with all bots
                        let n_intersections = bots.iter().filter(|b| intersects(b, &child)).count();

                        // only keep this node if it has as many intersections
                        // or more than the previous nodes
                        if n_intersections >= min_n_intersections {
                            if n_intersections > min_n_intersections {
                                // we found a better minimum number of
                                // intersections - throw away all other nodes
                                new_nodes.clear();
                                min_n_intersections = n_intersections;
                            }
                            new_nodes.push(child);
                        }
                    }
                }
            }
        }

        assert!(!new_nodes.is_empty(), "No nodes left");

        nodes = new_nodes;
        cx /= 2.;
        cy /= 2.;
        cz /= 2.;
    }

    // iterate through all integer positions in the remaining nodes (the search
    // space should be very small at this point) and calculate the number of
    // intersections with all bots as well as the distance to (0, 0, 0)
    let mut distances = Vec::new();
    for n in nodes {
        for x in (n.min.x.floor() as i64)..=(n.max.x.ceil() as i64) {
            for y in (n.min.y.floor() as i64)..=(n.max.y.ceil() as i64) {
                for z in (n.min.z.floor() as i64)..=(n.max.z.ceil() as i64) {
                    let p = Point {
                        x: x as f64,
                        y: y as f64,
                        z: z as f64,
                    };
                    let in_range = bots.iter().filter(|b| b.center.dist(&p) <= b.range).count();
                    distances.push((x + y + z, in_range));
                }
            }
        }
    }

    // find the points that are in range of the most bots and then select
    // the one with the lowest distance to (0, 0, 0)
    let max_in_range = distances.iter().map(|d| d.1).max().unwrap();
    let min_distance = distances
        .iter()
        .filter(|d| d.1 == max_in_range)
        .map(|d| d.0)
        .min()
        .unwrap();
    println!("{}", min_distance);
}
