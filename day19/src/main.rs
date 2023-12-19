use std::{
    collections::{HashMap, VecDeque},
    fs,
    ops::RangeInclusive,
};

fn evaluate<'a, 'b>(
    p: HashMap<&'a str, RangeInclusive<usize>>,
    workflow: &'b Vec<&str>,
) -> Vec<(HashMap<&'a str, RangeInclusive<usize>>, &'b str)> {
    let mut p = p;
    let mut result = Vec::new();

    for r in workflow {
        if r.contains(":") {
            let (cond, dest) = r.split_once(":").unwrap();

            let (prop, rel, v) = if cond.contains(">") {
                let r = cond.split_once(">").unwrap();
                (r.0, '>', r.1.parse::<usize>().unwrap())
            } else {
                let r = cond.split_once("<").unwrap();
                (r.0, '<', r.1.parse::<usize>().unwrap())
            };

            let actual_prop = p.get(&prop).unwrap();
            if rel == '>' {
                if *actual_prop.start() > v {
                    // the whole range matches the condition
                    result.push((p, dest));
                    break;
                } else if *actual_prop.end() <= v {
                    // the whole range does not match the condition - skip these parts
                } else {
                    // split the range
                    let mut new_r = p.clone();
                    let nri = new_r.get_mut(prop).unwrap();
                    *nri = v + 1..=*nri.end();
                    result.push((new_r, dest));

                    let npi = p.get_mut(prop).unwrap();
                    *npi = *npi.start()..=v;
                }
            } else {
                if *actual_prop.end() < v {
                    result.push((p, dest));
                    break;
                } else if *actual_prop.start() >= v {
                    // nothing to do
                } else {
                    let mut new_r = p.clone();
                    let nri = new_r.get_mut(prop).unwrap();
                    *nri = *nri.start()..=v - 1;
                    result.push((new_r, dest));

                    let npi = p.get_mut(prop).unwrap();
                    *npi = v..=*npi.end();
                }
            }
        } else {
            result.push((p, *r));
            break;
        }
    }
    result
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let (rules, parts) = input.split_once("\n\n").unwrap();
        let rules = rules
            .lines()
            .map(|line| line[0..line.len() - 1].split_once("{").unwrap())
            .map(|(name, conditions)| (name, conditions.split(",").collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        let mut workflows = HashMap::new();
        for r in rules {
            workflows.insert(r.0, r.1);
        }

        let parts = if part1 {
            parts
                .lines()
                .map(|l| l[1..l.len() - 1].split(",").collect::<Vec<_>>())
                .map(|p| {
                    p.into_iter()
                        .map(|c| {
                            let res = c.split_once("=").unwrap();
                            let rng = res.1.parse::<usize>().unwrap();
                            (res.0, rng..=rng)
                        })
                        .collect::<HashMap<_, _>>()
                })
                .collect::<Vec<_>>()
        } else {
            vec![HashMap::from([
                ("x", 1..=4000),
                ("m", 1..=4000),
                ("a", 1..=4000),
                ("s", 1..=4000),
            ])]
        };

        let mut sum = 0;
        for p in parts {
            let mut current_workflows = VecDeque::new();
            current_workflows.push_back((p, "in"));
            while !current_workflows.is_empty() {
                let next = current_workflows.pop_front().unwrap();
                let new_workflows = evaluate(next.0, workflows.get(next.1).unwrap());
                for nw in new_workflows {
                    if nw.1 == "A" {
                        if part1 {
                            sum += nw.0.iter().map(|v| v.1.end()).sum::<usize>();
                        } else {
                            sum +=
                                nw.0.iter()
                                    .map(|v| v.1.end() - v.1.start() + 1)
                                    .reduce(|a, b| a * b)
                                    .unwrap();
                        }
                    } else if nw.1 == "R" {
                        // reject - do nothing
                    } else {
                        current_workflows.push_back(nw);
                    }
                }
            }
        }

        println!("{sum}");
    }
}
