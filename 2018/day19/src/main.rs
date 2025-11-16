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
    registers: &mut [usize],
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

fn main() {
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
                let p = l.split_whitespace().collect::<Vec<_>>();
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

    // part 1
    let mut registers = vec![0; 6];
    run(&program, usize::MAX, pointer_register, &mut registers);
    println!("{}", registers[0]);

    // part 2 ...

    // let the program run for a while, so we get the value of register[4]
    let mut registers = vec![0; 6];
    registers[0] = 1;
    run(&program, 100, pointer_register, &mut registers);

    // this is basically what the program does:
    // look for all factors of registers[4] and sum them up
    let sum = (1..=registers[4])
        .filter(|&i| registers[4].is_multiple_of(i))
        .sum::<usize>();
    println!("{}", sum);
}
