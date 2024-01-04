use std::{collections::HashMap, fs};

enum Weight {
    Is(usize),
    ShouldBe(usize),
}

fn find_weight(
    tree: &HashMap<&str, Vec<&str>>,
    root: &str,
    weights: &HashMap<&str, usize>,
) -> Weight {
    let mut balanced_weights = Vec::new();
    let mut sum = 0;
    for c in &tree[root] {
        let cw = find_weight(tree, c, weights);
        match cw {
            Weight::ShouldBe(_) => return cw,
            Weight::Is(w) => {
                balanced_weights.push((w, c));
                sum += w;
            }
        }
    }

    if !balanced_weights.is_empty() {
        balanced_weights.sort();
        if balanced_weights[0].0 != balanced_weights[1].0 {
            let diff = balanced_weights[1].0 - balanced_weights[0].0;
            let cw = weights[balanced_weights[0].1];
            return Weight::ShouldBe(cw + diff);
        } else if balanced_weights[balanced_weights.len() - 2].0
            != balanced_weights[balanced_weights.len() - 1].0
        {
            let diff = balanced_weights[balanced_weights.len() - 1].0
                - balanced_weights[balanced_weights.len() - 2].0;
            let cw = weights[balanced_weights[balanced_weights.len() - 1].1];
            return Weight::ShouldBe(cw - diff);
        }
    }

    Weight::Is(weights[root] + sum)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut reverse_tree: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut weights: HashMap<&str, usize> = HashMap::new();

    input.lines().for_each(|l| {
        let p = l.split(" -> ").collect::<Vec<_>>();

        let (name, weight) = p[0].split_once(' ').unwrap();
        let weight = weight[1..weight.len() - 1].parse::<usize>().unwrap();

        let dests = if p.len() == 1 {
            vec![]
        } else {
            p[1].split(", ").collect::<Vec<_>>()
        };

        reverse_tree.entry(name).or_insert_with(Default::default);
        for d in &dests {
            reverse_tree.entry(d).or_default().push(name);
        }

        tree.insert(name, dests);
        weights.insert(name, weight);
    });

    let root = reverse_tree.into_iter().find(|p| p.1.is_empty()).unwrap().0;

    // part 1
    println!("{}", root);

    // part 2
    let Weight::ShouldBe(r) = find_weight(&tree, root, &weights) else { panic!(); };
    println!("{}", r);
}
