use std::fs;

use serde_json::Value;

fn is_red(v: &Value) -> bool {
    match v {
        Value::String(s) => s == "red",
        _ => false,
    }
}

fn visit(v: &Value, part1: bool) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(children) => children.iter().map(|c| visit(c, part1)).sum(),
        Value::Object(props) => {
            if part1 || !props.iter().any(|(_, v)| is_red(v)) {
                props.iter().map(|(_, v)| visit(v, part1)).sum()
            } else {
                0
            }
        }
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let v: Value = serde_json::from_str(&input).unwrap();
        println!("{}", visit(&v, part1));
    }
}
