use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let layers = input
        .lines()
        .map(|l| {
            let (layer, len) = l.split_once(": ").unwrap();
            (
                layer.parse::<usize>().unwrap(),
                len.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    // part 1
    let mut severity = 0;
    for l in &layers {
        if l.0 % (l.1 * 2 - 2) == 0 {
            severity += l.0 * l.1;
        }
    }
    println!("{severity}");

    // part 2
    let mut delay = 0;
    'outer: loop {
        delay += 1;
        for l in &layers {
            if (delay + l.0) % (l.1 * 2 - 2) == 0 {
                continue 'outer;
            }
        }
        break;
    }
    println!("{delay}");
}
