use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let lines = input.lines().collect::<Vec<_>>();
    let time = lines[0].split(" ").skip(1).filter(|x| !x.is_empty())
        .collect::<Vec<_>>().join("").parse::<i64>().unwrap();
    let distance = lines[1].split(" ").skip(1).filter(|x| !x.is_empty())
        .collect::<Vec<_>>().join("").parse::<i64>().unwrap();

    let mut n = 0;
    for t in 0..=time {
        if (time - t) * t > distance {
            n += 1;
        }
    }

    println!("{}", n);
}
