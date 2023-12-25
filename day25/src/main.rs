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
fn brandes<'a>(
    vertices: &Vec<&'a str>,
    connections: &HashMap<&'a str, HashSet<&'a str>>,
    top_n: usize,
) -> Vec<&'a str> {
    let empty_vec = vec![];
    let mut cb: HashMap<&str, f64> = HashMap::new();

    for s in vertices {
        let mut stack: VecDeque<&str> = VecDeque::new();
        let mut p: HashMap<&str, Vec<&str>> = HashMap::new();

        let mut rho: HashMap<&str, i64> = HashMap::new();
        for t in vertices {
            rho.insert(t, 0);
        }
        rho.insert(s, 1);

        let mut d: HashMap<&str, i64> = HashMap::new();
        d.insert(s, 0);

        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_back(s);
        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();
            stack.push_back(v);

            for w in &connections[v] {
                // w found for the first time?
                if !d.contains_key(w) {
                    queue.push_back(w);
                    d.insert(w, d[v] + 1);
                }

                // shortest path to w via v?
                if d[w] == d[v] + 1 {
                    *rho.entry(w).or_default() += *rho.get(v).unwrap();
                    p.entry(w).or_default().push(v);
                }
            }
        }

        let mut delta: HashMap<&str, f64> = HashMap::new();
        for t in vertices {
            delta.insert(t, 0.);
        }

        // stack returns vertices in order of non-increasing distance from s
        while !stack.is_empty() {
            let w = stack.pop_back().unwrap();
            let pl = if p.contains_key(w) {
                p.get(w).unwrap()
            } else {
                &empty_vec
            };
            for v in pl {
                *delta.entry(v).or_default() += rho[v] as f64 / rho[w] as f64 * (1. + delta[w]);
                if w != *s {
                    *cb.entry(w).or_default() += delta[w];
                }
            }
        }
    }

    let mut cb = cb.into_iter().collect::<Vec<_>>();
    cb.sort_by(|a, b| a.1.total_cmp(&b.1));
    cb.iter().rev().take(top_n).map(|n| n.0).collect::<Vec<_>>()
}

/// Perform DFS starting with node n and collect all found nodes until no
/// more node can be visited
fn dfs<'a>(
    n: &'a str,
    connections: &HashMap<&'a str, HashSet<&'a str>>,
    visited: &mut HashSet<&'a str>,
) {
    if visited.contains(n) {
        return;
    }

    visited.insert(n);
    for c in &connections[n] {
        dfs(c, connections, visited)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input
        .lines()
        .map(|l| {
            let a = l.split_once(": ").unwrap();
            (a.0, a.1.split(" ").collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    // create a map of all bidirectional connections
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for l in lines {
        connections.entry(l.0).or_default().extend(&l.1);
        for o in l.1 {
            connections.entry(o).or_default().insert(l.0);
        }
    }

    // get list of all vertices
    let vertices = connections.keys().map(|k| *k).collect::<Vec<_>>();

    // use Brandes' algorithm to find the top 6 vertices with the highest
    // betweenness centrality
    let top = brandes(&vertices, &connections, 6);

    // remove all connections between all top nodes
    for t in &top {
        for u in &top {
            connections.entry(t).or_default().remove(u);
        }
    }

    // calculate number of vertices in an arbitrary cluster and multiply
    // it with the number of vertices not belonging to this cluster
    let mut visited = HashSet::new();
    dfs(
        connections.keys().next().unwrap(),
        &connections,
        &mut visited,
    );
    let r = visited.len() * (connections.len() - visited.len());
    println!("{}", r);
}
