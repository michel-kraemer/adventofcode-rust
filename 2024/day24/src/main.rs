use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{self, File};
use std::io::{BufWriter, Write};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Logic {
    And,
    Or,
    Xor,
}

struct Gate<'a> {
    logic: Logic,
    a: &'a str,
    b: &'a str,
    out: &'a str,
}

/// Take all wires that start with the given prefix, sort them by name, and
/// decode them to an integer
fn get_value(wires: &HashMap<&str, bool>, prefix: &str) -> u64 {
    // find wires that start with prefix
    let mut wires_to_decode = wires
        .iter()
        .filter(|(name, _)| name.starts_with(prefix))
        .collect::<Vec<_>>();

    // sort wires by name
    wires_to_decode.sort();

    // 00 is the least significant bit
    wires_to_decode.reverse();

    // decode
    let mut result = 0u64;
    for (_, v) in wires_to_decode {
        result <<= 1;
        if *v {
            result += 1;
        }
    }

    result
}

/// For debugging: Write system into a dot file
///
/// Convert dot file into svg:
///
///     dot -Tsvg graph.dot -o graph.svg
fn write_dot_file(gates: &[Gate<'_>], filename: &str, disconnect_carry_bits: bool) {
    let mut writer = BufWriter::new(File::create(filename).unwrap());

    let logic_per_wire = gates
        .iter()
        .map(|g| (g.out, g.logic))
        .collect::<HashMap<_, _>>();

    writer.write_all(b"digraph {\n").unwrap();

    // write all nodes and colorize them according to their gate logic
    for lpw in &logic_per_wire {
        let color = match lpw.1 {
            Logic::And => "blue",
            Logic::Or => "yellow",
            Logic::Xor => "red",
        };
        writer
            .write_all(format!("{} [style=filled,fillcolor={}];\n", lpw.0, color).as_bytes())
            .unwrap();
    }

    // write all edges
    for g in gates {
        if !disconnect_carry_bits || g.logic != Logic::Or || g.out.starts_with("z") {
            writer
                .write_all(format!("{} -> {};\n", g.a, g.out).as_bytes())
                .unwrap();
            writer
                .write_all(format!("{} -> {};\n", g.b, g.out).as_bytes())
                .unwrap();
        }
    }

    writer.write_all(b"}").unwrap();
}

fn run<'a>(
    wires: &HashMap<&'a str, bool>,
    gates: &[Gate<'a>],
    renames: &HashMap<&'a str, &'a str>,
) -> u64 {
    let mut wires = wires.clone();

    let mut queue = VecDeque::from_iter(gates);

    while let Some(g) = queue.pop_front() {
        let out = renames.get(&g.out).unwrap_or(&g.out);

        // get values of a and b or push them back into the queue if
        // their values aren't available yet
        let Some(&a) = wires.get(g.a) else {
            queue.push_back(g);
            continue;
        };
        let Some(&b) = wires.get(g.b) else {
            queue.push_back(g);
            continue;
        };

        let v = match g.logic {
            Logic::And => a && b,
            Logic::Or => a || b,
            Logic::Xor => a != b,
        };

        wires.insert(out, v);
    }

    get_value(&wires, "z")
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let (wires, gates) = input.split_once("\n\n").unwrap();
    let wires = wires
        .lines()
        .map(|w| {
            let s = w.split_once(": ").unwrap();
            (s.0, s.1 == "1")
        })
        .collect::<HashMap<_, _>>();

    let gates = gates
        .lines()
        .map(|l| {
            let (left, out) = l.split_once(" -> ").unwrap();
            let s = left.split_whitespace().collect::<Vec<_>>();
            let logic = match s[1] {
                "AND" => Logic::And,
                "OR" => Logic::Or,
                "XOR" => Logic::Xor,
                _ => panic!("Invalid gate: {}", left),
            };
            Gate {
                logic,
                a: s[0],
                b: s[2],
                out,
            }
        })
        .collect::<Vec<_>>();

    // part 1
    let total1 = run(&wires, &gates, &HashMap::new());
    println!("{}", total1);

    // part 2
    // try to find broken nodes by checking common patterns (this is basically
    // what I've done visually)
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for g in &gates {
        edges.entry(g.a).or_default().push(g.out);
        edges.entry(g.b).or_default().push(g.out);
    }

    let mut broken_nodes = HashSet::new();
    for g in &gates {
        // z nodes must be XOR (except for the last one, z45)
        if g.out.starts_with("z") && g.out != "z45" && g.logic != Logic::Xor {
            broken_nodes.insert(g.out);
        }
        // z nodes must not be inputs of other nodes
        if g.a.starts_with("z") {
            broken_nodes.insert(g.a);
        }
        if g.b.starts_with("z") {
            broken_nodes.insert(g.b);
        }

        // inputs of XOR nodes (except for z nodes) must be x and y nodes
        if g.logic == Logic::Xor
            && !g.out.starts_with("z")
            && !((g.a.starts_with("x") && g.b.starts_with("y"))
                || (g.a.starts_with("y") && g.b.starts_with("x")))
        {
            broken_nodes.insert(g.out);
        }

        // XOR nodes (except z nodes) must always be input of exactly two
        // other nodes
        if g.logic == Logic::Xor && !g.out.starts_with("z") && edges[g.out].len() != 2 {
            broken_nodes.insert(g.out);
        }

        // AND nodes must always be input of exactly one other node (except
        // the very first one wired to x00 and y00)
        if g.logic == Logic::And
            && !g.out.starts_with("z")
            && edges[g.out].len() != 1
            && !((g.a == "x00" && g.b == "y00") || (g.a == "y00" && g.b == "x00"))
        {
            broken_nodes.insert(g.out);
        }
    }

    // this should be the answer:
    let mut broken_nodes = broken_nodes.into_iter().collect::<Vec<_>>();
    broken_nodes.sort();
    println!("{}", broken_nodes.join(","));

    // hard-coded answer for my puzzle input (just to check if the code above works)
    let renames = HashMap::from([
        // 1
        ("kqh", "ddn"),
        ("ddn", "kqh"),
        // 2
        ("z09", "nnf"),
        ("nnf", "z09"),
        // 3
        ("z20", "nhs"),
        ("nhs", "z20"),
        // 4
        ("z34", "wrc"),
        ("wrc", "z34"),
    ]);

    let x = get_value(&wires, "x");
    let y = get_value(&wires, "y");
    let total2 = run(&wires, &gates, &renames);
    if total2 == x + y {
        let mut broken_nodes2 = renames.keys().copied().collect::<Vec<_>>();
        broken_nodes2.sort();
        assert_eq!(
            broken_nodes, broken_nodes2,
            "This check is just for my puzzle input"
        );
    } else {
        panic!("Unsolvable");
    }

    // visualize broken graph
    write_dot_file(&gates, "graph_broken.dot", false);

    // visualize broken graph and disconnect carry bits from their inputs (this
    // is what I used for debugging during the contest)
    write_dot_file(&gates, "graph_broken_disconnected.dot", true);

    // visualize fixed graph
    let fixed_gates = gates
        .iter()
        .map(|g| {
            let out = renames.get(&g.out).unwrap_or(&g.out);
            Gate { out, ..*g }
        })
        .collect::<Vec<_>>();
    write_dot_file(&fixed_gates, "graph_fixed.dot", false);
    write_dot_file(&fixed_gates, "graph_fixed_disconnected.dot", true);
}
