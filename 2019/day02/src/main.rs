use std::fs;

fn run(memory: &[usize], noun: usize, verb: usize) -> usize {
    let mut memory = memory.to_owned();

    memory[1] = noun;
    memory[2] = verb;

    let mut i = 0;
    loop {
        let opcode = memory[i];
        if opcode == 99 {
            return memory[0];
        }

        let a = memory[memory[i + 1]];
        let b = memory[memory[i + 2]];
        let c = memory[i + 3];

        memory[c] = if opcode == 1 {
            a + b
        } else if opcode == 2 {
            a * b
        } else {
            panic!("Unknown opcode: {}", opcode)
        };

        i += 4;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let memory = input
        .trim()
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // part 1
    println!("{}", run(&memory, 12, 2));

    // part 2
    for noun in 0..100 {
        for verb in 0..100 {
            let r = run(&memory, noun, verb);
            if r == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }
}
