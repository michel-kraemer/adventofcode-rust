use std::fs;

/// All registers have at most three characters, so we can create a perfect hash
fn index(reg: &str) -> usize {
    assert!(reg.len() <= 3);
    let b = reg.as_bytes();
    let mut result = (b[0] - b'a') as usize;
    if b.len() > 1 {
        result *= 27;
        result += (b[1] - b'a') as usize;
    }
    if b.len() > 2 {
        result *= 27;
        result += (b[2] - b'a') as usize;
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let instructions = input.lines().map(|l| {
        let mut p = l.split_ascii_whitespace();
        (
            p.next().unwrap(),
            p.next().unwrap(),
            p.next().unwrap().parse::<i32>().unwrap(),
            p.nth(1).unwrap(),
            p.next().unwrap(),
            p.next().unwrap().parse::<i32>().unwrap(),
        )
    });

    let mut registers = vec![0; 27 * 27 * 27];

    let mut highest = i32::MIN;
    for i in instructions {
        let bv = registers[index(i.3)];
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
                    let r = &mut registers[index(i.0)];
                    *r += i.2;
                    highest = highest.max(*r);
                }
                "dec" => {
                    let r = &mut registers[index(i.0)];
                    *r -= i.2;
                    highest = highest.max(*r);
                }
                _ => panic!(),
            }
        }
    }

    // part 1
    println!("{}", registers.iter().max().unwrap());

    // part 2
    println!("{highest}");
}
