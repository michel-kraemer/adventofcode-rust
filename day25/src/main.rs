use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

/// Brandes' algorithm finds the top n nodes with highest betweenness centrality.
/// It basically calculates the shortest path between each pair of nodes and
/// counts how often each node is passed through. This algorithm is very efficient
/// and runs in O(n*m) time, where n is the number of vertices in the graph and
/// m is the number of edges.
///
/// See: Ulrik Brandes (2001) A Faster Algorithm for Betweenness Centrality, The
/// Journal of Mathematical Sociology, 25:2, 163-177, DOI: 10.1080/0022250X.2001.9990249
fn brandes(vertices: usize, connections: &Vec<HashSet<usize>>, top_n: usize) -> Vec<usize> {
    let mut cb = vec![0.; vertices];

    for s in 0..vertices {
        let mut stack = VecDeque::new();
        let mut p = vec![Vec::new(); vertices];

        let mut rho = vec![0; vertices];
        rho[s] = 1;

        let mut d = vec![-1; vertices];
        d[s] = 0;

        let mut queue = VecDeque::new();
        queue.push_back(s);
        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();
            stack.push_back(v);

            for w in &connections[v] {
                // w found for the first time?
                if d[*w] < 0 {
                    queue.push_back(*w);
                    d[*w] = d[v] + 1;
                }

                // shortest path to w via v?
                if d[*w] == d[v] + 1 {
                    rho[*w] += rho[v];
                    p[*w].push(v);
                }
            }
        }

        let mut delta = vec![0.; vertices];

        // stack returns vertices in order of non-increasing distance from s
        while !stack.is_empty() {
            let w = stack.pop_back().unwrap();
            for v in &p[w] {
                delta[*v] += rho[*v] as f64 / rho[w] as f64 * (1. + delta[w]);
                if w != s {
                    cb[w] += delta[w];
                }
            }
        }
    }

    let mut cb = cb.into_iter().enumerate().collect::<Vec<_>>();
    cb.sort_by(|a, b| a.1.total_cmp(&b.1));
    cb.iter().rev().take(top_n).map(|n| n.0).collect::<Vec<_>>()
}

/// Perform DFS starting with node n and collect all found nodes until no
/// more node can be visited
fn dfs(n: usize, connections: &Vec<HashSet<usize>>, visited: &mut Vec<bool>) -> usize {
    if visited[n] {
        return 0;
    }
    visited[n] = true;

    let mut r = 1;
    for c in &connections[n] {
        r += dfs(*c, connections, visited);
    }
    r
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut vertices = HashMap::new();
    let lines = input
        .lines()
        .map(|l| {
            let a = l.split_once(": ").unwrap();
            let len = vertices.len();
            let a0i = *vertices.entry(a.0).or_insert(len);
            (
                a0i,
                a.1.split(" ")
                    .map(|v| {
                        let len = vertices.len();
                        *vertices.entry(v).or_insert(len)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // create a map of all bidirectional connections
    let mut connections = vec![HashSet::new(); vertices.len()];
    for l in lines {
        connections[l.0].extend(&l.1);
        for o in l.1 {
            connections[o].insert(l.0);
        }
    }

    // use Brandes' algorithm to find the top 6 vertices with the highest
    // betweenness centrality
    let top = brandes(vertices.len(), &connections, 6);

    // remove all connections between all top nodes
    for t in &top {
        for u in &top {
            connections[*t].remove(u);
        }
    }

    // calculate number of vertices in an arbitrary cluster and multiply
    // it with the number of vertices not belonging to this cluster
    let mut visited = vec![false; vertices.len()];
    let mut astart = 0;
    while top.contains(&astart) {
        astart += 1
    }
    let n = dfs(astart, &connections, &mut visited);
    let r = n * (connections.len() - n);
    println!("{}", r);
}
