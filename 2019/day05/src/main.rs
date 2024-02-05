use std::fs;

fn run(memory: &[i64], input: i64) -> i64 {
    let mut memory = memory.to_owned();

    let mut output = 0;

    let mut i = 0;
    loop {
        let mut code = memory[i];

        let opcode = code % 100;
        code /= 100;
        let param1_mode = code % 10;
        code /= 10;
        let param2_mode = code % 10;

        if opcode == 99 {
            return output;
        }

        if opcode == 1 {
            let a = if param1_mode == 0 {
                memory[memory[i + 1] as usize]
            } else {
                memory[i + 1]
            };
            let b = if param2_mode == 0 {
                memory[memory[i + 2] as usize]
            } else {
                memory[i + 2]
            };
            let c = memory[i + 3] as usize;
            i += 4;
            memory[c] = a + b;
        } else if opcode == 2 {
            let a = if param1_mode == 0 {
                memory[memory[i + 1] as usize]
            } else {
                memory[i + 1]
            };
            let b = if param2_mode == 0 {
                memory[memory[i + 2] as usize]
            } else {
                memory[i + 2]
            };
            let c = memory[i + 3] as usize;
            i += 4;
            memory[c] = a * b;
        } else if opcode == 3 {
            let a = memory[i + 1] as usize;
            memory[a] = input;
            i += 2;
        } else if opcode == 4 {
            output = if param1_mode == 0 {
                memory[memory[i + 1] as usize]
            } else {
                memory[i + 1]
            };
            i += 2;
        } else if opcode == 5 {
            let a = if param1_mode == 0 {
                memory[memory[i + 1] as usize]
            } else {
                memory[i + 1]
            };
            let b = if param2_mode == 0 {
                memory[memory[i + 2] as usize]
            } else {
                memory[i + 2]
            };
            if a != 0 {
                i = b as usize;
            } else {
                i += 3;
            }
        } else if opcode == 6 {
            let a = if param1_mode == 0 {
                memory[memory[i + 1] as usize]
            } else {
                memory[i + 1]
            };
            let b = if param2_mode == 0 {
                memory[memory[i + 2] as usize]
            } else {
                memory[i + 2]
            };
            if a == 0 {
                i = b as usize;
            } else {
                i += 3;
            }
        } else if opcode == 7 {
            let a = if param1_mode == 0 {
                memory[memory[i + 1] as usize]
            } else {
                memory[i + 1]
            };
            let b = if param2_mode == 0 {
                memory[memory[i + 2] as usize]
            } else {
                memory[i + 2]
            };
            let c = memory[i + 3] as usize;
            if a < b {
                memory[c] = 1;
            } else {
                memory[c] = 0;
            }
            i += 4;
        } else if opcode == 8 {
            let a = if param1_mode == 0 {
                memory[memory[i + 1] as usize]
            } else {
                memory[i + 1]
            };
            let b = if param2_mode == 0 {
                memory[memory[i + 2] as usize]
            } else {
                memory[i + 2]
            };
            let c = memory[i + 3] as usize;
            if a == b {
                memory[c] = 1;
            } else {
                memory[c] = 0;
            }
            i += 4;
        } else {
            panic!("Unknown opcode: {}", opcode)
        };
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let memory = input
        .trim()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // part 1
    println!("{}", run(&memory, 1));

    // part 2
    println!("{}", run(&memory, 5));
}
