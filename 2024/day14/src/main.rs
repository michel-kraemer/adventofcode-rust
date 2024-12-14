//! I know this solution is not really fast and many people solved today's
//! problem differently, but that's how I did it during the contest and I would
//! like to keep the original code for posterity. :-)

use std::fs;

#[allow(unused)]
fn print_grid(grid: &[usize], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            if grid[y * w + x] == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn count(grid: &[usize], w: usize, sx: usize, ex: usize, sy: usize, ey: usize) -> usize {
    let mut r = 0;
    for y in sy..ey {
        for x in sx..ex {
            r += grid[y * w + x];
        }
    }
    r
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let w = 101;
    let h = 103;
    let mut grid = vec![0; w * h];
    let mut robots = Vec::new();

    for l in lines {
        let (p, v) = l.split_once(' ').unwrap();
        let (px, py) = p[2..]
            .split_once(",")
            .map(|(px, py)| (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap()))
            .unwrap();
        let (vx, vy) = v[2..]
            .split_once(",")
            .map(|(vx, vy)| (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap()))
            .unwrap();
        robots.push((px, py, vx, vy));
        grid[py as usize * w + px as usize] += 1;
    }

    let mut total1 = 0;
    let mut total2 = 0;
    let mut steps = 0;
    'outer: loop {
        for (px, py, vx, vy) in &mut robots {
            grid[*py as usize * w + *px as usize] -= 1;

            *px += *vx;
            if *px >= w as i32 {
                *px -= w as i32;
            }
            if *px < 0 {
                *px += w as i32;
            }

            *py += *vy;
            if *py >= h as i32 {
                *py -= h as i32;
            }
            if *py < 0 {
                *py += h as i32;
            }

            grid[*py as usize * w + *px as usize] += 1;
        }

        steps += 1;

        if steps == 100 {
            let t1 = count(&grid, w, 0, w / 2, 0, h / 2);
            let t2 = count(&grid, w, 0, w / 2, (h + 1) / 2, h);
            let t3 = count(&grid, w, (w + 1) / 2, w, 0, h / 2);
            let t4 = count(&grid, w, (w + 1) / 2, w, (h + 1) / 2, h);
            total1 = t1 * t2 * t3 * t4;
            if total2 > 0 {
                break;
            }
        }

        // look for at least 16 robots in a row. this should be our christmas tree.
        let min_run = 16;
        for y in 0..h {
            for x in 0..w - min_run {
                let mut found = true;
                for i in 0..min_run {
                    if grid[y * w + x + i] == 0 {
                        found = false;
                        break;
                    }
                }
                if found {
                    // use print_grid to check if it's really a christmas tree
                    // print_grid(&grid, w, h);

                    total2 = steps;
                    if total1 > 0 {
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("{}", total1);
    println!("{}", total2);
}
