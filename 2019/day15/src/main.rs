use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Clone)]
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
            input1_used: true,
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

struct State {
    droid: Machine,
    x: i64,
    y: i64,
    steps: usize,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let memory = input
        .trim()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // part 1 - run BFS with multiple droids (or clones of it) until one of
    // them finds the oxygen
    let mut queue = VecDeque::new();
    let initial = State {
        droid: Machine::new(&memory, 0),
        x: 0,
        y: 0,
        steps: 0,
    };
    queue.push_back(initial);

    let mut seen = HashSet::new();
    seen.insert((0, 0));

    let mut empty_cells = HashSet::new();
    empty_cells.insert((0, 0));

    let mut ox = 0;
    let mut oy = 0;
    let mut os = 0;
    while let Some(s) = queue.pop_front() {
        for (input, d) in [(0, -1), (0, 1), (-1, 0), (1, 0)].into_iter().enumerate() {
            let nx = s.x + d.0;
            let ny = s.y + d.1;
            if !seen.contains(&(nx, ny)) {
                seen.insert((nx, ny));
                let mut nd = s.droid.clone();
                let o = nd.run(input as i64 + 1).unwrap();
                if o != 0 {
                    if o == 2 {
                        os = s.steps + 1;
                        ox = nx;
                        oy = ny;
                        // don't break here - we need to explore the whole map
                        // for part 2
                    }
                    empty_cells.insert((nx, ny));
                    queue.push_back(State {
                        droid: nd,
                        x: nx,
                        y: ny,
                        steps: s.steps + 1,
                    })
                }
            }
        }
    }

    println!("{}", os);

    // part 2 - perform a BFS from the position of the oxygen and count how
    // many steps it takes to reach each empty cell
    let mut queue = VecDeque::new();
    let mut max_steps = 0;
    queue.push_back((ox, oy, 0));
    empty_cells.remove(&(ox, oy));
    while let Some(p) = queue.pop_front() {
        max_steps = max_steps.max(p.2);
        for d in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let nx = p.0 + d.0;
            let ny = p.1 + d.1;
            if empty_cells.remove(&(nx, ny)) {
                queue.push_back((nx, ny, p.2 + 1));
            }
        }
    }

    println!("{}", max_steps);
}
