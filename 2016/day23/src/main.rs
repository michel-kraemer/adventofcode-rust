use std::fs;

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Cpy,
    Inc,
    Dec,
    Jnz,
    Tgl,
}

fn reg(registers: &[i32], n: &str) -> i32 {
    registers[(n.as_bytes()[0] - b'a') as usize]
}

fn reg_mut<'a>(registers: &'a mut [i32], n: &str) -> &'a mut i32 {
    registers
        .get_mut((n.as_bytes()[0] - b'a') as usize)
        .unwrap()
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut instructions = input
            .lines()
            .map(|l| {
                let mut p = l.split(' ');
                let i = match p.next().unwrap() {
                    "cpy" => Instruction::Cpy,
                    "inc" => Instruction::Inc,
                    "dec" => Instruction::Dec,
                    "jnz" => Instruction::Jnz,
                    "tgl" => Instruction::Tgl,
                    _ => unreachable!(),
                };
                let params = p.collect::<Vec<_>>();
                (i, params)
            })
            .collect::<Vec<_>>();

        let mut registers = vec![0i32; 26];

        if part1 {
            registers[0] = 7;
        } else {
            registers[0] = 12;
        }

        let mut pointer = 0;
        while pointer < instructions.len() {
            let (i, params) = &instructions[pointer];
            match i {
                Instruction::Cpy => {
                    let v = if let Ok(n) = params[0].parse::<i32>() {
                        n
                    } else {
                        reg(&registers, params[0])
                    };
                    *reg_mut(&mut registers, params[1]) = v;
                }
                Instruction::Inc => *reg_mut(&mut registers, params[0]) += 1,
                Instruction::Dec => *reg_mut(&mut registers, params[0]) -= 1,
                Instruction::Jnz => {
                    let v = if let Ok(n) = params[0].parse::<i32>() {
                        n
                    } else {
                        reg(&registers, params[0])
                    };
                    if v != 0 {
                        let d = if let Ok(n) = params[1].parse::<i32>() {
                            n
                        } else {
                            reg(&registers, params[1])
                        };
                        if d == -2
                            && pointer > 1
                            && instructions[pointer - 2].0 == Instruction::Inc
                            && instructions[pointer - 1].0 == Instruction::Dec
                        {
                            *reg_mut(&mut registers, instructions[pointer - 2].1[0]) += v;
                            *reg_mut(&mut registers, instructions[pointer - 1].1[0]) -= v;
                            pointer += 1;
                        } else if d == -2
                            && pointer > 1
                            && instructions[pointer - 2].0 == Instruction::Dec
                            && instructions[pointer - 1].0 == Instruction::Inc
                        {
                            *reg_mut(&mut registers, instructions[pointer - 2].1[0]) -= v;
                            *reg_mut(&mut registers, instructions[pointer - 1].1[0]) += v;
                            pointer += 1;
                        } else {
                            pointer = (pointer as i32 + d) as usize;
                        }
                        continue;
                    }
                }
                Instruction::Tgl => {
                    let v = if let Ok(n) = params[0].parse::<i32>() {
                        n
                    } else {
                        reg(&registers, params[0])
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
            }

            pointer += 1;
        }

        println!("{}", registers[0]);
    }
}
