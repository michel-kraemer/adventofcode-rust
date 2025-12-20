use std::{collections::HashMap, fs};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let instructions = input
            .lines()
            .map(|i| {
                let (instr, params) = i.split_once(" ").unwrap();
                let mut params = params.split(", ").collect::<Vec<_>>();
                if params.len() > 1 {
                    params[1] = params[1].strip_prefix("+").unwrap();
                }
                (instr, params)
            })
            .collect::<Vec<_>>();

        let mut registers: HashMap<&str, u64> = HashMap::new();

        if !part1 {
            registers.insert("a", 1);
        }

        let mut pointer = 0usize;
        while pointer < instructions.len() {
            let i = &instructions[pointer];
            match i.0 {
                "hlf" => *registers.entry(i.1[0]).or_default() /= 2,
                "tpl" => *registers.entry(i.1[0]).or_default() *= 3,
                "inc" => *registers.entry(i.1[0]).or_default() += 1,
                "jmp" => pointer = (pointer as i64 + i.1[0].parse::<i64>().unwrap() - 1) as usize,
                "jie" => {
                    let r = registers.get(i.1[0]).unwrap_or(&0);
                    if r.is_multiple_of(2) {
                        pointer = (pointer as i64 + i.1[1].parse::<i64>().unwrap() - 1) as usize
                    }
                }
                "jio" => {
                    let r = registers.get(i.1[0]).unwrap_or(&0);
                    if *r == 1 {
                        pointer = (pointer as i64 + i.1[1].parse::<i64>().unwrap() - 1) as usize
                    }
                }
                _ => panic!("Unknown instruction"),
            }
            pointer += 1;
        }

        println!("{:?}", registers["b"]);
    }
}
