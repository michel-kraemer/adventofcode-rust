use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input
        .lines()
        .map(|l| {
            let mut p = l.split(" ");
            let (turn_on, turn_off, from, to) = if p.next().unwrap() == "turn" {
                let onoff = p.next().unwrap();
                let from = p.next().unwrap();
                let to = p.nth(1).unwrap();
                if onoff == "on" {
                    (true, false, from, to)
                } else {
                    (false, true, from, to)
                }
            } else {
                let from = p.next().unwrap();
                let to = p.nth(1).unwrap();
                (false, false, from, to)
            };

            let to = to
                .split_once(',')
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap();
            let from = from
                .split_once(',')
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap();

            (turn_on, turn_off, from, to)
        })
        .collect::<Vec<_>>();

    for part1 in [true, false] {
        let mut grid = vec![vec![0; 1000]; 1000];

        for &(turn_on, turn_off, from, to) in &input {
            for row in &mut grid[from.1..=to.1] {
                for c in &mut row[from.0..=to.0] {
                    if turn_on {
                        if part1 {
                            *c = 1;
                        } else {
                            *c += 1;
                        }
                    } else if turn_off {
                        if part1 {
                            *c = 0;
                        } else if *c > 0 {
                            *c -= 1;
                        }
                    } else if part1 {
                        if *c > 0 {
                            *c = 0;
                        } else {
                            *c = 1;
                        }
                    } else {
                        *c += 2;
                    }
                }
            }
        }

        println!("{}", grid.iter().flatten().sum::<i32>());
    }
}
