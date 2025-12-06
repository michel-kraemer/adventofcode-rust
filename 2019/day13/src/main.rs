use crossterm::{cursor, style, terminal, ExecutableCommand};
use std::error::Error;
use std::io::stdout;
use std::{env, fs, thread, time};

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

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut memory = input
        .trim()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // play for free
    memory[0] = 2;

    // should the game be visualized on the terminal?
    let visualize = env::var("AOC_VISUALIZE").is_ok();

    let mut stdout = stdout();
    let pos = if visualize {
        // make space on screen and reset cursor
        stdout.execute(terminal::ScrollUp(25))?;
        stdout.execute(cursor::MoveTo(0, cursor::position()?.1 - 25))?;

        // hide cursor
        stdout.execute(cursor::Hide)?;

        cursor::position()?
    } else {
        (0u16, 0u16)
    };

    let mut robot = Machine::new(&memory, 0);
    let mut block_tiles = 0;
    let mut score = 0;

    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        // steer towards the ball
        let joystick = match paddle_x.cmp(&ball_x) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Greater => -1,
            std::cmp::Ordering::Equal => 0,
        };

        if let Some(x) = robot.run(joystick) {
            if let Some(y) = robot.run(joystick) {
                if let Some(tpe) = robot.run(joystick) {
                    if x == -1 && y == 0 {
                        score = tpe;
                    } else {
                        let c = match tpe {
                            0 => ' ',
                            1 => '█',
                            2 => {
                                block_tiles += 1;
                                '▪'
                            }
                            3 => {
                                paddle_x = x;
                                '—'
                            }
                            4 => {
                                ball_x = x;
                                '○'
                            }
                            _ => panic!(),
                        };
                        if visualize {
                            stdout.execute(cursor::MoveTo(pos.0 + x as u16, pos.1 + y as u16))?;
                            stdout.execute(style::Print(c))?;
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }

        if visualize {
            thread::sleep(time::Duration::from_millis(1));
        }
    }

    if visualize {
        stdout.execute(cursor::MoveTo(0, 25))?;
        stdout.execute(cursor::Show)?;
    }

    // part 1
    println!("{}", block_tiles);

    // part 2
    println!("{}", score);

    Ok(())
}
