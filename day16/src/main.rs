use std::{collections::{HashSet, VecDeque}, fs, mem::swap};

#[derive(Clone, Hash, PartialEq, Eq)]
struct Beam {
    x: i32,
    y: i32,
    dir_x: i32,
    dir_y: i32,
}

fn count_energized(grid: &Vec<Vec<char>>, starting_beam: Beam) -> usize {
    let mut beams = VecDeque::new();
    beams.push_back(starting_beam);

    let mut energized = HashSet::new();
    let mut seen = HashSet::new();

    while !beams.is_empty() {
        let mut beam = beams.pop_front().unwrap();

        energized.insert((beam.x, beam.y));

        if !seen.contains(&beam) {
            seen.insert(beam.clone());

            let c = grid[beam.y as usize][beam.x as usize];
            match c {
                '/' => {
                    swap(&mut beam.dir_x, &mut beam.dir_y);
                    beam.dir_x = -beam.dir_x;
                    beam.dir_y = -beam.dir_y;
                }

                '\\' => {
                    swap(&mut beam.dir_x, &mut beam.dir_y);
                }

                '-' => {
                    if beam.dir_y != 0 {
                        beam.dir_x = -1;
                        beam.dir_y = 0;
                        if beam.x < grid[0].len() as i32 - 1 {
                            beams.push_back(Beam {
                                x: beam.x + 1,
                                y: beam.y,
                                dir_x: 1,
                                dir_y: 0,
                            })
                        }
                    }
                }

                '|' => {
                    if beam.dir_x != 0 {
                        beam.dir_x = 0;
                        beam.dir_y = -1;
                        if beam.y < grid.len() as i32 - 1 {
                            beams.push_back(Beam {
                                x: beam.x,
                                y: beam.y + 1,
                                dir_x: 0,
                                dir_y: 1,
                            })
                        }
                    }
                }

                _ => {}
            }

            beam.x += beam.dir_x;
            beam.y += beam.dir_y;

            if beam.x >= 0
                && beam.y >= 0
                && beam.x < grid[0].len() as i32
                && beam.y < grid.len() as i32
            {
                beams.push_back(beam);
            }
        }
    }

    energized.len()
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut max = 0;

        if part1 {
            max = count_energized(
                &grid,
                Beam {
                    x: 0,
                    y: 0,
                    dir_x: 1,
                    dir_y: 0,
                },
            );
        } else {
            for x in 0..grid[0].len() {
                let count = count_energized(
                    &grid,
                    Beam {
                        x: x as i32,
                        y: 0,
                        dir_x: 0,
                        dir_y: 1,
                    },
                );
                if count > max {
                    max = count;
                }
            }

            for x in 0..grid[0].len() {
                let count = count_energized(
                    &grid,
                    Beam {
                        x: x as i32,
                        y: grid.len() as i32 - 1,
                        dir_x: 0,
                        dir_y: -1,
                    },
                );
                if count > max {
                    max = count;
                }
            }

            for y in 0..grid.len() {
                let count = count_energized(
                    &grid,
                    Beam {
                        x: 0,
                        y: y as i32,
                        dir_x: 1,
                        dir_y: 0,
                    },
                );
                if count > max {
                    max = count;
                }
            }

            for y in 0..grid.len() {
                let count = count_energized(
                    &grid,
                    Beam {
                        x: grid[0].len() as i32 - 1,
                        y: y as i32,
                        dir_x: -1,
                        dir_y: 0,
                    },
                );
                if count > max {
                    max = count;
                }
            }
        }

        println!("{}", max);
    }
}
