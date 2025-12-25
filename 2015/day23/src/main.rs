use std::fs;

#[derive(Clone, Copy)]
enum Instruction {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(isize),
    Jie(usize, isize),
    Jio(usize, isize),
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let instructions = input
        .lines()
        .map(|i| {
            let (instr, params) = i.split_once(" ").unwrap();
            match instr {
                "hlf" => Instruction::Hlf((params.as_bytes()[0] - b'a') as usize),
                "tpl" => Instruction::Tpl((params.as_bytes()[0] - b'a') as usize),
                "inc" => Instruction::Inc((params.as_bytes()[0] - b'a') as usize),
                "jmp" => Instruction::Jmp(params.trim_start_matches("+").parse::<isize>().unwrap()),
                "jie" => {
                    let params = params.split_once(", ").unwrap();
                    Instruction::Jie(
                        (params.0.as_bytes()[0] - b'a') as usize,
                        params.1.trim_start_matches("+").parse::<isize>().unwrap(),
                    )
                }
                "jio" => {
                    let params = params.split_once(", ").unwrap();
                    Instruction::Jio(
                        (params.0.as_bytes()[0] - b'a') as usize,
                        params.1.trim_start_matches("+").parse::<isize>().unwrap(),
                    )
                }
                _ => panic!("Unknown instruction"),
            }
        })
        .collect::<Vec<_>>();

    for part1 in [true, false] {
        let mut registers: [u64; 2] = [0; 2];

        if !part1 {
            registers[0] = 1;
        }

        let mut pointer = 0usize;
        while pointer < instructions.len() {
            let i = instructions[pointer];
            match i {
                Instruction::Hlf(r) => {
                    registers[r] /= 2;
                    pointer += 1;
                }
                Instruction::Tpl(r) => {
                    registers[r] *= 3;
                    pointer += 1;
                }
                Instruction::Inc(r) => {
                    registers[r] += 1;
                    pointer += 1;
                }
                Instruction::Jmp(offset) => pointer = pointer.checked_add_signed(offset).unwrap(),
                Instruction::Jie(r, offset) => {
                    if registers[r].is_multiple_of(2) {
                        pointer = pointer.checked_add_signed(offset).unwrap()
                    } else {
                        pointer += 1;
                    }
                }
                Instruction::Jio(r, offset) => {
                    if registers[r] == 1 {
                        pointer = pointer.checked_add_signed(offset).unwrap()
                    } else {
                        pointer += 1;
                    }
                }
            }
        }

        println!("{}", registers[1]);
    }
}
