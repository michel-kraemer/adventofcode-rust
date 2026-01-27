use std::fs;

#[derive(Clone, Copy)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn apply(i: Opcode, a: usize, b: usize, c: usize, registers: &mut [usize]) {
    use Opcode::*;

    registers[c] = match i {
        Addr => registers[a] + registers[b],
        Addi => registers[a] + b,
        Mulr => registers[a] * registers[b],
        Muli => registers[a] * b,
        Banr => registers[a] & registers[b],
        Bani => registers[a] & b,
        Borr => registers[a] | registers[b],
        Bori => registers[a] | b,
        Setr => registers[a],
        Seti => a,
        Gtir => (a > registers[b]) as usize,
        Gtri => (registers[a] > b) as usize,
        Gtrr => (registers[a] > registers[b]) as usize,
        Eqir => (a == registers[b]) as usize,
        Eqri => (registers[a] == b) as usize,
        Eqrr => (registers[a] == registers[b]) as usize,
    };
}

fn run(
    program: &[(Opcode, usize, usize, usize)],
    max_steps: usize,
    pointer_register: usize,
    registers: &mut [usize; 6],
) {
    let mut pointer = 0;
    let mut steps = 0;
    while steps < max_steps && pointer < program.len() {
        registers[pointer_register] = pointer;
        apply(
            program[pointer].0,
            program[pointer].1,
            program[pointer].2,
            program[pointer].3,
            registers,
        );
        pointer = registers[pointer_register];
        pointer += 1;
        steps += 1;
    }
}

fn divisor_sum(val: usize) -> usize {
    let mut cur = 1;
    let mut result = 0;
    while cur * cur <= val {
        if val.is_multiple_of(cur) {
            if val / cur == cur {
                // perfect square
                result += cur;
                cur += 1;
            } else {
                // all divisors greater than 1 come in pairs
                // (e.g. 6 / 2 == 3 and 6 / 3 == 2)
                result += cur;
                result += val / cur;
            }
        }
        cur += 1;
    }
    result
}

fn main() {
    // We basically implement what the program does in Rust: Look for all
    // divisors of registers[4] and sum them up

    use Opcode::*;

    // parse
    let mut pointer_register = 0;
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let program = input
        .lines()
        .filter_map(|l| {
            if l.starts_with('#') {
                pointer_register = l[4..].parse::<usize>().unwrap();
                None
            } else {
                let p = l.split_ascii_whitespace().collect::<Vec<_>>();
                let opcode = match p[0] {
                    "addr" => Addr,
                    "addi" => Addi,
                    "mulr" => Mulr,
                    "muli" => Muli,
                    "banr" => Banr,
                    "bani" => Bani,
                    "borr" => Borr,
                    "bori" => Bori,
                    "setr" => Setr,
                    "seti" => Seti,
                    "gtir" => Gtir,
                    "gtri" => Gtri,
                    "gtrr" => Gtrr,
                    "eqir" => Eqir,
                    "eqri" => Eqri,
                    "eqrr" => Eqrr,
                    _ => panic!(),
                };
                Some((
                    opcode,
                    p[1].parse::<usize>().unwrap(),
                    p[2].parse::<usize>().unwrap(),
                    p[3].parse::<usize>().unwrap(),
                ))
            }
        })
        .collect::<Vec<_>>();

    // part 1 ...
    // let the program run for a while to get the value of register[4]
    let mut registers = [0; 6];
    run(&program, 75, pointer_register, &mut registers);
    println!("{}", divisor_sum(registers[4]));

    // part 2 ...
    let mut registers = [0; 6];
    registers[0] = 1;
    run(&program, 75, pointer_register, &mut registers);
    println!("{}", divisor_sum(registers[4]));
}
