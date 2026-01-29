use std::{collections::VecDeque, fs, str::Bytes};

use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

fn dfs(
    input: &[u8],
    i: usize,
    start_x: i32,
    start_y: i32,
    seen: &mut FxHashSet<(usize, i32, i32)>,
    graph: &mut FxHashMap<(i32, i32), Vec<(i32, i32)>>,
) -> Vec<(usize, i32, i32)> {
    let mut result: Vec<(usize, i32, i32)> = Vec::new();

    let mut queue = Vec::new();
    queue.push((i, start_x, start_y));

    let mut options_end = 0;
    while let Some((mut i, mut x, mut y)) = queue.pop() {
        while i < input.len() {
            if !seen.insert((i, x, y)) {
                break;
            }
            match input[i] {
                b'N' => {
                    let a = (x, y);
                    y -= 1;
                    let b = (x, y);
                    graph.entry(a).or_default().push(b);
                    graph.entry(b).or_default().push(a);
                }
                b'S' => {
                    let a = (x, y);
                    y += 1;
                    let b = (x, y);
                    graph.entry(a).or_default().push(b);
                    graph.entry(b).or_default().push(a);
                }
                b'W' => {
                    let a = (x, y);
                    x -= 1;
                    let b = (x, y);
                    graph.entry(a).or_default().push(b);
                    graph.entry(b).or_default().push(a);
                }
                b'E' => {
                    let a = (x, y);
                    x += 1;
                    let b = (x, y);
                    graph.entry(a).or_default().push(b);
                    graph.entry(b).or_default().push(a);
                }
                b'(' => {
                    queue.extend(&dfs(input, i + 1, x, y, seen, graph));
                    break;
                }
                b')' => {
                    options_end = i;
                    result.push((usize::MAX, x, y));
                    break;
                }
                b'|' => {
                    result.push((usize::MAX, x, y));
                    x = start_x;
                    y = start_y;
                }
                _ => {}
            }
            i += 1;
        }
    }

    for r in result.iter_mut() {
        r.0 = options_end + 1;
    }

    result
}

/// Fast alternative solution that makes use of the fact that the input is
/// structured in a very specific way: whenever a list of options ends, we can
/// assume that we are back where we were when the list of options started. This
/// most likely applies to all official AoC inputs for this puzzle. However,
/// since I prefer generic approaches, I disabled this code in favour of a
/// slower solution that works with every input.
#[allow(unused)]
fn dfs_fast(
    input: &mut Bytes,
    mut x: i32,
    mut y: i32,
    mut steps: usize,
    map: &mut FxHashMap<(i32, i32), usize>,
) -> bool {
    while let Some(c) = input.next() {
        map.entry((x, y))
            .and_modify(|old| *old = steps.min(*old))
            .or_insert(steps);
        match c {
            b'N' => {
                y -= 1;
                steps += 1;
            }
            b'S' => {
                y += 1;
                steps += 1;
            }
            b'W' => {
                x -= 1;
                steps += 1;
            }
            b'E' => {
                x += 1;
                steps += 1;
            }
            b'(' => while dfs_fast(input, x, y, steps, map) {},
            b'|' => return true,
            _ => return false,
        }
    }
    true
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input[1..].trim().as_bytes();

    // Create a map (i.e. an undirected graph of rooms and their neighbors).
    // Note that the values are Vecs instead of HashSets. We don't care about
    // duplicate edges. They will be filtered out later during the BFS anyhow.
    let mut graph: FxHashMap<(i32, i32), Vec<(i32, i32)>> = FxHashMap::default();
    dfs(input, 0, 0, 0, &mut FxHashSet::default(), &mut graph);

    // perform BFS and record the minimum number of steps required to reach each
    // room
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    let mut best = FxHashMap::with_capacity_and_hasher(graph.len(), FxBuildHasher);
    best.insert((0, 0), 0);

    while let Some((steps, x, y)) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&(x, y)) {
            for &(cx, cy) in neighbors {
                let old = *best.get(&(cx, cy)).unwrap_or(&usize::MAX);
                if steps + 1 < old {
                    best.insert((cx, cy), steps + 1);
                    queue.push_back((steps + 1, cx, cy));
                }
            }
        }
    }

    // part 1
    println!("{}", best.values().max().unwrap());

    // part 2
    println!("{}", best.values().filter(|v| **v >= 1000).count());
}
