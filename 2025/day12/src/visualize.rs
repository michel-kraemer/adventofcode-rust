use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use crate::screen::Screen;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Visualization {
    Compact,
    Simple,
}

fn flip(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut result = grid.to_vec();
    for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            result[grid.len() - y - 1][x] = v;
        }
    }
    result
}

fn rotate(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut result = grid.to_vec();
    for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            result[row.len() - x - 1][y] = v;
        }
    }
    result
}

fn try_place(area: &mut [Vec<u64>], p: &[Vec<u8>], x: usize, y: usize, tile_id: u64) -> bool {
    for ny in 0..p.len() {
        for nx in 0..p[ny].len() {
            if p[ny][nx] == b'#' {
                if area[y + ny][x + nx] > 0 {
                    return false;
                }
                area[y + ny][x + nx] = tile_id;
            }
        }
    }
    true
}

fn dfs(
    area: &mut [Vec<u64>],
    presents: &Vec<HashSet<Vec<Vec<u8>>>>,
    required_tiles: &mut Vec<usize>,
    max: (usize, usize),
    tile_id: &mut u64,
    screen: &mut Screen,
    visualization: Visualization,
) -> bool {
    if required_tiles.iter().all(|f| *f == 0) {
        return true;
    }

    screen.update(area);

    for first in 0..required_tiles.len() {
        if required_tiles[first] == 0 {
            continue;
        }

        required_tiles[first] -= 1;
        *tile_id += 1;

        for y in 0..max.1.min(area.len() - 2) {
            for x in 0..max.0.min(area[0].len() - 2) {
                for p in &presents[first] {
                    let mut new_area = area.to_vec();
                    if try_place(&mut new_area, p, x, y, *tile_id)
                        && dfs(
                            &mut new_area,
                            presents,
                            required_tiles,
                            if visualization == Visualization::Compact {
                                (x + 3, y + 3)
                            } else {
                                (usize::MAX, usize::MAX)
                            },
                            tile_id,
                            screen,
                            visualization,
                        )
                    {
                        return true;
                    }
                }
            }
        }

        required_tiles[first] += 1;
        *tile_id -= 1;
    }

    false
}

pub fn visualize() {
    let selections = &[
        "Insane     (even for areas that are too small, RUNS FOREVER!)",
        "Compact    (only for areas that provide enough space, SLOWER)",
        "Simple     (only for areas that provide enough space, FASTER)",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of visualization you want to see")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    let visualization = if selection == 2 {
        Visualization::Simple
    } else {
        Visualization::Compact
    };

    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let blocks = input.split("\n\n").collect::<Vec<_>>();
    let raw_presents = &blocks[0..blocks.len() - 1];
    let areas = blocks[blocks.len() - 1];

    let mut presents: Vec<HashSet<Vec<Vec<u8>>>> = Vec::new();
    for p in raw_presents {
        let p2 = &p[3..];
        let grid = p2
            .lines()
            .map(|l| l.as_bytes().to_vec())
            .collect::<Vec<_>>();
        let rotated1 = rotate(&grid);
        let rotated2 = rotate(&rotated1);
        let rotated3 = rotate(&rotated2);

        let mut set = HashSet::new();
        for g in [grid, rotated1, rotated2, rotated3] {
            let f = flip(&g);
            set.insert(g);
            set.insert(f);
        }

        presents.push(set);
    }

    let mut num_tiles = Vec::new();
    for p in &presents {
        let first = p.iter().next().unwrap();
        let mut n = 0;
        for row in first {
            for &v in row {
                if v == b'#' {
                    n += 1;
                }
            }
        }
        num_tiles.push(n);
    }

    let mut total = 0;
    for a in areas.lines() {
        let parts = a.split_ascii_whitespace().collect::<Vec<_>>();
        let (width, height) = parts[0].split_once('x').unwrap();
        let width = width.parse::<usize>().unwrap();
        let height = height[0..height.len() - 1].parse::<usize>().unwrap();

        let mut required_presents = parts[1..]
            .iter()
            .map(|p| p.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let ar = width * height;
        let mut req = 0;
        for (i, t) in required_presents.iter().enumerate() {
            req += t * num_tiles[i];
        }

        if req <= ar {
            total += 1;
        }

        if selection == 0 || req <= ar {
            let mut screen = Screen::new(width, height);

            let mut area = vec![vec![0; width]; height];
            let mut tile = 0;
            dfs(
                &mut area,
                &presents,
                &mut required_presents,
                if visualization == Visualization::Compact {
                    (1, 1)
                } else {
                    (usize::MAX, usize::MAX)
                },
                &mut tile,
                &mut screen,
                visualization,
            );

            screen.finish();

            println!();
        }
    }

    println!("{total}");
}
