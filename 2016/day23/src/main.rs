use std::{
    fmt::{Display, Formatter},
    fs,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Input {
    Value(i64),
    Register(usize),
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Value(v) => write!(f, "{v}"),
            Input::Register(r) => write!(f, "{}", (b'a' + (*r as u8)) as char),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Cpy { input: Input, register: Input },
    Inc { register: Input, offset: i64 },
    Jnz { input: Input, offset: Input },
    Tgl { input: Input },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Cpy { input, register } => write!(f, "cpy {input} {register}"),
            Instruction::Inc { register, offset } => {
                if *offset == 1 {
                    write!(f, "inc {register}")
                } else if *offset == -1 {
                    write!(f, "dec {register}")
                } else {
                    write!(f, "inc {register} {offset}")
                }
            }
            Instruction::Jnz { input, offset } => write!(f, "jnz {input} {offset}"),
            Instruction::Tgl { input } => write!(f, "tgl {input}"),
        }
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
                    register: parse_input(parts.next().unwrap()),
                },
                "inc" => Instruction::Inc {
                    register: parse_input(parts.next().unwrap()),
                    offset: 1,
                },
                "dec" => Instruction::Inc {
                    register: parse_input(parts.next().unwrap()),
                    offset: -1,
                },
                "jnz" => Instruction::Jnz {
                    input: parse_input(parts.next().unwrap()),
                    offset: parse_input(parts.next().unwrap()),
                },
                "tgl" => Instruction::Tgl {
                    input: parse_input(parts.next().unwrap()),
                },
                _ => panic!("Unknown instruction: {instruction}"),
            }
        })
        .collect::<Vec<_>>();

    for part1 in [true, false] {
        let mut instructions = instructions.clone();
        let mut registers: [i64; 4] = [0; 4];

        if part1 {
            registers[0] = 7;
        } else {
            registers[0] = 12;
        }

        let mut ip = 0;
        while ip < instructions.len() {
            match instructions[ip] {
                Instruction::Cpy { input, register } => {
                    if let Input::Register(register) = register {
                        match input {
                            Input::Value(v) => registers[register] = v,
                            Input::Register(r) => registers[register] = registers[r],
                        }
                    }
                    ip += 1;
                }

                Instruction::Inc { register, offset } => {
                    if let Input::Register(register) = register {
                        registers[register] += offset;
                    }
                    ip += 1;
                }

                Instruction::Jnz { input, offset } => {
                    let v = match input {
                        Input::Value(v) => v,
                        Input::Register(r) => registers[r],
                    };
                    if v != 0 {
                        let offset = match offset {
                            Input::Value(value) => value,
                            Input::Register(register) => registers[register],
                        } as isize;

                        // JIT-optimization:
                        if offset == -2
                            && ip > 1
                            && let Instruction::Inc {
                                register: Input::Register(r1),
                                offset: inc_offset1,
                            } = instructions[ip - 2]
                            && let Instruction::Inc {
                                register: Input::Register(r2),
                                offset: inc_offset2,
                            } = instructions[ip - 1]
                        {
                            // quick inc
                            registers[r1] += v * inc_offset1;
                            registers[r2] += v * inc_offset2;
                            ip += 1;
                        } else if offset == -5
                            && ip > 4
                            // first instruction must be a cpy
                            && let Instruction::Cpy {
                                input: cpy_input,
                                register: Input::Register(cpy_dest),
                            } = instructions[ip - 5]
                            // second and third instructions must be inc/dec
                            && let Instruction::Inc {
                                register: Input::Register(inc1_r),
                                offset: inc_offset1,
                            } = instructions[ip - 4]
                            && let Instruction::Inc {
                                register: Input::Register(inc2_r),
                                offset: inc_offset2,
                            } = instructions[ip - 3]
                            // one of these two must be an inc and the other
                            // must be a dec
                            && inc_offset1 == -inc_offset2
                            // fourth instruction must be a jnz
                            && let Instruction::Jnz {
                                input: Input::Register(jnz_r),
                                offset: Input::Value(jnz_v),
                            } = instructions[ip - 2]
                            // this jnz must use the register that is decremented
                            && ((inc_offset1 == -1 && jnz_r == inc1_r)
                                || (inc_offset2 == -1 && jnz_r == inc2_r))
                            // and the jnz must have an offset of -2
                            && jnz_v == -2
                            // fifth instruction must be a dec
                            && let Instruction::Inc {
                                register: Input::Register(dec2_r),
                                offset: inc_offset3,
                            } = instructions[ip - 1]
                            && inc_offset3 == -1
                            // and this dec must affect our input
                            && let Input::Register(my_r) = input
                            && dec2_r == my_r
                        {
                            // quick multiplication as hinted in the problem
                            // statement
                            let cpy_source = match cpy_input {
                                Input::Value(v) => v,
                                Input::Register(r) => registers[r],
                            };
                            if inc_offset1 == -1 {
                                registers[inc2_r] += cpy_source * registers[my_r];
                            } else {
                                registers[inc1_r] += cpy_source * registers[my_r];
                            }
                            registers[cpy_dest] = 0;
                            registers[my_r] = 0;
                            ip += 1;
                        } else {
                            // execute the instruction normally
                            ip = ip.checked_add_signed(offset).unwrap();
                        }
                    } else {
                        ip += 1;
                    }
                }

                Instruction::Tgl { input } => {
                    let j = ip
                        + match input {
                            Input::Value(value) => value,
                            Input::Register(register) => registers[register],
                        } as usize;
                    if j < instructions.len() {
                        let target_instr = &mut instructions[j];
                        *target_instr = match target_instr {
                            Instruction::Cpy { input, register } => Instruction::Jnz {
                                input: *input,
                                offset: *register,
                            },
                            Instruction::Inc { register, offset } => Instruction::Inc {
                                register: *register,
                                offset: -*offset,
                            },
                            Instruction::Jnz { input, offset } => Instruction::Cpy {
                                input: *input,
                                register: *offset,
                            },
                            Instruction::Tgl { input } => Instruction::Inc {
                                register: *input,
                                offset: 1,
                            },
                        };
                    }
                    ip += 1;
                }
            }
        }

        println!("{}", registers[0]);
    }
}
