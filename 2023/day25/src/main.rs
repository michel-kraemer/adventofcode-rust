use std::{
    collections::{HashMap, VecDeque},
    fs,
};

/// Brandes' algorithm calculates the betweenness centrality of graph nodes.
/// It basically finds the shortest path between each pair of nodes and counts
/// how often each node is passed through. This algorithm is very efficient
/// and runs in O(n*m) time, where n is the number of vertices in the graph and
/// m is the number of edges.
///
/// See: Ulrik Brandes (2001) A Faster Algorithm for Betweenness Centrality, The
/// Journal of Mathematical Sociology, 25:2, 163-177, DOI: 10.1080/0022250X.2001.9990249
fn brandes(edges: &[Vec<usize>]) -> Vec<f64> {
    let mut result = vec![0.0; edges.len()];
    let mut delta = vec![0.0; edges.len()];
    let mut sigma = vec![0; edges.len()];
    let mut dist = vec![0; edges.len()];
    let mut prev = vec![Vec::new(); edges.len()];
    let mut queue = VecDeque::new();

    for s in 0..edges.len() {
        let mut stack = Vec::new();
        prev.fill(Vec::new());

        sigma.fill(0);
        sigma[s] = 1;

        dist.fill(usize::MAX);
        dist[s] = 0;

        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);

            for &w in &edges[v] {
                // w found for the first time?
                if dist[w] == usize::MAX {
                    dist[w] = dist[v] + 1;
                    queue.push_back(w);
                }

                // shortest path to w via v?
                if dist[w] == dist[v] + 1 {
                    sigma[w] += sigma[v];
                    prev[w].push(v);
                }
            }
        }

        delta.fill(0.0);

        // stack returns vertices in order of non-increasing distance from s
        for i in (1..stack.len()).rev() {
            let w = stack[i];
            let p = &prev[w];
            for &u in p {
                delta[u] += sigma[u] as f64 / sigma[w] as f64 * (1.0 + delta[w]);
                if u != s {
                    result[w] += delta[w];
                }
            }
        }
    }

    result
}

/// Perform DFS starting with node i and count all nodes until no more node
/// can be visited
fn count_cluster(edges: &[Vec<usize>], i: usize, seen: &mut [bool]) -> usize {
    seen[i] = true;
    let mut count = 1;
    for &j in &edges[i] {
        if !seen[j] {
            count += count_cluster(edges, j, seen);
        }
    }
    count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // parse input and create bi-directional graph
    let mut vertices = HashMap::new();
    let mut edges: Vec<Vec<usize>> = Vec::new();
    for l in lines {
        let (from, tos) = l.split_once(": ").unwrap();
        let tos = tos.split_whitespace().collect::<Vec<_>>();
        for to in tos {
            let fil = vertices.len();
            let fi = *vertices.entry(from).or_insert_with(|| {
                edges.push(Vec::new());
                fil
            });
            let til = vertices.len();
            let ti = *vertices.entry(to).or_insert_with(|| {
                edges.push(Vec::new());
                til
            });
            if !edges[fi].contains(&ti) {
                edges[fi].push(ti);
            }
            if !edges[ti].contains(&fi) {
                edges[ti].push(fi);
            }
        }
    }

    // use Brandes' algorithm to calculate betweenness centrality
    let betweenness = brandes(&edges);

    // sort by centrality and get top 6 nodes
    let mut sorted_betweenness = betweenness.into_iter().enumerate().collect::<Vec<_>>();
    sorted_betweenness.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let top = &sorted_betweenness[0..6];

    // remove connections between top nodes
    for (t, _) in top {
        for (u, _) in top {
            if let Some(i) = edges[*t].iter().position(|x| x == u) {
                edges[*t].remove(i);
            }
        }
    }

    // count vertices in an arbitrary cluster and multiply it by the number
    // of vertices not belonging to this cluster
    let mut seen = vec![false; edges.len()];
    let count = count_cluster(&edges, 0, &mut seen);
    println!("{}", count * (edges.len() - count));
}
