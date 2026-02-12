use std::collections::{HashMap, hash_map::Entry};

use screen::{
    Screen,
    style::{Color, StyledContent, Stylize, style},
};

use crate::{Unit, UnitType};

// Up, Left, Right, Down
const DIRS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

// sand colors
const SAND_COLOR: (u8, u8, u8) = (210, 177, 111);
const SAND_DARKER: (u8, u8, u8) = (190, 142, 67);
const SAND_LIGHTER: (u8, u8, u8) = (228, 205, 145);

// rock colors
const ROCK_COLOR: (u8, u8, u8) = (55, 43, 26);
const ROCK_DARKER: (u8, u8, u8) = (47, 37, 22);
const ROCK_DARKEST: (u8, u8, u8) = (43, 34, 20);

// Default elf colors
const ELF_COLOR_1: (u8, u8, u8) = (250, 240, 230);
const ELF_COLOR_2: (u8, u8, u8) = (180, 30, 30);

// Colors when elves are hit
const ELF_COLOR_LIGHTER_1: (u8, u8, u8) = (252, 250, 242);
const ELF_COLOR_LIGHTER_2: (u8, u8, u8) = (217, 37, 37);
const ELF_COLOR_LIGHTEST_1: (u8, u8, u8) = (255, 255, 255);
const ELF_COLOR_LIGHTEST_2: (u8, u8, u8) = (255, 43, 43);

// Default goblin colors
const GOBLIN_COLOR_1: (u8, u8, u8) = (35, 135, 15);
const GOBLIN_COLOR_2: (u8, u8, u8) = (25, 85, 5);

// Colors when goblins are hit
const GOBLIN_COLOR_LIGHTER_1: (u8, u8, u8) = (46, 179, 20);
const GOBLIN_COLOR_LIGHTER_2: (u8, u8, u8) = (34, 115, 7);
const GOBLIN_COLOR_LIGHTEST_1: (u8, u8, u8) = (56, 217, 24);
const GOBLIN_COLOR_LIGHTEST_2: (u8, u8, u8) = (49, 166, 10);

/// Simple pseudo-random generator. See https://en.wikipedia.org/wiki/Xorshift.
struct Xorshift {
    x32: u32,
}

impl Xorshift {
    fn new() -> Self {
        Self { x32: 3141592 }
    }

    fn next(&mut self) -> u32 {
        self.x32 ^= self.x32 << 13;
        self.x32 ^= self.x32 >> 17;
        self.x32 ^= self.x32 << 5;
        self.x32
    }
}

/// The hit state of a unit. When a unit is hit, it enters the `Lightest` state,
/// then the `Lighter` state on the next frame, and then back to normal.
#[derive(Clone, Copy)]
enum UnitHitState {
    Lightest,
    Lighter,
    Normal,
}

pub struct Visualization {
    /// The screen to use for the visualization
    screen: Screen,

    /// The static background grid (rocks + sand)
    static_grid: Vec<StyledContent<char>>,

    /// The total health points of the elves at the beginning of the visualization
    elves_max_points: i32,

    /// The total health points of the goblins at the beginning of the visualization
    goblins_max_points: i32,

    /// The health points of each unit at the last rendered frame
    last_unit_points: HashMap<usize, i32>,

    /// The current hit state of each unit
    unit_hit_states: HashMap<usize, UnitHitState>,
}

impl Visualization {
    pub fn new(grid: &[u8], width: usize, height: usize, units: &[Unit]) -> Self {
        let rock = Color::from(ROCK_COLOR);
        let rock_darker = Color::from(ROCK_DARKER);
        let rock_darkest = Color::from(ROCK_DARKEST);

        let mut rng = Xorshift::new();

        // create static background grid (rocks + sand)
        let mut static_grid = Vec::new();
        for b in grid {
            match b {
                b'#' => {
                    let c = style('█').with(rock);
                    static_grid.push(c);
                    static_grid.push(c);
                }

                _ => {
                    let sand_colors: [Color; 4] = std::array::from_fn(|_| {
                        let r = rng.next() % 800; // normal sand is much more likely
                        if r > 50 {
                            Color::from(SAND_COLOR)
                        } else if r.is_multiple_of(2) {
                            Color::from(SAND_DARKER)
                        } else {
                            Color::from(SAND_LIGHTER)
                        }
                    });
                    static_grid.push(style('▄').with(sand_colors[0]).on(sand_colors[1]));
                    static_grid.push(style('▄').with(sand_colors[2]).on(sand_colors[3]));
                }
            }
        }

        // create color gradient at the borders of rocky areas
        for (old_color, new_color) in [(rock, rock_darker), (rock_darker, rock_darkest)] {
            for y in 0..height {
                for x in 0..width {
                    if grid[y * width + x] != b'#' {
                        continue;
                    }

                    // count how many neighbors have `old_color` or `new_color`
                    let mut neighbor_count = 0;
                    for (dx, dy) in DIRS {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0
                            && ny >= 0
                            && (nx as usize) < width
                            && (ny as usize) < height
                            && (static_grid[(ny as usize * width + nx as usize) * 2]
                                .style()
                                .foreground_color
                                == Some(old_color)
                                || static_grid[(ny as usize * width + nx as usize) * 2]
                                    .style()
                                    .foreground_color
                                    == Some(new_color))
                        {
                            neighbor_count += 1;
                        }
                    }

                    // determine expected number of neighbors
                    let mut max_count = 4;
                    if x == 0 || x == width - 1 {
                        max_count -= 1;
                    }
                    if y == 0 || y == height - 1 {
                        max_count -= 1;
                    }

                    // make current cell darker if neighbors are all `old_color`
                    // or `new_color`
                    if neighbor_count == max_count {
                        static_grid[(y * width + x) * 2]
                            .style_mut()
                            .foreground_color = Some(new_color);
                        static_grid[(y * width + x) * 2 + 1]
                            .style_mut()
                            .foreground_color = Some(new_color);
                    }
                }
            }
        }

        let elves_max_points = units
            .iter()
            .filter(|u| u.tpe == UnitType::Elf)
            .map(|u| u.points)
            .sum();
        let goblins_max_points = units
            .iter()
            .filter(|u| u.tpe == UnitType::Goblin)
            .map(|u| u.points)
            .sum();
        let last_unit_points = units.iter().map(|u| (u.id, u.points)).collect();

        Self {
            screen: Screen::new(width * 2, height, 7),
            static_grid,
            elves_max_points,
            goblins_max_points,
            last_unit_points,
            unit_hit_states: HashMap::new(),
        }
    }

    /// Update the hit state and the last points of the given unit
    fn update_unit_hit_state(&mut self, unit: &Unit) {
        let last_points = self.last_unit_points.entry(unit.id).or_default();
        match self.unit_hit_states.entry(unit.id) {
            Entry::Occupied(mut e) => match e.get() {
                UnitHitState::Lightest => {
                    e.insert(UnitHitState::Lighter);
                }
                UnitHitState::Lighter => {
                    e.insert(UnitHitState::Normal);
                }
                UnitHitState::Normal => {
                    if *last_points != unit.points {
                        e.insert(UnitHitState::Lightest);
                    } else {
                        e.remove_entry();
                    }
                }
            },
            Entry::Vacant(e) => {
                if *last_points != unit.points {
                    e.insert(UnitHitState::Lightest);
                }
            }
        }
        *last_points = unit.points;
    }

    /// Get the two colors of the given unit
    fn get_unit_colors(&self, unit: &Unit) -> (Color, Color) {
        let r = match unit.tpe {
            UnitType::Elf => match self.unit_hit_states.get(&unit.id) {
                Some(UnitHitState::Lightest) => (ELF_COLOR_LIGHTEST_1, ELF_COLOR_LIGHTEST_2),
                Some(UnitHitState::Lighter) => (ELF_COLOR_LIGHTER_1, ELF_COLOR_LIGHTER_2),
                _ => (ELF_COLOR_1, ELF_COLOR_2),
            },
            UnitType::Goblin => match self.unit_hit_states.get(&unit.id) {
                Some(UnitHitState::Lightest) => (GOBLIN_COLOR_LIGHTEST_1, GOBLIN_COLOR_LIGHTEST_2),
                Some(UnitHitState::Lighter) => (GOBLIN_COLOR_LIGHTER_1, GOBLIN_COLOR_LIGHTER_2),
                _ => (GOBLIN_COLOR_1, GOBLIN_COLOR_2),
            },
        };
        (Color::from(r.0), Color::from(r.1))
    }

    /// Update the visualization. This should be called on every frame.
    pub fn update(&mut self, units: &[Unit]) {
        let width = self.screen.width();

        let mut new_grid = vec![style(' '); width * 2 + self.static_grid.len()];

        // Status bar: Elves
        new_grid[0] = style('▟').with(Color::from(ELF_COLOR_1));
        new_grid[1] = style('▛').with(Color::from(ELF_COLOR_2));
        let elves_count = units.iter().filter(|u| u.tpe == UnitType::Elf).count();
        let elves_str = format!("ELVES {elves_count:8>}");
        for (i, c) in elves_str.chars().enumerate() {
            new_grid[3 + i] = style(c).bold().white();
        }
        let elves_digits = (self.elves_max_points.ilog10() + 1) as usize;
        let elves_points = units
            .iter()
            .filter(|u| u.tpe == UnitType::Elf)
            .map(|u| u.points)
            .sum::<i32>();
        let elves_hearts = (elves_points as usize * 10).div_ceil(self.elves_max_points as usize);
        let elves_points = format!("{elves_points:<elves_digits$}");
        let elves_hearts_str = format!("{:·<10}", "♥".repeat(elves_hearts));
        for (i, c) in elves_hearts_str.chars().enumerate() {
            new_grid[width + 3 + i] = style(c).with(Color::from(ELF_COLOR_2));
        }
        for (i, c) in elves_points.chars().enumerate() {
            new_grid[width + 14 + i] = style(c).bold().white();
        }

        // Status bar: Goblins
        new_grid[width - 2] = style('▟').with(Color::from(GOBLIN_COLOR_1));
        new_grid[width - 1] = style('▛').with(Color::from(GOBLIN_COLOR_2));
        let goblins_count = units.iter().filter(|u| u.tpe == UnitType::Goblin).count();
        let goblins_str = format!("{goblins_count:8>} GOBLINS");
        for (i, c) in goblins_str.chars().enumerate() {
            new_grid[width - 3 - goblins_str.len() + i] = style(c).bold().white();
        }
        let goblins_digits = (self.goblins_max_points.ilog10() + 1) as usize;
        let goblins_points = units
            .iter()
            .filter(|u| u.tpe == UnitType::Goblin)
            .map(|u| u.points)
            .sum::<i32>();
        let goblins_hearts =
            (goblins_points as usize * 10).div_ceil(self.goblins_max_points as usize);
        let goblins_points = format!("{goblins_points:>goblins_digits$}");
        let goblins_hearts_str = format!("{:·>10}", "♥".repeat(goblins_hearts));
        for (i, c) in goblins_hearts_str.chars().enumerate() {
            new_grid[width * 2 - 13 + i] = style(c).with(Color::from(GOBLIN_COLOR_1));
        }
        for (i, c) in goblins_points.chars().enumerate() {
            new_grid[width * 2 - 14 - goblins_digits + i] = style(c).bold().white();
        }

        // copy static grid to row 3
        new_grid[width * 2..].copy_from_slice(&self.static_grid);

        // place units
        let sand = Color::from(SAND_COLOR);
        for u in units {
            self.update_unit_hit_state(u);
            let (col1, col2) = self.get_unit_colors(u);
            new_grid[(u.y as usize + 2) * width + u.x as usize * 2] =
                style('▟').with(col1).on(sand);
            new_grid[(u.y as usize + 2) * width + u.x as usize * 2 + 1] =
                style('▛').with(col2).on(sand);
        }

        self.screen.update_with_style(new_grid);
    }
}
