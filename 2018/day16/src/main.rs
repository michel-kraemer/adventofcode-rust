use std::fs;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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

const ALL_OPCODES: [Opcode; 16] = [
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

fn apply(i: Opcode, instr: &[usize; 4], registers: &mut [usize; 4]) {
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

/// This is much faster than using split_ascii_whitespace() and then parse()
fn parse_number<I>(bytes: &mut I) -> usize
where
    I: Iterator<Item = u8>,
{
    let mut r = 0;
    for b in bytes {
        if !b.is_ascii_digit() {
            break;
        }
        r *= 10;
        r += (b - b'0') as usize;
    }
    r
}

fn parse_registers<I>(bytes: &mut I) -> [usize; 4]
where
    I: Iterator<Item = u8>,
{
    bytes.nth(8); // skip "Before: ["
    let a = parse_number(bytes);
    bytes.next(); // skip space
    let b = parse_number(bytes);
    bytes.next(); // skip space
    let c = parse_number(bytes);
    bytes.next(); // skip space
    let d = parse_number(bytes);
    [a, b, c, d]
}

fn parse_instruction<I>(bytes: &mut I) -> [usize; 4]
where
    I: Iterator<Item = u8>,
{
    let a = parse_number(bytes);
    let b = parse_number(bytes);
    let c = parse_number(bytes);
    let d = parse_number(bytes);
    [a, b, c, d]
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut opcodes = [[true; 16]; 16];

    let mut total1 = 0;
    let mut sl = input.bytes().peekable();
    loop {
        let registers_before = parse_registers(&mut sl);
        sl.next(); // skip end of line
        let instr = parse_instruction(&mut sl);
        let registers_after = parse_registers(&mut sl);

        // skip empty line
        sl.nth(1);

        if let Some(&next) = sl.peek()
            && next == b'\n'
        {
            break;
        }

        let mut matches = 0;
        let mut result = registers_before;
        for &i in &ALL_OPCODES {
            apply(i, &instr, &mut result);
            let out = instr[3];
            if result[out] == registers_after[out] {
                matches += 1;
            } else {
                opcodes[instr[0]][i as usize] = false;
            }
            result[out] = registers_before[out];
        }

        if matches >= 3 {
            total1 += 1;
        }
    }

    // part 1
    println!("{total1}");

    // assign an index to each opcode and convert candidates to bit masks
    // it's faster to do it here than already at the beginning
    let mut opcodes = opcodes
        .into_iter()
        .map(|os| {
            let mut mask = 0;
            for (i, o) in os.iter().enumerate() {
                mask |= (*o as u16) << ALL_OPCODES[i] as u32;
            }
            mask
        })
        .enumerate()
        .collect::<Vec<_>>();

    // find opcodes where we know exactly what they are (i.e. that have exactly
    // one candidate)
    let mut good_opcodes_found = 0;
    let mut good_opcodes = [Opcode::Addi; 16];
    for (i, o) in &opcodes {
        if o.count_ones() == 1 {
            good_opcodes[*i] = ALL_OPCODES[o.trailing_zeros() as usize];
            good_opcodes_found |= o;
        }
    }

    opcodes.retain(|(_, o)| o.count_ones() > 1);

    // iteratively remove candidates until all opcodes have been determined
    while !opcodes.is_empty() {
        let mut oi = 0;
        while oi < opcodes.len() {
            let o = &mut opcodes[oi];
            o.1 &= !good_opcodes_found;
            if o.1.count_ones() == 1 {
                good_opcodes[o.0] = ALL_OPCODES[o.1.trailing_zeros() as usize];
                good_opcodes_found |= o.1;
                opcodes.swap_remove(oi);
            } else {
                oi += 1;
            }
        }
    }

    // skip empty line
    sl.nth(1);

    // execute sample program
    let mut registers = [0; 4];
    while sl.peek().is_some() {
        let instr = parse_instruction(&mut sl);
        let opcode = good_opcodes[instr[0]];
        apply(opcode, &instr, &mut registers);
    }

    // part 2
    println!("{}", registers[0]);
}
