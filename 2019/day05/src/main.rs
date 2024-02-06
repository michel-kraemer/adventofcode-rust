use std::fs;

fn run(memory: &[i64], input: i64) -> i64 {
    let mut memory = memory.to_owned();

    let mut output = 0;

    let mut i = 0;
    loop {
        let code = memory[i];

        let opcode = code % 100;
        if opcode == 99 {
            return output;
        }

        /// get value of the parameter at index $pi
        macro_rules! inp {
            ($pi:literal) => {{
                let mode = (code / 100 / 10i64.pow($pi - 1)) % 10;
                if mode == 0 {
                    memory[memory[i + $pi] as usize]
                } else {
                    memory[i + $pi]
                }
            }};
        }

        /// write $v to the memory location specified by the parameter at index $pi
        macro_rules! out {
            ($pi:literal, $v:expr) => {{
                let o = memory[i + $pi] as usize;
                memory[o] = $v;
            }};
        }

        match opcode {
            1 => {
                // add
                out!(3, inp!(1) + inp!(2));
                i += 4;
            }

            2 => {
                // mul
                out!(3, inp!(1) * inp!(2));
                i += 4;
            }

            3 => {
                // read input
                out!(1, input);
                i += 2;
            }

            4 => {
                // write output
                output = inp!(1);
                i += 2;
            }

            5 => {
                // jump if true
                if inp!(1) != 0 {
                    i = inp!(2) as usize;
                } else {
                    i += 3;
                }
            }

            6 => {
                // jump if false
                if inp!(1) == 0 {
                    i = inp!(2) as usize;
                } else {
                    i += 3;
                }
            }

            7 => {
                // less than
                out!(3, (inp!(1) < inp!(2)) as i64);
                i += 4;
            }

            8 => {
                // equals
                out!(3, (inp!(1) == inp!(2)) as i64);
                i += 4;
            }

            _ => {
                panic!("Unknown opcode: {}", opcode)
            }
        }
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
