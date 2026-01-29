use std::{collections::hash_map::Entry::Vacant, fs};

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pointer_register: usize,
    registers: &mut [usize],
) {
    let mut seen = FxHashMap::default();
    let mut pointer = 0;
    let mut steps = 0u64;
    while pointer < program.len() {
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

        if pointer == program.len() - 3 {
            assert_eq!(program[pointer].0, Opcode::Eqrr);
            if let Vacant(e) = seen.entry(registers[program[pointer].1]) {
                e.insert(steps);
            } else {
                // part 1
                println!("{}", seen.iter().min_by_key(|s| s.1).unwrap().0);

                // part2
                println!("{}", seen.iter().max_by_key(|s| s.1).unwrap().0);

                break;
            }
        }
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

    // Brute-force solution. Takes about 12 seconds on my computer for my input
    if cfg!(feature = "brute-force") {
        let mut registers = vec![0; 6];
        registers[0] = 0;
        run(&program, pointer_register, &mut registers);
    } else {
        // program translated to Rust and simplified/optimized
        let seed = program[7].1;

        let mut seen = FxHashSet::default();
        let mut first_number = None;
        let mut last_number = 0;
        let mut b = 65536;
        let mut c = seed;
        loop {
            let a = b & 255;
            c += a;
            c &= 16777215;
            c *= 65899;
            c &= 16777215;
            if 256 > b {
                if seen.contains(&c) {
                    break;
                }
                if first_number.is_none() {
                    first_number = Some(c);
                }
                last_number = c;
                seen.insert(c);
                b = c | 65536;
                c = seed;
            } else {
                b /= 256;
            }
        }

        // part 1
        println!("{}", first_number.unwrap());

        // part 2
        println!("{last_number}");
    }
}
