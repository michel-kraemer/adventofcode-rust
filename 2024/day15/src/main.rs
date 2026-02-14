use std::fs;

#[cfg(feature = "visualize")]
use screen::Screen;

fn is_movable_vertical(grid: &[u8], w: usize, b: (usize, usize), y: usize, dy: isize) -> bool {
    let ny = y.checked_add_signed(dy).unwrap();

    // check if this box can be moved or if a wall is in the way
    if grid[ny * w + b.0] == b'#' || grid[ny * w + b.1] == b'#' {
        return false;
    }

    // check if there is a small box in the way, and if so, if it can be moved
    if grid[ny * w + b.0] == b'O' && !is_movable_vertical(grid, w, (b.0, b.0), ny, dy) {
        return false;
    }
    if grid[ny * w + b.1] == b'O' && !is_movable_vertical(grid, w, (b.1, b.1), ny, dy) {
        return false;
    }

    // check if there is a large box directly in the way and if it can be moved
    if grid[ny * w + b.0] == b'[' && !is_movable_vertical(grid, w, (b.0, b.0 + 1), ny, dy) {
        return false;
    }

    // check if there is a large box slightly to the left
    if grid[ny * w + b.0] == b']' && !is_movable_vertical(grid, w, (b.0 - 1, b.0), ny, dy) {
        return false;
    }

    // check if there is a large box slightly to the right
    if grid[ny * w + b.1] == b'[' && !is_movable_vertical(grid, w, (b.1, b.1 + 1), ny, dy) {
        return false;
    }

    true
}

fn move_vertical(grid: &mut [u8], w: usize, b: (usize, usize), y: usize, dy: isize) {
    let ny = y.checked_add_signed(dy).unwrap();

    // if there is a small box in the way, move it first
    if grid[ny * w + b.0] == b'O' {
        move_vertical(grid, w, (b.0, b.0), ny, dy);
    }
    if grid[ny * w + b.1] == b'O' {
        move_vertical(grid, w, (b.1, b.1), ny, dy);
    }

    // if there is a large box directly in the way, move it first
    if grid[ny * w + b.0] == b'[' {
        move_vertical(grid, w, (b.0, b.0 + 1), ny, dy);
    }

    // if there is a large box slightly to the left, move it first
    if grid[ny * w + b.0] == b']' {
        move_vertical(grid, w, (b.0 - 1, b.0), ny, dy);
    }

    // if there is a large box slightly to the right, move it first
    if grid[ny * w + b.1] == b'[' {
        move_vertical(grid, w, (b.1, b.1 + 1), ny, dy);
    }

    // move this box
    grid[ny * w + b.0] = grid[y * w + b.0];
    grid[ny * w + b.1] = grid[y * w + b.1];

    // clear previous position
    grid[y * w + b.0] = b'.';
    grid[y * w + b.1] = b'.';
}

fn move_horizontal(grid: &mut [u8], w: usize, pos: (usize, usize), dx: isize) {
    let sx = pos.1 * w + pos.0.checked_add_signed(dx).unwrap();
    let mut i = sx;
    while grid[i] == b'O' || grid[i] == b'[' || grid[i] == b']' {
        i = i.checked_add_signed(dx).unwrap();
    }
    if i != sx && grid[i] == b'.' {
        if i < sx {
            grid.copy_within(i + 1..=sx + 1, i);
        } else {
            grid.copy_within(sx - 1..i, sx);
        }
    }
}

fn run_instructions(
    mut pos: (usize, usize),
    instructions: Vec<u8>,
    mut grid: Vec<u8>,
    w: usize,
    h: usize,
    #[cfg(feature = "visualize")] screen: &mut Screen,
) -> usize {
    for instr in instructions {
        match instr {
            // move right
            b'>' => {
                move_horizontal(&mut grid, w, pos, 1);
                if grid[pos.1 * w + pos.0 + 1] == b'.' {
                    pos.0 += 1;
                }
            }

            // move left
            b'<' => {
                move_horizontal(&mut grid, w, pos, -1);
                if grid[pos.1 * w + pos.0 - 1] == b'.' {
                    pos.0 -= 1;
                }
            }

            // move up or down
            b'^' | b'v' => {
                let (new_y, dy) = if instr == b'^' {
                    (pos.1 - 1, -1)
                } else {
                    (pos.1 + 1, 1)
                };
                let b = match grid[new_y * w + pos.0] {
                    b'O' => Some((pos.0, pos.0)),
                    b'[' => Some((pos.0, pos.0 + 1)),
                    b']' => Some((pos.0 - 1, pos.0)),
                    _ => None,
                };
                if let Some(b) = b
                    && is_movable_vertical(&grid, w, b, new_y, dy)
                {
                    move_vertical(&mut grid, w, b, new_y, dy);
                }
                if grid[new_y * w + pos.0] == b'.' {
                    pos.1 = new_y;
                }
            }

            _ => panic!("Unknown instruction: {}", instr),
        }

        #[cfg(feature = "visualize")]
        visualize(pos, &grid, screen);
    }

    let mut total = 0;
    for y in 0..h {
        for x in 0..w {
            let c = grid[y * w + x];
            if c == b'O' || c == b'[' {
                total += 100 * y + x;
            }
        }
    }

    total
}

#[cfg(feature = "visualize")]
fn visualize(pos: (usize, usize), grid: &[u8], screen: &mut Screen) {
    let mut new_grid = grid
        .iter()
        .map(|b| match b {
            b'.' => ' ',
            b'#' => '█',
            _ => *b as char,
        })
        .collect::<Vec<_>>();
    new_grid[pos.1 * screen.width() + pos.0] = '@';
    screen.update(new_grid);
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let (grid, instructions) = input.split_once("\n\n").unwrap();

        let grid_lines = grid.lines().collect::<Vec<_>>();
        let mut width = grid_lines[0].len();
        let height = grid_lines.len();
        let mut grid = grid_lines
            .iter()
            .flat_map(|l| l.as_bytes())
            .copied()
            .collect::<Vec<_>>();

        let instructions = instructions
            .lines()
            .flat_map(|l| l.as_bytes())
            .copied()
            .collect::<Vec<_>>();

        if !part1 {
            // make grid twice as wide
            width *= 2;
            let mut wider_grid = vec![b'.'; width * height];

            for (i, c) in grid.into_iter().enumerate() {
                match c {
                    b'#' | b'.' => {
                        wider_grid[i * 2] = c;
                        wider_grid[i * 2 + 1] = c;
                    }
                    b'O' => {
                        wider_grid[i * 2] = b'[';
                        wider_grid[i * 2 + 1] = b']';
                    }
                    b'@' => {
                        wider_grid[i * 2] = b'@';
                        wider_grid[i * 2 + 1] = b'.';
                    }
                    _ => panic!("Unknown character in grid: {}", c),
                }
            }

            grid = wider_grid;
        }

        #[cfg(feature = "visualize")]
        let mut screen = Screen::new(width, height, 200);

        // find robot
        let mut pos = (0, 0);
        'outer: for y in 0..height {
            for x in 0..width {
                if grid[y * width + x] == b'@' {
                    pos = (x, y);
                    break 'outer;
                }
            }
        }
        grid[pos.1 * width + pos.0] = b'.';

        let total = run_instructions(
            pos,
            instructions,
            grid,
            width,
            height,
            #[cfg(feature = "visualize")]
            &mut screen,
        );

        #[cfg(feature = "visualize")]
        drop(screen);

        println!("{total}");
    }
}
