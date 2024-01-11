use std::fs;

enum Instruction {
    Set(usize, Value),
    Sub(usize, Value),
    Mul(usize, Value),
    Jnz(Value, Value),
}

enum Value {
    Reg(usize),
    Const(i64),
}

struct Program<'a> {
    instructions: &'a [Instruction],
    pointer: i64,
    registers: Vec<i64>,
}

impl<'a> Program<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Program {
            instructions,
            pointer: 0,
            registers: vec![0; 26],
        }
    }

    fn to_val(&self, v: &Value) -> i64 {
        match v {
            Value::Reg(r) => self.registers[*r],
            Value::Const(c) => *c,
        }
    }

    fn run(&mut self) -> usize {
        use Instruction::*;

        let mut multiplications = 0;

        while self.pointer >= 0 && (self.pointer as usize) < self.instructions.len() {
            let instr = &self.instructions[self.pointer as usize];

            match instr {
                Set(r, v) => {
                    self.registers[*r] = self.to_val(v);
                }
                Sub(r, v) => {
                    self.registers[*r] -= self.to_val(v);
                }
                Mul(r, v) => {
                    self.registers[*r] *= self.to_val(v);
                    multiplications += 1;
                }
                Jnz(x, y) => {
                    if self.to_val(x) != 0 {
                        self.pointer += self.to_val(y);
                        continue;
                    }
                }
            }

            self.pointer += 1;
        }

        multiplications
    }
}

fn to_reg(s: &str) -> usize {
    (s.as_bytes()[0] - b'a') as usize
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let instructions = input
        .lines()
        .map(|l| {
            let p = l.split(' ').collect::<Vec<_>>();
            let p1 = if let Ok(v) = p[1].parse::<i64>() {
                Value::Const(v)
            } else {
                Value::Reg(to_reg(p[1]))
            };
            let p2 = if p.len() > 2 {
                Some(if let Ok(v) = p[2].parse::<i64>() {
                    Value::Const(v)
                } else {
                    Value::Reg(to_reg(p[2]))
                })
            } else {
                None
            };
            match p[0] {
                "set" => Instruction::Set(to_reg(p[1]), p2.unwrap()),
                "sub" => Instruction::Sub(to_reg(p[1]), p2.unwrap()),
                "mul" => Instruction::Mul(to_reg(p[1]), p2.unwrap()),
                "jnz" => Instruction::Jnz(p1, p2.unwrap()),
                _ => panic!(),
            }
        })
        .collect::<Vec<_>>();

    // part 1
    let mut p = Program::new(&instructions);
    let multiplications = p.run();
    println!("{}", multiplications);

    // part 2 ...
    // first get all instructions dealing with b and c
    let filtered_instructions = instructions
        .into_iter()
        .filter(|i| {
            matches!(
                i,
                Instruction::Set(1, _)
                    | Instruction::Set(2, _)
                    | Instruction::Sub(1, _)
                    | Instruction::Sub(2, _)
                    | Instruction::Mul(1, _)
                    | Instruction::Mul(2, _)
            )
        })
        .collect::<Vec<_>>();

    // assumption: the last instruction should be the one incrementing b
    let Some(Instruction::Sub(1, step)) = filtered_instructions.iter().last()
        else { panic!("Last instruction does not decrease register 'b'") };
    let Value::Const(step) = step else { panic!("Parameter of last instruction is not const") };
    let step = (-step) as usize;

    // run filtered instructions (without the last one) to get the values of
    // registers b and c
    let mut p = Program::new(&filtered_instructions[..filtered_instructions.len() - 1]);
    p.run();
    let b = p.registers[1];
    let c = p.registers[2];

    // This is basically what the assembler code is doing, translated
    // to Rust and optimized, so it's not so naive. The code iterates through
    // all values between b and c (with the given step size) and counts how
    // many values are not prime.
    let non_primes = (b..=c)
        .step_by(step)
        .filter(|v| (2i64..((*v as f64).sqrt() + 1.0).floor() as i64).any(|i| v % i == 0))
        .count();
    println!("{}", non_primes);
}
