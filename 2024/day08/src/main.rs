use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let mut grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let mut positions = [const { Vec::new() }; 128];
    for y in 0..height {
        for x in 0..width {
            let c = grid[y * width + x];
            if c != b'.' {
                positions[c as usize].push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes_part1 = 0;
    let mut antinodes_part2 = 0;
    for antennas in positions {
        for i in 0..antennas.len() {
            for j in 0..antennas.len() {
                if i == j {
                    continue;
                }

                let dx = antennas[j].0 - antennas[i].0;
                let dy = antennas[j].1 - antennas[i].1;
                for n in 1..i32::MAX {
                    let nx = antennas[i].0 + dx * n;
                    let ny = antennas[i].1 + dy * n;

                    if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                        let ni = ny as usize * width + nx as usize;
                        if grid[ni] > 2 {
                            antinodes_part2 += 1;
                            grid[ni] = 2;
                        }
                        if n == 2 && grid[ni] != 1 {
                            antinodes_part1 += 1;
                            grid[ni] = 1;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("{}", antinodes_part1);
    println!("{}", antinodes_part2);
}
