use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

/// DFS to find longest path in weighted graph
fn longest_path(
    edges: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
    pos: (usize, usize),
    end: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
) -> usize {
    seen.insert(pos);
    let neighbors = edges.get(&pos).unwrap();
    let m = neighbors
        .iter()
        .map(|n| {
            if seen.contains(&n.0) {
                0
            } else if n.0 == end {
                n.1
            } else {
                let m2 = longest_path(edges, n.0, end, seen);
                if m2 == 0 {
                    0
                } else {
                    n.1 + m2
                }
            }
        })
        .max()
        .unwrap();
    seen.remove(&pos);
    m
}

fn unsafe_add(x: usize, d: i64) -> usize {
    ((x as i64) + d) as usize
}

/// Start from the given position `pos` and proceed until the next junction.
/// Never go back to `prev_pos`. Also return a junction if `end` is found.
fn find_next_junction(
    pos: (usize, usize),
    prev_pos: Option<(usize, usize)>,
    grid: &Vec<Vec<char>>,
    end: &(usize, usize),
    part1: bool,
) -> Option<((usize, usize), Vec<(i64, i64)>, usize)> {
    if pos == *end {
        return Some((pos, Vec::new(), 1));
    }

    let all_directions = if part1 {
        if grid[pos.1][pos.0] == '>' {
            vec![(1, 0)]
        } else if grid[pos.1][pos.0] == '<' {
            vec![(-1, 0)]
        } else if grid[pos.1][pos.0] == '^' {
            vec![(0, -1)]
        } else if grid[pos.1][pos.0] == 'v' {
            vec![(0, 1)]
        } else {
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
        }
    } else {
        vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
    };

    // check which directions would be possible from here
    let mut possible_dirs = Vec::new();
    for dir in all_directions {
        let mut x = pos.0 as i64;
        let mut y = pos.1 as i64;
        x += dir.0;
        y += dir.1;
        if x >= 0 && (x as usize) < grid[0].len() && y >= 0 && (y as usize) < grid.len() {
            if let Some(pp) = prev_pos {
                if (x as usize, y as usize) == pp {
                    continue;
                }
            }
            if grid[y as usize][x as usize] == '#' {
                continue;
            }
            possible_dirs.push(dir);
        } else {
            continue;
        }
    }

    if possible_dirs.len() == 0 {
        // dead end
        None
    } else if possible_dirs.len() == 1 {
        // there is only one direction possible - proceed until the next junction
        let r = find_next_junction(
            (
                unsafe_add(pos.0, possible_dirs[0].0),
                unsafe_add(pos.1, possible_dirs[0].1),
            ),
            Some(pos),
            grid,
            end,
            part1,
        );
        if let Some(r) = r {
            Some((r.0, r.1, 1 + r.2))
        } else {
            r
        }
    } else {
        // there are several directions possible - this is a junction
        Some((pos, possible_dirs, 1))
    }
}

/// Insert an edge into the list of edges but only if it either does not exist
/// yet or if it's length is longer than the existing entry.
fn insert_edge(
    edges: &mut HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
    from: (usize, usize),
    to: (usize, usize),
    steps: usize,
) {
    if let Some(v) = edges.get_mut(&from) {
        if v.iter().any(|e| e.0 == to && e.1 >= steps) {
            return;
        }
        v.push((to, steps));
    } else {
        edges.insert(from, vec![(to, steps)]);
    }
}

fn find_edges(
    grid: &Vec<Vec<char>>,
    end: &(usize, usize),
    start: (usize, usize),
    part1: bool,
) -> HashMap<(usize, usize), Vec<((usize, usize), usize)>> {
    let mut nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut edges: HashMap<(usize, usize), Vec<((usize, usize), usize)>> = HashMap::new();
    let mut queue: VecDeque<((usize, usize), Option<(usize, usize)>)> = VecDeque::new();
    queue.push_back((start, None));
    while !queue.is_empty() {
        let (next_pos, prev_pos) = queue.pop_front().unwrap();

        // proceed from the current position and find the next junction
        if let Some((j, possible_dirs, steps)) =
            find_next_junction(next_pos, prev_pos, grid, end, part1)
        {
            // we've found a junction
            if let Some(pp) = prev_pos {
                // part 1: create unidirectional edge
                // part 2: create bidirectional edges
                insert_edge(&mut edges, pp, j, steps);
                if !part1 {
                    insert_edge(&mut edges, j, pp, steps);
                }
            } else {
                // create unidirectional edge from the start node to this junction
                insert_edge(&mut edges, start, j, steps - 1);
            }

            // if we haven't seen this node before, examine all possible
            // directions to find the next junctions
            if !nodes.contains(&j) {
                nodes.insert(j);
                for p in possible_dirs {
                    queue.push_back(((unsafe_add(j.0, p.0), unsafe_add(j.1, p.1)), Some(j)));
                }
            }
        }
    }
    edges
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // find start
        let mut start = (0, 0);
        for x in 0..grid[0].len() {
            if grid[0][x] == '.' {
                start = (x, 0);
                break;
            }
        }

        // find end
        let mut end = (0, 0);
        for x in 0..grid[grid.len() - 1].len() {
            if grid[grid.len() - 1][x] == '.' {
                end = (x, grid.len() - 1);
                break;
            }
        }

        // Convert the grid into a graph. Each node represents a junction where
        // the player could go into different directions. The nodes are
        // connected through edges. The value of an edge between nodes A and B
        // is equal to the distance between A and B.
        let edges = find_edges(&grid, &end, start, part1);

        // Use DFS to find the longest path from the start node to the end node.
        // Since we've converted the grid into a graph, this is a lot less
        // complex than doing DFS directly on the grid.
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let l = longest_path(&edges, start, end, &mut seen);

        println!("{}", l);
    }
}
