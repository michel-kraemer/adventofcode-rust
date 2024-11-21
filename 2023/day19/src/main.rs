use std::{collections::HashMap, fs, ops::RangeInclusive};

enum Command<'a> {
    Accept,
    Reject,
    Goto(&'a str),
}

enum Operator {
    Gt,
    Lt,
}

struct Condition<'a> {
    field: &'a str,
    op: Operator,
    value: i64,
    command: Command<'a>,
}

struct Rule<'a> {
    conditions: Vec<Condition<'a>>,
    otherwise: Command<'a>,
}

fn apply(
    rules: &HashMap<&str, Rule>,
    cur: &str,
    mut ratings: Vec<(&str, RangeInclusive<i64>)>,
) -> i64 {
    let r = rules.get(cur).unwrap();
    let mut total = 0;
    for c in &r.conditions {
        let i = ratings.iter().position(|r| r.0 == c.field).unwrap();
        let range = &ratings[i].1;
        let (new_range, updated_range) = match c.op {
            Operator::Gt => {
                if *range.end() <= c.value {
                    (None, None)
                } else if *range.start() > c.value {
                    (Some(range.clone()), None)
                } else {
                    (
                        Some(c.value + 1..=*range.end()),
                        Some(*range.start()..=c.value),
                    )
                }
            }
            Operator::Lt => {
                if *range.start() >= c.value {
                    (None, None)
                } else if *range.end() < c.value {
                    (Some(range.clone()), None)
                } else {
                    (
                        Some(*range.start()..=c.value - 1),
                        Some(c.value..=*range.end()),
                    )
                }
            }
        };

        if let Some(new_range) = new_range {
            let mut cl = ratings.clone();
            cl[i].1 = new_range;

            match c.command {
                Command::Accept => {
                    total += cl
                        .iter()
                        .map(|r| r.1.end() - r.1.start() + 1)
                        .product::<i64>();
                }
                Command::Reject => {
                    // skip
                }
                Command::Goto(rule) => {
                    total += apply(rules, rule, cl);
                }
            }

            if let Some(updated_range) = updated_range {
                ratings[i].1 = updated_range;
            } else {
                return total;
            }
        }
    }

    match r.otherwise {
        Command::Accept => {
            total += ratings
                .iter()
                .map(|r| r.1.end() - r.1.start() + 1)
                .product::<i64>();
        }
        Command::Reject => {
            // skip
        }
        Command::Goto(rule) => {
            total += apply(rules, rule, ratings);
        }
    }

    total
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::new();
    for w in workflows.lines() {
        let (name, rest) = w.split_once("{").unwrap();
        let rule_parts = rest[0..rest.len() - 1].split(",").collect::<Vec<_>>();

        let mut conditions = Vec::new();
        for r in &rule_parts[0..rule_parts.len() - 1] {
            let (condition, command) = r.split_once(":").unwrap();
            let field = &condition[0..1];
            let op = match &condition[1..2] {
                ">" => Operator::Gt,
                "<" => Operator::Lt,
                _ => unreachable!(),
            };
            let value = condition[2..].parse::<i64>().unwrap();
            let command = match command {
                "A" => Command::Accept,
                "R" => Command::Reject,
                _ => Command::Goto(command),
            };
            conditions.push(Condition {
                field,
                op,
                value,
                command,
            });
        }

        let otherwise = match rule_parts[rule_parts.len() - 1] {
            "A" => Command::Accept,
            "R" => Command::Reject,
            dest => Command::Goto(dest),
        };

        rules.insert(
            name,
            Rule {
                conditions,
                otherwise,
            },
        );
    }

    // part 1
    let mut total_part1 = 0;
    for p in parts.lines() {
        let ratings = p[1..p.len() - 1]
            .split(",")
            .map(|r| r.split_once("=").unwrap())
            .map(|r| (r.0.to_string(), r.1.parse::<i64>().unwrap()))
            .collect::<Vec<_>>();
        let r = apply(
            &rules,
            "in",
            ratings
                .iter()
                .map(|r| (r.0.as_str(), r.1..=r.1))
                .collect::<Vec<_>>(),
        );
        if r > 0 {
            total_part1 += ratings.iter().map(|r| r.1).sum::<i64>();
        }
    }
    println!("{}", total_part1);

    // part 2
    let total_part2 = apply(
        &rules,
        "in",
        vec![
            ("x", 1..=4000),
            ("m", 1..=4000),
            ("a", 1..=4000),
            ("s", 1..=4000),
        ],
    );
    println!("{}", total_part2);
}
