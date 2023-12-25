use std::fs;

use serde_json::Value;

fn is_red(v: &Value) -> bool {
    match v {
        Value::String(s) => s == "red",
        _ => false
    }
}

fn visit(v: &Value, sum: &mut i64, part1: bool) {
    match v {
        Value::Null => {},
        Value::Bool(_) => {},
        Value::Number(n) => *sum += n.as_i64().unwrap(),
        Value::String(_) => {},
        Value::Array(children) => children.iter().for_each(|c| visit(c, sum, part1)),
        Value::Object(props) => {
            if part1 || !props.iter().any(|(_, v)| is_red(v)) {
                props.iter().for_each(|(_, v)| visit(v, sum, part1));
            }
        },
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let v: Value = serde_json::from_str(&input).unwrap();

        let mut sum = 0;
        visit(&v, &mut sum, part1);
        println!("{}", sum);
    }
}
