use std::{
    collections::{HashMap, HashSet},
    fs,
};

use enum_iterator::Sequence;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Sequence)]
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

fn apply(i: Opcode, instr: &[usize], registers: &mut [usize]) {
    use Opcode::*;

    let a = instr[1];
    let b = instr[2];

    registers[instr[3]] = match i {
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

fn parse_registers(l: &str) -> Vec<usize> {
    l[9..l.len() - 1]
        .split(',')
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_instruction(l: &str) -> Vec<usize> {
    l.split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    // parse
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (samples, program) = input.trim().split_once("\n\n\n\n").unwrap();
    let samples = samples.trim().split("\n\n").collect::<Vec<_>>();

    let all_opcodes = enum_iterator::all::<Opcode>().collect::<Vec<_>>();
    let mut opcodes = vec![all_opcodes.iter().copied().collect::<HashSet<_>>(); 16];

    let mut sum = 0;
    for s in samples {
        let mut sl = s.lines();
        let registers_before = parse_registers(sl.next().unwrap());
        let instr = parse_instruction(sl.next().unwrap());
        let registers_after = parse_registers(sl.next().unwrap());

        let mut matches = 0;
        for &i in &all_opcodes {
            let mut result = registers_before.clone();
            apply(i, &instr, &mut result);
            if result == registers_after {
                matches += 1;
            } else {
                opcodes[instr[0]].remove(&i);
            }
        }

        if matches >= 3 {
            sum += 1;
        }
    }

    // part 1
    println!("{}", sum);

    // assign an index to each opcode
    let mut opcodes = opcodes.into_iter().enumerate().collect::<Vec<_>>();

    // find opcodes where we know exactly what they are (i.e. that have exactly
    // one candidate)
    let mut good_opcodes = opcodes
        .iter()
        .filter_map(|(i, o)| {
            if o.len() == 1 {
                Some((*i, *o.iter().next().unwrap()))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    opcodes.retain(|(i, _)| !good_opcodes.contains_key(i));

    // iteratively remove candidates until all opcodes have been determined
    while !opcodes.is_empty() {
        let mut oi = 0;
        while oi < opcodes.len() {
            let o = &mut opcodes[oi];
            for go in good_opcodes.values() {
                o.1.remove(go);
            }
            if o.1.len() == 1 {
                good_opcodes.insert(o.0, *o.1.iter().next().unwrap());
                opcodes.remove(oi);
            } else {
                oi += 1;
            }
        }
    }

    // execute sample program
    let mut registers = vec![0; 4];
    for l in program.lines() {
        let instr = parse_instruction(l);
        let opcode = good_opcodes[&instr[0]];
        apply(opcode, &instr, &mut registers);
    }

    // part 2
    println!("{}", registers[0]);
}
