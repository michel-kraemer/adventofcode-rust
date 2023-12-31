use std::{collections::HashMap, fs};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let instructions = input.lines().collect::<Vec<_>>();

        let mut registers: HashMap<&str, i32> = HashMap::new();

        if !part1 {
            registers.insert("c", 1);
        }

        let mut pointer = 0;
        while pointer < instructions.len() {
            let instr = instructions[pointer];

            let p = instr.split(' ').collect::<Vec<_>>();
            match p[0] {
                "cpy" => {
                    let v = if let Ok(n) = p[1].parse::<i32>() {
                        n
                    } else {
                        *registers.entry(p[1]).or_default()
                    };
                    *registers.entry(p[2]).or_default() = v;
                }
                "inc" => *registers.get_mut(p[1]).unwrap() += 1,
                "dec" => *registers.get_mut(p[1]).unwrap() -= 1,
                "jnz" => {
                    let v = if let Ok(n) = p[1].parse::<i32>() {
                        n
                    } else {
                        *registers.entry(p[1]).or_default()
                    };
                    if v != 0 {
                        let d = p[2].parse::<i32>().unwrap();
                        pointer = (pointer as i32 + d) as usize;
                        continue;
                    }
                }
                _ => unreachable!(),
            }

            pointer += 1;
        }

        println!("{}", registers["a"]);
    }
}
