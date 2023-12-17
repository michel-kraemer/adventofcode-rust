use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let lines = input.lines().collect::<Vec<_>>();
        let (times, distances) = if part1 {
            (
                lines[0]
                    .split(" ")
                    .skip(1)
                    .filter(|x| !x.is_empty())
                    .map(|d| d.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
                lines[1]
                    .split(" ")
                    .skip(1)
                    .filter(|x| !x.is_empty())
                    .map(|d| d.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        } else {
            (
                vec![lines[0]
                    .split(" ")
                    .skip(1)
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>()
                    .join("")
                    .parse::<usize>()
                    .unwrap()],
                vec![lines[1]
                    .split(" ")
                    .skip(1)
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>()
                    .join("")
                    .parse::<usize>()
                    .unwrap()],
            )
        };

        let prod = times
            .into_iter()
            .zip(distances.into_iter())
            .map(|(time, distance)| {
                let mut n = 0;
                for t in 0..=time {
                    if (time - t) * t > distance {
                        n += 1;
                    }
                }
                n
            })
            .reduce(|a, b| a * b)
            .unwrap();

        println!("{}", prod);
    }
}
