use std::fs;

fn calc_max(
    grid: &[i64],
    prev_grid: &mut [i64],
    s: usize,
    w: usize,
) -> (i64, (usize, usize, usize)) {
    let mut max_power = i64::MIN;
    let mut coords = (0, 0, 0);
    for y in 1..=(w - s) {
        for x in 1..=(w - s) {
            let pgi = (y - 1) * w + x - 1;
            let mut power = prev_grid[pgi];
            for r in 0..s {
                power += grid[(y + r - 1) * w + x + s - 2];
            }
            for c in 0..s - 1 {
                power += grid[(y + s - 2) * w + x + c - 1];
            }
            prev_grid[pgi] = power;
            if power > max_power {
                max_power = power;
                coords = (x, y, s);
            }
        }
    }
    (max_power, coords)
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt")
            .expect("Could not read file")
            .trim()
            .parse::<i64>()
            .unwrap();

        const W: usize = 300;

        let mut grid = [0i64; W * W];

        let mut max_power = i64::MIN;
        let mut coords = (0, 0, 0);

        for y in 1..=W {
            for x in 1..=W {
                let rack_id = (x as i64) + 10;
                let power = ((rack_id * (y as i64) + input) * rack_id) / 100 % 10 - 5;
                grid[(y - 1) * W + x - 1] = power;
                if power > max_power {
                    max_power = power;
                    coords = (x, y, 1);
                }
            }
        }

        if part1 {
            let mut prev_grid = grid;
            calc_max(&grid, &mut prev_grid, 2, W);
            let (_, coords) = calc_max(&grid, &mut prev_grid, 3, W);
            println!("{},{}", coords.0, coords.1);
        } else {
            let mut prev_grid = grid;
            for s in 2..=W {
                let (mp, c) = calc_max(&grid, &mut prev_grid, s, W);
                if mp > max_power {
                    max_power = mp;
                    coords = c;
                }
            }
            println!("{},{},{}", coords.0, coords.1, coords.2);
        }
    }
}
