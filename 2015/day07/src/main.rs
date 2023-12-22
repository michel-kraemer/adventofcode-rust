use std::{collections::HashMap, fs};

fn eval<'a>(
    instructions: &HashMap<&'a str, &'a str>,
    wire: &'a str,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(c) = cache.get(wire) {
        return *c;
    }

    let r = wire.parse().unwrap_or_else(|_| {
        let instr = instructions[wire];
        if instr.contains("AND") {
            let ops = instr.split(" ").collect::<Vec<_>>();
            eval(instructions, ops[0], cache) & eval(instructions, ops[2], cache)
        } else if instr.contains("OR") {
            let ops = instr.split(" ").collect::<Vec<_>>();
            eval(instructions, ops[0], cache) | eval(instructions, ops[2], cache)
        } else if instr.starts_with("NOT") {
            let ops = instr.split(" ").collect::<Vec<_>>();
            !eval(instructions, ops[1], cache)
        } else if instr.contains("LSHIFT") {
            let ops = instr.split(" ").collect::<Vec<_>>();
            eval(instructions, ops[0], cache) << ops[2].parse::<u16>().unwrap()
        } else if instr.contains("RSHIFT") {
            let ops = instr.split(" ").collect::<Vec<_>>();
            eval(instructions, ops[0], cache) >> ops[2].parse::<u16>().unwrap()
        } else {
            eval(instructions, instr, cache)
        }
    });

    cache.insert(wire, r);

    r
}

fn run(b_override: Option<&str>) -> u16 {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut cache: HashMap<&str, u16> = HashMap::new();
    let mut instructions = HashMap::new();
    for l in input.lines() {
        let (instr, output) = l.split_once(" -> ").unwrap();
        instructions.insert(output, instr);
    }

    if let Some(bo) = b_override {
        instructions.insert("b", bo);
    }

    let r = eval(&instructions, "a", &mut cache);
    println!("{}", r);

    r
}

fn main() {
    // part 1
    let a = run(None);

    // part 2
    run(Some(&a.to_string()));
}
