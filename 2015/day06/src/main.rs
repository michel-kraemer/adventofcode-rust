use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut grid = vec![vec![0; 1000]; 1000];

        for l in input.lines() {
            let p = l.split(" ").collect::<Vec<_>>();
            let to = p[p.len() - 1];
            let from = p[p.len() - 3];
            let to = to
                .split_once(',')
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap();
            let from = from
                .split_once(',')
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap();

            for row in &mut grid[from.1..=to.1] {
                for c in &mut row[from.0..=to.0] {
                    if l.starts_with("turn on") {
                        if part1 {
                            *c = 1;
                        } else {
                            *c += 1;
                        }
                    } else if l.starts_with("turn off") {
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
