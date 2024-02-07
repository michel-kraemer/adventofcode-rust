use std::fs;

struct Machine {
    memory: Vec<i64>,
    input1: i64,
    input1_used: bool,
    i: usize,
}

impl Machine {
    fn new(memory: &Vec<i64>, input1: i64) -> Self {
        Machine {
            memory: memory.to_owned(),
            input1,
            input1_used: false,
            i: 0,
        }
    }

    fn run(&mut self, input2: i64) -> Option<i64> {
        loop {
            let code = self.memory[self.i];

            let opcode = code % 100;
            if opcode == 99 {
                return None;
            }

            /// get value of the parameter at index $pi
            macro_rules! inp {
                ($pi:literal) => {{
                    let mode = (code / 100 / 10i64.pow($pi - 1)) % 10;
                    if mode == 0 {
                        self.memory[self.memory[self.i + $pi] as usize]
                    } else {
                        self.memory[self.i + $pi]
                    }
                }};
            }

            /// write $v to the memory location specified by the parameter at index $pi
            macro_rules! out {
                ($pi:literal, $v:expr) => {{
                    let o = self.memory[self.i + $pi] as usize;
                    self.memory[o] = $v;
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

                _ => {
                    panic!("Unknown opcode: {}", opcode)
                }
            }
        }
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let memory = input
            .trim()
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut m = 0;
        for a in if part1 { 0..5 } else { 5..10 } {
            for b in if part1 { 0..5 } else { 5..10 } {
                if b == a {
                    continue;
                }
                for c in if part1 { 0..5 } else { 5..10 } {
                    if c == a || c == b {
                        continue;
                    }
                    for d in if part1 { 0..5 } else { 5..10 } {
                        if d == a || d == b || d == c {
                            continue;
                        }
                        for e in if part1 { 0..5 } else { 5..10 } {
                            if e == a || e == b || e == c || e == d {
                                continue;
                            }

                            let mut ma = Machine::new(&memory, a);
                            let mut mb = Machine::new(&memory, b);
                            let mut mc = Machine::new(&memory, c);
                            let mut md = Machine::new(&memory, d);
                            let mut me = Machine::new(&memory, e);

                            let mut fre = 0;
                            'outer: loop {
                                if let Some(ra) = ma.run(fre) {
                                    if let Some(rb) = mb.run(ra) {
                                        if let Some(rc) = mc.run(rb) {
                                            if let Some(rd) = md.run(rc) {
                                                if let Some(re) = me.run(rd) {
                                                    fre = re;
                                                } else {
                                                    break 'outer;
                                                }
                                            } else {
                                                break 'outer;
                                            }
                                        } else {
                                            break 'outer;
                                        }
                                    } else {
                                        break 'outer;
                                    }
                                } else {
                                    break 'outer;
                                }

                                if part1 {
                                    break;
                                }
                            }

                            m = m.max(fre);
                        }
                    }
                }
            }
        }

        println!("{}", m);
    }
}
