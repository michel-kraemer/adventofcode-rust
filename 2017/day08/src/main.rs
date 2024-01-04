use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let instructions = input
        .lines()
        .map(|l| {
            let mut p = l.split_whitespace();
            (
                p.next().unwrap(),
                p.next().unwrap(),
                p.next().unwrap().parse::<i32>().unwrap(),
                p.nth(1).unwrap(),
                p.next().unwrap(),
                p.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut registers: HashMap<&str, i32> = HashMap::new();

    let mut highest = i32::MIN;
    for i in instructions {
        let bv = *registers.get(&i.3).unwrap_or(&0);
        let ok = match i.4 {
            ">" => bv > i.5,
            "<" => bv < i.5,
            ">=" => bv >= i.5,
            "<=" => bv <= i.5,
            "==" => bv == i.5,
            "!=" => bv != i.5,
            _ => panic!(),
        };
        if ok {
            match i.1 {
                "inc" => {
                    let r = registers.entry(i.0).or_default();
                    *r += i.2;
                    highest = highest.max(*r);
                }
                "dec" => {
                    let r = registers.entry(i.0).or_default();
                    *r -= i.2;
                    highest = highest.max(*r);
                }
                _ => panic!(),
            }
        }
    }

    // part 1
    println!("{}", registers.values().max().unwrap());

    // part 2
    println!("{}", highest);
}
