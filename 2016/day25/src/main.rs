use std::fs;

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Cpy,
    Inc,
    Dec,
    Jnz,
    Tgl,
    Out,
}

enum Param {
    Reg(usize),
    Num(i32),
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut instructions = input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let i = match parts.next().unwrap() {
                "cpy" => Instruction::Cpy,
                "inc" => Instruction::Inc,
                "dec" => Instruction::Dec,
                "jnz" => Instruction::Jnz,
                "tgl" => Instruction::Tgl,
                "out" => Instruction::Out,
                _ => unreachable!(),
            };
            let params = parts
                .map(|p| {
                    if let Ok(n) = p.parse::<i32>() {
                        Param::Num(n)
                    } else {
                        Param::Reg((p.as_bytes()[0] - b'a') as usize)
                    }
                })
                .collect::<Vec<_>>();
            (i, params)
        })
        .collect::<Vec<_>>();

    // we assume 10000 is enough to check if the sequence is infinite
    let max_steps = 10000;

    for i in 0..i32::MAX {
        let mut registers = vec![0i32; 26];

        registers[0] = i;

        let mut pointer = 0;
        let mut steps = 0;
        let mut current = 1i32;
        while steps < max_steps && pointer < instructions.len() {
            let (i, params) = &instructions[pointer];
            match i {
                Instruction::Cpy => {
                    let v = match params[0] {
                        Param::Num(n) => n,
                        Param::Reg(r) => registers[r],
                    };
                    if let Param::Reg(r) = params[1] {
                        registers[r] = v;
                    }
                }
                Instruction::Inc => {
                    let Param::Reg(r) = params[0] else { unreachable!() };
                    registers[r] += 1;
                }
                Instruction::Dec => {
                    let Param::Reg(r) = params[0] else { unreachable!() };
                    registers[r] -= 1;
                }
                Instruction::Jnz => {
                    let v = match params[0] {
                        Param::Num(n) => n,
                        Param::Reg(r) => registers[r],
                    };
                    if v != 0 {
                        let d = match params[1] {
                            Param::Num(n) => n,
                            Param::Reg(r) => registers[r],
                        };
                        if d == -2
                            && pointer > 1
                            && instructions[pointer - 2].0 == Instruction::Inc
                            && instructions[pointer - 1].0 == Instruction::Dec
                        {
                            let Param::Reg(r2) = instructions[pointer - 2].1[0] else { unreachable!() };
                            let Param::Reg(r1) = instructions[pointer - 1].1[0] else { unreachable!() };
                            registers[r2] += v;
                            registers[r1] -= v;
                            pointer += 1;
                        } else if d == -2
                            && pointer > 1
                            && instructions[pointer - 2].0 == Instruction::Dec
                            && instructions[pointer - 1].0 == Instruction::Inc
                        {
                            let Param::Reg(r2) = instructions[pointer - 2].1[0] else { unreachable!() };
                            let Param::Reg(r1) = instructions[pointer - 1].1[0] else { unreachable!() };
                            registers[r2] -= v;
                            registers[r1] += v;
                            pointer += 1;
                        } else {
                            pointer = (pointer as i32 + d) as usize;
                        }
                        continue;
                    }
                }
                Instruction::Tgl => {
                    let v = match params[0] {
                        Param::Num(n) => n,
                        Param::Reg(r) => registers[r],
                    };
                    let j = (pointer as i32 + v) as usize;
                    if j < instructions.len() {
                        let target_instr = &instructions[j].0;
                        let target_params = &instructions[j].1;
                        if target_params.len() == 1 {
                            if *target_instr == Instruction::Inc {
                                instructions[j].0 = Instruction::Dec;
                            } else {
                                instructions[j].0 = Instruction::Inc;
                            }
                        } else if *target_instr == Instruction::Jnz {
                            instructions[j].0 = Instruction::Cpy;
                        } else {
                            instructions[j].0 = Instruction::Jnz;
                        }
                    }
                }
                Instruction::Out => {
                    let v = match params[0] {
                        Param::Num(n) => n,
                        Param::Reg(r) => registers[r],
                    };
                    if (current == 0 && v == 1) || (current == 1 && v == 0) {
                        current = v;
                        steps += 1;
                    } else {
                        break;
                    }
                }
            }

            pointer += 1;
        }

        if steps == max_steps {
            println!("{}", i);
            break;
        }
    }
}
