use std::{fs, str::Bytes};

/// This is faster than splitting the lines by whitespace and then using parse()
fn parse_next_number(bytes: &mut Bytes) -> Option<i64> {
    let mut b = bytes.next()?;
    while b != b'-' && !b.is_ascii_digit() {
        b = bytes.next()?;
    }

    let negative = if b == b'-' {
        b = bytes.next()?;
        true
    } else {
        false
    };

    let mut r = 0;
    while b.is_ascii_digit() {
        r *= 10;
        r += (b - b'0') as i64;
        b = bytes.next()?;
    }

    Some(if negative { -r } else { r })
}

/// Get the bounding box of the particles at a given point in time
fn get_bbox(particles: &[(i64, i64, i64, i64)], seconds: i64) -> (i64, i64, i64, i64) {
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    for p in particles {
        let x = p.0 + p.2 * seconds;
        let y = p.1 + p.3 * seconds;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    (min_x, min_y, max_x, max_y)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut bytes = input.bytes();
    let mut particles = Vec::new();
    loop {
        let Some(px) = parse_next_number(&mut bytes) else {
            break;
        };
        let py = parse_next_number(&mut bytes).unwrap();
        let vx = parse_next_number(&mut bytes).unwrap();
        let vy = parse_next_number(&mut bytes).unwrap();
        particles.push((px, py, vx, vy));
    }

    // simulate particle movement until the area covered by the particles
    // reaches a minimum
    let mut seconds = 0;
    let mut min_x;
    let mut min_y;
    let mut max_x;
    let mut max_y;
    loop {
        (min_x, min_y, max_x, max_y) = get_bbox(&particles, seconds);
        let area1 = (max_x - min_x) * (max_y - min_y);
        let (min_x, min_y, max_x, max_y) = get_bbox(&particles, seconds + 1);
        let area2 = (max_x - min_x) * (max_y - min_y);
        if area2 > area1 {
            break;
        }
        // find difference and quickly skip ahead
        seconds += (area1 / (area1 - area2) - 1).max(1);
    }

    // render particles into a grid
    let w = (max_x - min_x + 1) as usize;
    let h = (max_y - min_y + 1) as usize;
    let mut grid = vec![vec![' '; w]; h];
    for p in particles {
        let x = p.0 + p.2 * seconds - min_x;
        let y = p.1 + p.3 * seconds - min_y;
        grid[y as usize][x as usize] = '█';
    }

    // part 1
    for r in grid {
        println!("{}", String::from_iter(r));
    }

    // part 2
    println!("{seconds}");
}
