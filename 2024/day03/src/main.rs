use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();

        let r = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
        let mut total = 0;
        let mut act = true;
        for l in lines {
            let matches = r.captures_iter(l);
            for m in matches {
                let instr = m.get(0).unwrap().as_str();
                if instr == "do()" {
                    act = true;
                } else if instr == "don't()" {
                    act = part1;
                } else {
                    let a = m.get(1).unwrap().as_str().parse::<i64>().unwrap();
                    let b = m.get(2).unwrap().as_str().parse::<i64>().unwrap();
                    if act {
                        total += a * b;
                    }
                }
            }
        }
        println!("{}", total);
    }
}
