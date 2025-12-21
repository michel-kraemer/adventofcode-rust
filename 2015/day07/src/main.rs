use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operand {
    Literal(u16),
    Register(usize),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Literal(u16),
    Move(usize),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
    Lshift(Operand, usize),
    Rshift(Operand, usize),
    Unknown,
}

fn to_register(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut result = (bytes[0] - b'a' + 1) as usize;
    if bytes.len() > 1 {
        result *= 27;
        result += (bytes[1] - b'a' + 1) as usize;
    }
    result
}

fn eval_operand(wires: &mut [Instruction], operand: Operand) -> u16 {
    match operand {
        Operand::Literal(l) => l,
        Operand::Register(r) => eval_register(wires, r),
    }
}

fn eval_register(wires: &mut [Instruction], register: usize) -> u16 {
    let r = match wires[register] {
        Instruction::Literal(a) => return a,
        Instruction::Move(a) => eval_register(wires, a),
        Instruction::And(a, b) => eval_operand(wires, a) & eval_operand(wires, b),
        Instruction::Or(a, b) => eval_operand(wires, a) | eval_operand(wires, b),
        Instruction::Not(a) => !eval_operand(wires, a),
        Instruction::Lshift(a, i) => eval_operand(wires, a) << i,
        Instruction::Rshift(a, i) => eval_operand(wires, a) >> i,
        Instruction::Unknown => panic!("Invalid input"),
    };
    wires[register] = Instruction::Literal(r);
    r
}

fn parse_operand(o: &str) -> Operand {
    if let Ok(literal) = o.parse::<u16>() {
        Operand::Literal(literal)
    } else {
        Operand::Register(to_register(o))
    }
}

fn main() {
    let mut wires = vec![Instruction::Unknown; 27 * 27];

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    for l in input.lines() {
        let (instr, output) = l.split_once(" -> ").unwrap();
        let parts = instr.split_ascii_whitespace().collect::<Vec<_>>();
        let instr = if parts.len() == 1 {
            let op = parse_operand(parts[0]);
            match op {
                Operand::Literal(l) => Instruction::Literal(l),
                Operand::Register(r) => Instruction::Move(r),
            }
        } else if parts.len() == 2 {
            Instruction::Not(parse_operand(parts[1]))
        } else if parts[1] == "AND" {
            Instruction::And(parse_operand(parts[0]), parse_operand(parts[2]))
        } else if parts[1] == "OR" {
            Instruction::Or(parse_operand(parts[0]), parse_operand(parts[2]))
        } else if parts[1] == "LSHIFT" {
            Instruction::Lshift(parse_operand(parts[0]), parts[2].parse().unwrap())
        } else {
            Instruction::Rshift(parse_operand(parts[0]), parts[2].parse().unwrap())
        };
        let output = to_register(output);
        wires[output] = instr;
    }

    // part 1
    let mut wires_part1 = wires.clone();
    let a = eval_register(&mut wires_part1, to_register("a"));
    println!("{a}");

    // part 2
    wires[to_register("b")] = Instruction::Literal(a);
    let a = eval_register(&mut wires, to_register("a"));
    println!("{a}");
}
