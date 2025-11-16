use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut particles = input
        .lines()
        .map(|l| {
            let p = l.split(&['<', '>']).collect::<Vec<_>>();
            let (px, py) = p[1].split_once(',').unwrap();
            let (vx, vy) = p[3].split_once(',').unwrap();
            let px = px.trim().parse::<i64>().unwrap();
            let py = py.trim().parse::<i64>().unwrap();
            let vx = vx.trim().parse::<i64>().unwrap();
            let vy = vy.trim().parse::<i64>().unwrap();
            (px, py, vx, vy)
        })
        .collect::<Vec<_>>();

    let mut seconds = 0;
    let mut grid = Vec::new();
    let mut min_area = i64::MAX;
    loop {
        let min_x = particles.iter().map(|p| p.0).min().unwrap();
        let min_y = particles.iter().map(|p| p.1).min().unwrap();
        let max_x = particles.iter().map(|p| p.0).max().unwrap();
        let max_y = particles.iter().map(|p| p.1).max().unwrap();
        let area = (max_x - min_x) * (max_y - min_y);
        if area > min_area {
            break;
        }
        min_area = area;

        if area < 1000 {
            let w = (max_x - min_x + 1) as usize;
            let h = (max_y - min_y + 1) as usize;
            grid = vec![vec![' '; w]; h];
            for p in &particles {
                let x = p.0 - min_x;
                let y = p.1 - min_y;
                grid[y as usize][x as usize] = '█';
            }
        }

        for p in &mut particles {
            p.0 += p.2;
            p.1 += p.3;
        }

        seconds += 1;
    }

    // part 1
    grid.iter()
        .for_each(|r| println!("{}", String::from_iter(r)));

    // part 2
    println!("{}", seconds - 1);
}
