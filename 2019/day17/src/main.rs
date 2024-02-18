use std::{collections::HashMap, fs};

struct Machine {
    memory: Vec<i64>,
    input1: i64,
    input1_used: bool,
    i: usize,
    relative_base: i64,
    ii: usize,
}

impl Machine {
    fn new(memory: &Vec<i64>, input1: i64) -> Self {
        Machine {
            memory: memory.to_owned(),
            input1,
            input1_used: true,
            i: 0,
            relative_base: 0,
            ii: 0,
        }
    }

    fn ensure_memory(&mut self, len: usize) {
        if self.memory.len() <= len {
            self.memory.extend(vec![0; len - self.memory.len() + 1]);
        }
    }

    fn run(&mut self, input2: &str) -> Option<i64> {
        loop {
            self.ensure_memory(self.i);
            let code = self.memory[self.i];

            let opcode = code % 100;
            if opcode == 99 {
                self.i += 1;
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
                        out!(1, input2.chars().nth(self.ii).unwrap() as i64);
                        self.ii += 1;
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

fn find_new_dir(x: i32, y: i32, dir: (i32, i32), grid: &HashMap<(i32, i32), i64>) -> Option<char> {
    for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let nx = x + d.0;
        let ny = y + d.1;
        if let Some(35) = grid.get(&(nx, ny)) {
            let new_dir = match (dir, d) {
                ((0, -1), (0, -1)) => continue,
                ((0, -1), (0, 1)) => continue,
                ((0, -1), (1, 0)) => 'R',
                ((0, -1), (-1, 0)) => 'L',

                ((0, 1), (0, 1)) => continue,
                ((0, 1), (0, -1)) => continue,
                ((0, 1), (-1, 0)) => 'R',
                ((0, 1), (1, 0)) => 'L',

                ((-1, 0), (-1, 0)) => continue,
                ((-1, 0), (1, 0)) => continue,
                ((-1, 0), (0, -1)) => 'R',
                ((-1, 0), (0, 1)) => 'L',

                ((1, 0), (1, 0)) => continue,
                ((1, 0), (-1, 0)) => continue,
                ((1, 0), (0, 1)) => 'R',
                ((1, 0), (0, -1)) => 'L',

                _ => panic!(),
            };
            return Some(new_dir);
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut memory = input
        .trim()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut grid = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    let mut dir_x = 0;
    let mut dir_y = -1;

    let mut robot_pos = (0, 0);

    let mut robot = Machine::new(&memory, 0);
    loop {
        if let Some(o) = robot.run("") {
            if o != 35 && o != 46 && o != 10 {
                println!("{}", o);
                robot_pos = (x, y);
            }
            match o {
                10 => {
                    y += 1;
                    x = 0;
                }
                _ => {
                    grid.insert((x, y), o);
                    x += 1;
                }
            }
        } else {
            break;
        }
    }

    let mut sum = 0;
    for (k, &v) in &grid {
        if v == 35 {
            let x = k.0;
            let y = k.1;
            let mut c = 0;
            for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + d.0;
                let ny = y + d.1;
                if let Some(35) = grid.get(&(nx, ny)) {
                    c += 1;
                }
            }
            if c == 4 {
                sum += x * y;
            }
        }
    }

    println!("{}", sum);

    println!("{:?}", robot_pos);
    let c = (*grid.get(&robot_pos).unwrap() as u8) as char;
    println!("{}", c);

    x = robot_pos.0;
    y = robot_pos.1;
    let mut dir = (0, -1);
    let mut s = String::new();
    while let Some(dir_instr) = find_new_dir(x, y, dir, &grid) {
        dir = match (dir, dir_instr) {
            ((0, 1), 'R') => (-1, 0),
            ((0, -1), 'R') => (1, 0),
            ((1, 0), 'R') => (0, 1),
            ((-1, 0), 'R') => (0, -1),

            ((0, 1), 'L') => (1, 0),
            ((0, -1), 'L') => (-1, 0),
            ((1, 0), 'L') => (0, -1),
            ((-1, 0), 'L') => (0, 1),

            _ => panic!(),
        };

        let mut nx = x + dir.0;
        let mut ny = y + dir.1;
        let mut steps = 0;
        while let Some(35) = grid.get(&(nx, ny)) {
            x = nx;
            y = ny;
            nx += dir.0;
            ny += dir.1;
            steps += 1;
        }

        if !s.is_empty() {
            s.push(',');
        }
        s.push(dir_instr);
        s.push(',');
        s.push_str(&steps.to_string());
    }

    println!("{}", s);

    // => "R,4,L,10,L,10,L,8,R,12,R,10,R,4,R,4,L,10,L,10,L,8,R,12,R,10,R,4,R,4,L,10,L,10,L,8,L,8,R,10,R,4,L,8,R,12,R,10,R,4,L,8,L,8,R,10,R,4,R,4,L,10,L,10,L,8,L,8,R,10,R,4";

    let movement = "A,B,A,B,A,C,B,C,A,C";

    let a = "R,4,L,10,L,10";
    let b = "L,8,R,12,R,10,R,4";
    let c = "L,8,L,8,R,10,R,4";

    let mut input = String::new();
    input.push_str(movement);
    input.push('\n');
    input.push_str(a);
    input.push('\n');
    input.push_str(b);
    input.push('\n');
    input.push_str(c);
    input.push('\n');
    input.push('n');
    input.push('\n');

    robot.memory[0] = 2;
    let mut last_output = 0;
    while let Some(o) = robot.run(&input) {
        last_output = o;
    }

    println!("{}", last_output);
}
