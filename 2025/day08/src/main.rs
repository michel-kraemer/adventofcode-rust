use std::fs;

struct Node {
    parent: usize,
    size: usize,
}

fn find(x: usize, nodes: &mut [Node]) -> usize {
    if nodes[x].parent != x {
        nodes[x].parent = find(nodes[x].parent, nodes);
        return nodes[x].parent;
    }
    x
}

fn union(x: usize, y: usize, nodes: &mut [Node]) -> usize {
    let mut x = find(x, nodes);
    let mut y = find(y, nodes);

    if x == y {
        return x;
    }

    if nodes[x].size < nodes[y].size {
        std::mem::swap(&mut x, &mut y);
    }

    nodes[y].parent = x;
    nodes[x].size += nodes[y].size;

    x
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let boxes = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // calculate the distances between all pairs and sort them
    let mut distances = Vec::new();
    for a in 0..boxes.len() {
        for b in a + 1..boxes.len() {
            let dx = boxes[a][0].abs_diff(boxes[b][0]);
            let dy = boxes[a][1].abs_diff(boxes[b][1]);
            let dz = boxes[a][2].abs_diff(boxes[b][2]);
            let d = dx * dx + dy * dy + dz * dz; // squared distance is enough
            distances.push((d, a, b));
        }
    }
    distances.sort_unstable_by_key(|d| d.0);

    // create union-find data structure
    let mut nodes = (0..boxes.len())
        .map(|i| Node { parent: i, size: 1 })
        .collect::<Vec<_>>();

    // make the first 1000 connections and keep track of the cluster sizes
    let mut sizes = vec![1; nodes.len()];
    let mut n_clusters = nodes.len();
    for &d in distances.iter().take(1000) {
        let a = find(d.1, &mut nodes);
        let b = find(d.2, &mut nodes);
        if a != b {
            n_clusters -= 1;
            let parent = union(a, b, &mut nodes);
            sizes[a] = 0;
            sizes[b] = 0;
            sizes[parent] = nodes[parent].size;
        }
    }

    // take the product of the top 3 sizes
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    println!("{}", sizes[0..3].iter().product::<usize>());

    // make the remaining connections
    for &d in distances.iter().skip(1000) {
        let a = find(d.1, &mut nodes);
        let b = find(d.2, &mut nodes);
        if a != b {
            union(a, b, &mut nodes);
            n_clusters -= 1;

            if n_clusters == 1 {
                // there is only one cluster left
                println!("{}", boxes[d.1][0] * boxes[d.2][0]);
                break;
            }
        }
    }
}
