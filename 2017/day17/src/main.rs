use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let steps = input.trim().parse::<usize>().unwrap();

    // part 1
    let mut buffer = Vec::new();
    buffer.push(0);

    let mut i = 0;
    for s in 1..=2017 {
        i = (i + steps) % buffer.len();
        buffer.insert(i + 1, s);
        i += 1;
    }
    println!("{}", buffer[(i + 1) % buffer.len()]);

    // part 2
    let mut result = 0;
    let mut i = 0;
    let mut len = 1;
    for s in 1..=50_000_000 {
        i = (i + steps) % len;
        if i == 0 {
            result = s;
        }
        len += 1;
        i += 1;
    }
    println!("{}", result);
}
