use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut total1 = 0;
    let mut total2 = 0;
    let mut pos = 50;
    for l in input.lines() {
        let i = l[1..].parse::<i64>().unwrap();
        if l.starts_with("L") {
            total2 += i / 100;
            if pos != 0 && i % 100 >= pos {
                total2 += 1;
            }
            pos = (pos - i).rem_euclid(100);
        } else {
            pos += i;
            total2 += pos / 100;
            pos = pos.rem_euclid(100);
        }
        if pos == 0 {
            total1 += 1;
        }
    }

    println!("{total1}");
    println!("{total2}");
}
