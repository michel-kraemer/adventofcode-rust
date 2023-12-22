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
                .split(",")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let from = from
                .split(",")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            for x in from[0]..=to[0] {
                for y in from[1]..=to[1] {
                    if l.starts_with("turn on") {
                        if part1 {
                            grid[y][x] = 1;
                        } else {
                            grid[y][x] += 1;
                        }
                    } else if l.starts_with("turn off") {
                        if part1 {
                            grid[y][x] = 0;
                        } else {
                            if grid[y][x] > 0 {
                                grid[y][x] -= 1;
                            }
                        }
                    } else {
                        if part1 {
                            if grid[y][x] > 0 {
                                grid[y][x] = 0;
                            } else {
                                grid[y][x] = 1;
                            }
                        } else {
                            grid[y][x] += 2;
                        }
                    }
                }
            }
        }

        let mut sum = 0;
        for y in 0..1000 {
            for x in 0..1000 {
                sum += grid[y][x];
            }
        }
        println!("{sum}");
    }
}
