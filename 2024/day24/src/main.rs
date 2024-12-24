use std::collections::{HashMap, VecDeque};
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
#[allow(unused)]
fn write_dot_file(gates: &[Gate<'_>]) {
    let mut writer = BufWriter::new(File::create("graph.dot").unwrap());

    let logic_per_wire = gates
        .iter()
        .map(|g| (g.out, g.logic))
        .collect::<HashMap<_, _>>();

    writer.write_all(b"digraph {{\n").unwrap();

    // write all nodes and colorize them according to their gate logic
    for lpw in &logic_per_wire {
        let color = match lpw.1 {
            Logic::And => "blue",
            Logic::Or => "yellow",
            Logic::Xor => "red",
        };
        writer
            .write_all(format!("{} [style=filled,fillcolor={}];", lpw.0, color).as_bytes())
            .unwrap();
    }

    // write all edges
    for g in gates {
        writer
            .write_all(format!("{} -> {};", g.a, g.out).as_bytes())
            .unwrap();
        writer
            .write_all(format!("{} -> {};", g.b, g.out).as_bytes())
            .unwrap();
    }

    writer.write_all(b"}}").unwrap();
}

fn run<'a>(
    wires: &HashMap<&'a str, bool>,
    gates: &[Gate<'a>],
    renames: &HashMap<&'a str, &'a str>,
) -> u64 {
    let mut wires = wires.clone();

    let mut queue = VecDeque::from_iter(gates);

    while !queue.is_empty() {
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
    let x = get_value(&wires, "x");
    let y = get_value(&wires, "y");

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

    let total2 = run(&wires, &gates, &renames);
    if total2 == x + y {
        let mut broken_wires = renames.keys().copied().collect::<Vec<_>>();
        broken_wires.sort();
        println!("{}", broken_wires.join(","));
    } else {
        panic!("Unsolvable");
    }
}
