use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Input {
    Value(u64),
    Register(usize),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Cpy { input: Input, register: usize },
    Inc { register: usize },
    Dec { register: usize },
    Jnz { input: Input, offset: isize },
}

fn parse_register(s: &str) -> usize {
    match s {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Invalid register: {s}"),
    }
}

fn parse_input(s: &str) -> Input {
    match s {
        "a" => Input::Register(0),
        "b" => Input::Register(1),
        "c" => Input::Register(2),
        "d" => Input::Register(3),
        _ => Input::Value(s.parse().unwrap()),
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let instructions = input
        .lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();
            let instruction = parts.next().unwrap();
            match instruction {
                "cpy" => Instruction::Cpy {
                    input: parse_input(parts.next().unwrap()),
                    register: parse_register(parts.next().unwrap()),
                },
                "inc" => Instruction::Inc {
                    register: parse_register(parts.next().unwrap()),
                },
                "dec" => Instruction::Dec {
                    register: parse_register(parts.next().unwrap()),
                },
                "jnz" => Instruction::Jnz {
                    input: parse_input(parts.next().unwrap()),
                    offset: parts.next().unwrap().parse().unwrap(),
                },
                _ => panic!("Unknown instruction: {instruction}"),
            }
        })
        .collect::<Vec<_>>();

    for part1 in [true, false] {
        let mut registers: [u64; 4] = [0; 4];

        if !part1 {
            registers[2] = 1;
        }

        let mut ip = 0;
        while ip < instructions.len() {
            match instructions[ip] {
                Instruction::Cpy { input, register } => {
                    match input {
                        Input::Value(v) => registers[register] = v,
                        Input::Register(r) => registers[register] = registers[r],
                    }
                    ip += 1;
                }
                Instruction::Inc { register } => {
                    registers[register] += 1;
                    ip += 1;
                }
                Instruction::Dec { register } => {
                    registers[register] -= 1;
                    ip += 1;
                }
                Instruction::Jnz { input, offset } => {
                    let v = match input {
                        Input::Value(v) => v,
                        Input::Register(r) => registers[r],
                    };
                    if v != 0 {
                        ip = ip.checked_add_signed(offset).unwrap();
                    } else {
                        ip += 1;
                    }
                }
            }
        }

        println!("{}", registers[0]);
    }
}
