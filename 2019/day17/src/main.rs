use std::{collections::HashMap, fs};

struct Machine {
    memory: Vec<i64>,
    i: usize,
    relative_base: i64,
}

impl Machine {
    fn new(memory: &Vec<i64>) -> Self {
        Machine {
            memory: memory.to_owned(),
            i: 0,
            relative_base: 0,
        }
    }

    fn ensure_memory(&mut self, len: usize) {
        if self.memory.len() <= len {
            self.memory.extend(vec![0; len - self.memory.len() + 1]);
        }
    }

    fn run(&mut self, mut read_input: impl FnMut() -> i64) -> Option<i64> {
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
                    out!(1, read_input());
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

enum Turn {
    Left,
    Right,
}

struct Move(Turn, usize);

fn find_turn(
    x: i32,
    y: i32,
    dir: (i32, i32),
    grid: &HashMap<(i32, i32), u8>,
) -> Option<(Turn, (i32, i32))> {
    for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let nx = x + d.0;
        let ny = y + d.1;
        if let Some(b'#') = grid.get(&(nx, ny)) {
            if dir == d || dir == (-d.0, -d.1) {
                continue;
            }
            if dir == (d.1, -d.0) {
                return Some((Turn::Right, d));
            }
            if dir == (-d.1, d.0) {
                return Some((Turn::Left, d));
            }
            panic!("Invalid direction");
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let memory = input
        .trim()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // run machine and receive map
    let mut grid = HashMap::new();

    let mut x = 0;
    let mut y = 0;

    let mut robot_pos = (0, 0);

    let mut robot = Machine::new(&memory);
    while let Some(o) = robot.run(|| 0) {
        match o as u8 {
            b'\n' => {
                y += 1;
                x = 0;
            }
            b'<' | b'>' | b'^' | b'v' => {
                robot_pos = (x, y);
                grid.insert((x, y), o as u8);
                x += 1;
            }
            b'#' | b'.' => {
                grid.insert((x, y), o as u8);
                x += 1;
            }
            _ => panic!("Invalid output {}", o),
        }
    }

    // part 1: count intersections in map
    let mut total_intersections = 0;
    for ((x, y), _) in grid.iter().filter(|(_, &v)| v == b'#') {
        let mut c = 0;
        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + d.0;
            let ny = y + d.1;
            if let Some(b'#') = grid.get(&(nx, ny)) {
                c += 1;
            }
        }
        if c == 4 {
            total_intersections += x * y;
        }
    }

    println!("{}", total_intersections);

    // part 2 ...
    // start at the robot's position and trace the scaffolding
    x = robot_pos.0;
    y = robot_pos.1;
    let mut dir = match grid.get(&robot_pos) {
        Some(b'^') => (0, -1),
        Some(b'v') => (0, 1),
        Some(b'>') => (1, 0),
        Some(b'<') => (-1, 0),
        _ => panic!("Invalid robot position"),
    };
    let mut trace = Vec::new();
    while let Some((turn, new_dir)) = find_turn(x, y, dir, &grid) {
        dir = new_dir;

        // trace until next turn (or end of scaffolding)
        let mut nx = x + dir.0;
        let mut ny = y + dir.1;
        let mut steps = 0;
        while let Some(b'#') = grid.get(&(nx, ny)) {
            x = nx;
            y = ny;
            nx += dir.0;
            ny += dir.1;
            steps += 1;
        }

        trace.push(Move(turn, steps));
    }

    // => "R,4,L,10,L,10,L,8,R,12,R,10,R,4,R,4,L,10,L,10,L,8,R,12,R,10,R,4,R,4,L,10,L,10,L,8,L,8,R,10,R,4,L,8,R,12,R,10,R,4,L,8,L,8,R,10,R,4,R,4,L,10,L,10,L,8,L,8,R,10,R,4";

    let main = "A,B,A,B,A,C,B,C,A,C";

    let a = "R,4,L,10,L,10";
    let b = "L,8,R,12,R,10,R,4";
    let c = "L,8,L,8,R,10,R,4";
    let video = "n";

    let input = [main, a, b, c, video, ""].join("\n");

    // wake up robot
    robot.memory[0] = 2;

    // Provide instructions to robot and wait until it has visited every
    // scaffolding piece. Read final output (= collected dust).
    let mut last_output = 0;
    let mut input_iter = input.chars().map(|c| c as i64);
    while let Some(o) = robot.run(|| input_iter.next().unwrap()) {
        last_output = o;
    }

    println!("{}", last_output);
}
