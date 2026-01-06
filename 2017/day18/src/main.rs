use std::{collections::VecDeque, fs};

enum Instruction {
    Snd(Value),
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Rcv(usize),
    Jgz(Value, Value),
}

enum Value {
    Reg(usize),
    Const(i64),
}

struct Program<'a> {
    instructions: &'a Vec<Instruction>,
    pointer: i64,
    registers: [i64; 26],
    sent_messages: usize,
}

impl<'a> Program<'a> {
    fn new(instructions: &'a Vec<Instruction>, initial_p: i64) -> Self {
        let mut r = Program {
            instructions,
            pointer: 0,
            registers: [0; 26],
            sent_messages: 0,
        };
        r.registers[to_reg("p")] = initial_p;
        r
    }

    fn to_val(&self, v: &Value) -> i64 {
        match v {
            Value::Reg(r) => self.registers[*r],
            Value::Const(c) => *c,
        }
    }

    fn run(&mut self, mut inbox: VecDeque<i64>) -> VecDeque<i64> {
        use Instruction::*;

        let mut outbox = VecDeque::new();

        while self.pointer >= 0 && (self.pointer as usize) < self.instructions.len() {
            let instr = &self.instructions[self.pointer as usize];

            match instr {
                Snd(v) => {
                    outbox.push_back(self.to_val(v));
                    self.sent_messages += 1;
                }
                Set(r, v) => {
                    self.registers[*r] = self.to_val(v);
                }
                Add(r, v) => {
                    self.registers[*r] += self.to_val(v);
                }
                Mul(r, v) => {
                    self.registers[*r] *= self.to_val(v);
                }
                Mod(r, v) => {
                    self.registers[*r] %= self.to_val(v);
                }
                Rcv(r) => {
                    if let Some(v) = inbox.pop_front() {
                        self.registers[*r] = v;
                    } else {
                        break;
                    }
                }
                Jgz(x, y) => {
                    if self.to_val(x) > 0 {
                        self.pointer += self.to_val(y);
                        continue;
                    }
                }
            }

            self.pointer += 1;
        }

        outbox
    }
}

fn to_reg(s: &str) -> usize {
    (s.as_bytes()[0] - b'a') as usize
}

fn to_value(s: &str) -> Value {
    if let Ok(v) = s.parse::<i64>() {
        Value::Const(v)
    } else {
        Value::Reg(to_reg(s))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let instructions = input
        .lines()
        .map(|l| {
            let mut p = l.split_ascii_whitespace();
            match p.next().unwrap() {
                "snd" => Instruction::Snd(to_value(p.next().unwrap())),
                "set" => Instruction::Set(to_reg(p.next().unwrap()), to_value(p.next().unwrap())),
                "add" => Instruction::Add(to_reg(p.next().unwrap()), to_value(p.next().unwrap())),
                "mul" => Instruction::Mul(to_reg(p.next().unwrap()), to_value(p.next().unwrap())),
                "mod" => Instruction::Mod(to_reg(p.next().unwrap()), to_value(p.next().unwrap())),
                "rcv" => Instruction::Rcv(to_reg(p.next().unwrap())),
                "jgz" => Instruction::Jgz(to_value(p.next().unwrap()), to_value(p.next().unwrap())),
                _ => panic!(),
            }
        })
        .collect::<Vec<_>>();

    // part 1
    let mut q = VecDeque::new();
    let mut p0 = Program::new(&instructions, 0);
    q = p0.run(q);
    println!("{}", q.iter().last().unwrap());

    // part 2
    let mut p1 = Program::new(&instructions, 1);

    loop {
        let p0_waiting = q.is_empty();
        q = p1.run(q);
        if p0_waiting && q.is_empty() {
            break;
        }
        q = p0.run(q);
    }

    println!("{}", p1.sent_messages);
}
