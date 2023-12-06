use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0].split(" ").skip(1).filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let distances = lines[1].split(" ").skip(1).filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let result = (0..times.len()).map(|i| {
        let time = times[i];
        let distance = distances[i];
        let mut n = 0;
        for t in 0..=time {
            if (time - t) * t > distance {
                n += 1;
            }
        }
        n
    }).reduce(|a, b| a * b).unwrap();

    println!("{}", result);
}
