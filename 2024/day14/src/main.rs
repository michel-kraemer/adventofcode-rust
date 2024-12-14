use std::fs;

#[allow(unused)]
fn print_grid(grid: &[u16], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            for i in (0..16).rev() {
                if grid[y * w + x] & 1 << i > 0 {
                    print!("#");
                } else {
                    print!(".");
                }
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
    let bw = (w + 15) / 16;
    let mut binary_grid = vec![0u16; bw * h];

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
        binary_grid[py as usize * bw + px as usize / 16] |= 1 << (15 - px % 16);
    }

    let mut total1 = None;
    let mut total2 = None;
    let mut steps = 0;
    'outer: loop {
        for (px, py, vx, vy) in &mut robots {
            let ni = *py as usize * w + *px as usize;
            grid[ni] -= 1;
            if grid[ni] == 0 {
                binary_grid[*py as usize * bw + *px as usize / 16] &= !(1 << (15 - *px % 16));
            }
            *px = (*px + *vx).rem_euclid(w as i32);
            *py = (*py + *vy).rem_euclid(h as i32);
            grid[*py as usize * w + *px as usize] += 1;
            binary_grid[*py as usize * bw + *px as usize / 16] |= 1 << (15 - *px % 16);
        }

        steps += 1;

        if steps == 100 {
            let t1 = count(&grid, w, 0, w / 2, 0, h / 2);
            let t2 = count(&grid, w, 0, w / 2, (h + 1) / 2, h);
            let t3 = count(&grid, w, (w + 1) / 2, w, 0, h / 2);
            let t4 = count(&grid, w, (w + 1) / 2, w, (h + 1) / 2, h);
            total1 = Some(t1 * t2 * t3 * t4);
            if total2.is_some() {
                break;
            }
        }

        // look for at least 16 robots in a row. this should be our christmas tree.
        for y in 0..h {
            for x in 0..bw - 1 {
                if binary_grid[y * bw + x].trailing_ones()
                    + binary_grid[y * bw + x + 1].leading_ones()
                    >= 16
                {
                    // use print_grid to check if it's really a christmas tree
                    // print_grid(&binary_grid, bw, h);

                    total2 = Some(steps);
                    if total1.is_some() {
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("{}", total1.unwrap());
    println!("{}", total2.unwrap());
}
