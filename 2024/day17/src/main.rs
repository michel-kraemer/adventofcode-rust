use std::fs;

fn run(mut a: u64, mut b: u64, mut c: u64, program: &[u64]) -> Vec<u64> {
    let mut ip = 0;
    let mut output = Vec::new();

    while ip < program.len() {
        let opcode = program[ip];
        let literal = program[ip + 1];

        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => literal, // invalid combo - pass through
        };

        match opcode {
            0 => a >>= combo,
            1 => b ^= literal,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push(combo % 8),
            6 => b = a >> combo,
            7 => c = a >> combo,
            _ => panic!(),
        };

        ip += 2;
    }

    output
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let a = lines[0][12..].parse::<u64>().unwrap();
    let b = lines[1][12..].parse::<u64>().unwrap();
    let c = lines[2][12..].parse::<u64>().unwrap();

    let program = lines[4][9..]
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // part 1 - just run the program
    let part1 = run(a, b, c, &program);
    println!(
        "{}",
        part1
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    // part 2
    // The digits in the output seem to change after a fixed period. Each
    // period is the previous one times 8: [1, 8, 64, 512, ...].

    // Try all factors of each period and compare their output.
    let mut factors = vec![0; program.len()];

    loop {
        let mut init_a = 0;
        for (i, f) in factors.iter().enumerate() {
            init_a += 8u64.pow(i as u32) * f
        }

        let output = run(init_a, b, c, &program);

        if output == program {
            println!("{}", init_a);
            break;
        }

        for i in (0..program.len()).rev() {
            if output.len() < i || output[i] != program[i] {
                // Increase factor at position i and reset all other factors
                // before it. Note that the resetting was not necessary for my
                // input, but it should be more and generic and doesn't harm.
                // It just adds a few more cycles until we find the solution.
                factors[i] += 1;
                for f in factors.iter_mut().take(i) {
                    *f = 0;
                }
                break;
            }
        }
    }
}
