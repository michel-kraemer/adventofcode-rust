use std::collections::VecDeque;

use rustc_hash::FxHashSet;
use scarlet::{
    color::{Color, RGBColor},
    colormap::{ColorMap, ListedColorMap},
    colors::CIELABColor,
};
use screen::Screen;

use crate::State;

pub fn visualize(
    grid_with_sizes: &[Vec<(usize, usize)>],
    grid: &[Vec<bool>],
    empty: (usize, usize, usize, usize),
    max_x: usize,
) {
    // initialize screen
    let width = grid_with_sizes[0].len();
    let height = grid_with_sizes.len();
    let mut screen = Screen::new(width, height, 20);

    // get minimum and maximum disk usage, but exclude empty node as well as the
    // nodes whose data cannot be moved
    let min_size = grid_with_sizes
        .iter()
        .flatten()
        .filter_map(|s| if s.0 > 0 { Some(s.0) } else { None })
        .min()
        .unwrap();
    let mut max_size = 0;
    for (y, row) in grid_with_sizes.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if grid[y][x] {
                max_size = max_size.max(cell.0);
            }
        }
    }

    // create colors for each node
    let mut colors: Vec<Vec<(u8, u8, u8)>> = vec![vec![(0, 0, 0); width]; height];
    let color_map = ListedColorMap::turbo();
    for (y, row) in grid_with_sizes.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let ratio = (((cell.0 - min_size) as f64 / (max_size - min_size) as f64) * 0.75
                + 0.125)
                .min(1.0);
            let rgb: RGBColor = color_map.transform_single(ratio);
            colors[y][x] = rgb.int_rgb_tup();
        }
    }

    let mut luminance_factors: Vec<Vec<f64>> = vec![vec![1.0; width]; height];

    // perform BFS, similar to the main solution but visualize each step and
    // record shortest path
    let mut queue = VecDeque::new();
    let mut seen = FxHashSet::default();
    let initial = State {
        empty_x: empty.0 as i32,
        empty_y: empty.1 as i32,
        goal_x: max_x as i32,
        goal_y: 0,
    };
    queue.push_back((0, initial, vec![initial]));
    seen.insert(initial);

    let mut prev_steps = -1;
    let mut shortest_path = Vec::new();

    while !queue.is_empty() {
        if queue.front().unwrap().0 != prev_steps {
            // whenever we find a new frontier in the queue ...

            // reset luminance factors
            luminance_factors
                .iter_mut()
                .flatten()
                .for_each(|lf| *lf = (*lf + 0.10).min(1.0));

            // darken luminance of current frontier
            for q in &queue {
                luminance_factors[q.1.empty_y as usize][q.1.empty_x as usize] = 0.10;
            }

            update_screen(
                initial,
                &colors,
                &luminance_factors,
                &mut screen,
                width,
                height,
            );

            prev_steps = queue.front().unwrap().0;
        }

        let (steps, s, path) = queue.pop_front().unwrap();

        if s.goal_x == 0 && s.goal_y == 0 {
            shortest_path = path;
            break;
        }

        for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = s.empty_x + dir.0;
            let ny = s.empty_y + dir.1;
            if nx >= 0
                && ny >= 0
                && nx < grid[0].len() as i32
                && ny < grid.len() as i32
                && grid[ny as usize][nx as usize]
            {
                let mut ngx = s.goal_x;
                let mut ngy = s.goal_y;
                if ngx == nx && ngy == ny {
                    ngx = s.empty_x;
                    ngy = s.empty_y;
                }
                let ns = State {
                    empty_x: nx,
                    empty_y: ny,
                    goal_x: ngx,
                    goal_y: ngy,
                };
                if seen.insert(ns) {
                    let mut new_path = path.clone();
                    new_path.push(ns);
                    queue.push_back((steps + 1, ns, new_path));
                }
            }
        }
    }

    // reset luminance, but slowly (i.e. fade in)
    while luminance_factors.iter().flatten().any(|lf| *lf < 1.0) {
        luminance_factors
            .iter_mut()
            .flatten()
            .for_each(|lf| *lf = (*lf + 0.10).min(1.0));
        update_screen(
            initial,
            &colors,
            &luminance_factors,
            &mut screen,
            width,
            height,
        );
    }

    // visualize shortest path
    let mut prev_empty_x = empty.0;
    let mut prev_empty_y = empty.1;
    for state in shortest_path {
        // swap colors
        (
            colors[prev_empty_y][prev_empty_x],
            colors[state.empty_y as usize][state.empty_x as usize],
        ) = (
            colors[state.empty_y as usize][state.empty_x as usize],
            colors[prev_empty_y][prev_empty_x],
        );
        prev_empty_x = state.empty_x as usize;
        prev_empty_y = state.empty_y as usize;

        update_screen(
            state,
            &colors,
            &luminance_factors,
            &mut screen,
            width,
            height,
        );
    }
}

fn update_screen(
    state: State,
    colors: &[Vec<(u8, u8, u8)>],
    luminance_factors: &[Vec<f64>],
    screen: &mut Screen,
    width: usize,
    height: usize,
) {
    let mut visualized_grid = vec![('⬮', (0, 0, 0)); width * height];
    for y in 0..height {
        for x in 0..width {
            visualized_grid[y * width + x].0 =
                if x == state.goal_x as usize && y == state.goal_y as usize {
                    '█'
                } else {
                    '⬮'
                };
            if x == state.empty_x as usize && y == state.empty_y as usize {
                visualized_grid[y * width + x].1 = (0, 0, 0);
            } else {
                let mut rgb = RGBColor::from(colors[y][x]);
                let mut cielab: CIELABColor = rgb.convert();
                cielab.l *= luminance_factors[y][x];
                rgb = cielab.convert();
                visualized_grid[y * width + x].1 = rgb.int_rgb_tup();
            }
        }
    }

    screen.update_with_colors(&visualized_grid);
}
