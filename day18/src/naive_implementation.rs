// This is a naive implementation that only works for part A. For part B,
// it requires too much memory. I didn't want to throw it away as it was very
// helpful for debugging and can be used to draw the whole grid.

use std::fs;

#[allow(dead_code)]
fn naive() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(" ").collect())
        .collect();

    let mut x = 300;
    let mut y = 300;
    let mut grid = vec![vec![' '; 600]; 600];
    let mut prev_dir = "U";

    for line in &lines {
        let dir = line[0];
        let dist = line[1].parse::<i64>().unwrap();

        for _ in 0..dist {
            match dir {
                "R" => {
                    match prev_dir {
                        "R" => grid[y][x] = '-',
                        "L" => grid[y][x] = '-',
                        "D" => grid[y][x] = 'L',
                        "U" => grid[y][x] = 'F',
                        _ => panic!(),
                    }
                    x += 1;
                }
                "L" => {
                    match prev_dir {
                        "R" => grid[y][x] = '-',
                        "L" => grid[y][x] = '-',
                        "D" => grid[y][x] = 'J',
                        "U" => grid[y][x] = '7',
                        _ => panic!(),
                    }
                    x -= 1;
                }
                "D" => {
                    match prev_dir {
                        "R" => grid[y][x] = '7',
                        "L" => grid[y][x] = 'F',
                        "D" => grid[y][x] = '|',
                        "U" => grid[y][x] = '|',
                        _ => panic!(),
                    }
                    y += 1;
                }
                "U" => {
                    match prev_dir {
                        "R" => grid[y][x] = 'J',
                        "L" => grid[y][x] = 'L',
                        "D" => grid[y][x] = '|',
                        "U" => grid[y][x] = '|',
                        _ => panic!(),
                    }
                    y -= 1;
                }
                _ => {}
            }
            prev_dir = dir;
        }
    }

    println!("{}", prev_dir);

    let mut area = 0;
    for (_, row) in grid.iter().enumerate() {
        let mut inside = false;
        let mut x = 0;
        let mut row_area = 0;
        let mut only_pipe = true;
        while x < row.len() {
            let s = row[x];
            match s {
                '|' => {
                    inside = !inside;
                    row_area += 1;
                }
                'F' => {
                    only_pipe = false;
                    row_area += 1;
                    while !(row[x] == 'J' || row[x] == '7') {
                        x += 1;
                        row_area += 1;
                    }
                    if row[x] == 'J' {
                        inside = !inside;
                    }
                }
                'L' => {
                    only_pipe = false;
                    row_area += 1;
                    while !(row[x] == 'J' || row[x] == '7') {
                        x += 1;
                        row_area += 1;
                    }
                    if row[x] == '7' {
                        inside = !inside;
                    }
                }
                _ => {
                    if inside {
                        row_area += 1;
                    }
                }
            }
            x += 1;
        }
        if !only_pipe {
            println!("{row_area}")
        }
        area += row_area;
    }

    println!("{area}");
}
