use std::fs;

struct Machine {
    memory: Vec<i64>,
    input1: i64,
    input1_used: bool,
    i: usize,
    relative_base: i64,
}

impl Machine {
    fn new(memory: &Vec<i64>, input1: i64) -> Self {
        Machine {
            memory: memory.to_owned(),
            input1,
            input1_used: false,
            i: 0,
            relative_base: 0,
        }
    }

    fn ensure_memory(&mut self, len: usize) {
        if self.memory.len() <= len {
            self.memory.extend(vec![0; len - self.memory.len() + 1]);
        }
    }

    fn run(&mut self, input2: i64) -> Option<i64> {
        loop {
            self.ensure_memory(self.i);
            let code = self.memory[self.i];

            let opcode = code % 100;
            if opcode == 99 {
                return None;
            }

            /// get value of the parameter at index $pi
            macro_rules! inp {
                ($pi:literal) => {{
                    let mode = (code / 100 / 10i64.pow($pi - 1)) % 10;
                    let j = if mode == 0 {
                        self.memory[self.i + $pi] as usize
                    } else if mode == 1 {
                        self.i + $pi
                    } else {
                        (self.relative_base + self.memory[self.i + $pi]) as usize
                    };
                    self.ensure_memory(j);
                    self.memory[j]
                }};
            }

            /// write $v to the memory location specified by the parameter at index $pi
            macro_rules! out {
                ($pi:literal, $v:expr) => {{
                    let mode = (code / 100 / 10i64.pow($pi - 1)) % 10;
                    let o = self.memory[self.i + $pi] as usize;
                    let j = if mode == 0 {
                        o
                    } else if mode == 2 {
                        (self.relative_base + o as i64) as usize
                    } else {
                        panic!("Invalid mode for output instruction")
                    };
                    self.ensure_memory(j);
                    self.memory[j] = $v;
                }};
            }

            match opcode {
                1 => {
                    // add
                    out!(3, inp!(1) + inp!(2));
                    self.i += 4;
                }

                2 => {
                    // mul
                    out!(3, inp!(1) * inp!(2));
                    self.i += 4;
                }

                3 => {
                    // read input
                    if self.input1_used {
                        out!(1, input2);
                    } else {
                        out!(1, self.input1);
                        self.input1_used = true;
                    }
                    self.i += 2;
                }

                4 => {
                    // write output
                    let output = inp!(1);
                    self.i += 2;
                    return Some(output);
                }

                5 => {
                    // jump if true
                    if inp!(1) != 0 {
                        self.i = inp!(2) as usize;
                    } else {
                        self.i += 3;
                    }
                }

                6 => {
                    // jump if false
                    if inp!(1) == 0 {
                        self.i = inp!(2) as usize;
                    } else {
                        self.i += 3;
                    }
                }

                7 => {
                    // less than
                    out!(3, (inp!(1) < inp!(2)) as i64);
                    self.i += 4;
                }

                8 => {
                    // equals
                    out!(3, (inp!(1) == inp!(2)) as i64);
                    self.i += 4;
                }

                9 => {
                    self.relative_base += inp!(1);
                    self.i += 2;
                }

                _ => {
                    panic!("Unknown opcode: {}", opcode)
                }
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
    let mut m1 = Machine::new(&memory, 1);
    println!("{}", m1.run(0).unwrap());

    // part 2
    let mut m2 = Machine::new(&memory, 2);
    println!("{}", m2.run(0).unwrap());
}
