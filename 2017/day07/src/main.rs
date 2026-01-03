use std::{collections::hash_map::Entry, fs};

use rustc_hash::{FxBuildHasher, FxHashMap};

struct Node<'a> {
    weight: u64,
    children: Vec<&'a str>,
    is_root: bool,
}

enum Weight {
    /// A node's own weight and the sum of its weight and its children
    Is(u64, u64),

    /// The expected weight of a node to balance the tree
    ShouldBe(u64),
}

/// Find the weight of the node with the given `name`. Either return
/// [Weight::Is] with the node's own weight and the sum of its weight and its
/// children, or return [Weight::ShouldBe] if the node is only node with the
/// wrong weight and its actual value should be something different.
fn find_weight(name: &str, tree: &FxHashMap<&str, Node>) -> Weight {
    let node = &tree[name];
    let mut sum = 0;
    let mut wa = 0;
    let mut wb = 0;
    let mut wa_recursive = 0;
    let mut wb_recursive = 0;

    // Iterate through all children, compute their weights. If there is a node
    // whose weight differs from its siblings, wa_recursive and wb_recursive
    // will both be non-zero and will have different values.
    for &c in &node.children {
        match find_weight(c, tree) {
            s @ Weight::ShouldBe(_) => return s,
            Weight::Is(cw, cw_recursive) => {
                if wa_recursive == 0 {
                    wa = cw;
                    wa_recursive = cw_recursive;
                } else if cw_recursive != wa_recursive {
                    wb = cw;
                    wb_recursive = cw_recursive;
                }
                sum += cw_recursive;
            }
        }
    }

    if wb != 0 {
        // determine which weight is the incorrect one
        return if (node.children.len() as u64 - 1) * wa_recursive + wb_recursive == sum {
            // wb is the wrong weight
            Weight::ShouldBe(
                wb.checked_add_signed(wa_recursive as i64 - wb_recursive as i64)
                    .unwrap(),
            )
        } else {
            // wa is the wrong weight
            Weight::ShouldBe(
                wa.checked_add_signed(wb_recursive as i64 - wa_recursive as i64)
                    .unwrap(),
            )
        };
    }

    Weight::Is(node.weight, node.weight + sum)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // parse input and build tree
    let mut tree: FxHashMap<&str, Node> = FxHashMap::with_capacity_and_hasher(2048, FxBuildHasher);
    for l in input.lines() {
        let (from, children) = if let Some((from, to)) = l.split_once(" -> ") {
            let mut children = Vec::new();
            for c in to.split(", ") {
                tree.entry(c)
                    .and_modify(|n| n.is_root = false)
                    .or_insert_with(|| Node {
                        weight: 0,
                        children: Vec::new(),
                        is_root: false,
                    });
                children.push(c);
            }
            (from, children)
        } else {
            (l, Vec::new())
        };

        let (name, weight) = from.split_once(' ').unwrap();
        let weight = weight[1..weight.len() - 1].parse::<u64>().unwrap();

        let e = tree.entry(name);
        if let Entry::Occupied(mut e) = e {
            let n = e.get_mut();
            n.weight = weight;
            n.children = children;
        } else {
            e.insert_entry(Node {
                weight,
                children,
                is_root: true,
            });
        }
    }

    let root = tree.iter().find(|(_, n)| n.is_root).unwrap().0;

    // part 1
    println!("{root}");

    // part 2
    let Weight::ShouldBe(r) = find_weight(root, &tree) else {
        unreachable!();
    };
    println!("{r}");
}
