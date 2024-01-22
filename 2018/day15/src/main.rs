use std::{cmp::Reverse, collections::BinaryHeap, fs};

/// Type of a unit
#[derive(PartialEq, Eq, Copy, Clone)]
enum UnitType {
    Elf,
    Goblin,
}

/// State of a unit
#[derive(PartialEq, Eq)]
struct Unit {
    /// The unit's position
    x: i32,
    y: i32,

    /// The unit's type
    tpe: UnitType,

    /// The unit's remaining hit points
    points: i32,
}

/// Order units in reading order (from top to bottom and left to right)
impl Ord for Unit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Internal state for [`shortest_paths()`]
#[derive(PartialEq, Eq)]
struct ShortestPathState {
    /// The number of steps taken so far
    steps: usize,

    /// The current position
    x: i32,
    y: i32,

    /// The position of the first step taken
    first_step_x: i32,
    first_step_y: i32,
}

/// Sort shortest path states by number of steps as well as position of the
/// first step in reading order. This makes sure, if there are multiple paths
/// to a destination cell starting at different cells, that the shortest path
/// always starts at the first cell in reading order.
impl Ord for ShortestPathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps
            .cmp(&other.steps)
            .then(self.first_step_y.cmp(&other.first_step_y))
            .then(self.first_step_x.cmp(&other.first_step_x))
    }
}

impl PartialOrd for ShortestPathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Result of [`shortest_paths()`]
struct ShortestPathResult {
    steps: usize,
    x: i32,
    y: i32,
    first_step_x: i32,
    first_step_y: i32,
}

/// Finds the shortest paths of the unit with index `ui` to all cells adjacent
/// to all reachable enemies. If there are multiple paths to these neighbors,
/// the function selects the first one in reading order.
fn shortest_paths(ui: usize, units: &[Unit], grid: &[Vec<char>]) -> Vec<ShortestPathResult> {
    let u = &units[ui];

    let w = grid[0].len();
    let mut seen = vec![false; w * grid.len()];
    let mut queue = BinaryHeap::new();
    seen[u.y as usize * w + u.x as usize] = true;
    queue.push(Reverse(ShortestPathState {
        steps: 0,
        x: u.x,
        y: u.y,
        first_step_x: u.x,
        first_step_y: u.y,
    }));

    let mut result = Vec::new();
    while !queue.is_empty() {
        let s = queue.pop().unwrap().0;
        if s.steps > 0 {
            // We have moved. Check if we are next to an enemy.
            if can_attack(s.x, s.y, units[ui].tpe, grid) {
                // We are next to an enemy. This is a possible destination cell.
                result.push(ShortestPathResult {
                    steps: s.steps,
                    x: s.x,
                    y: s.y,
                    first_step_x: s.first_step_x,
                    first_step_y: s.first_step_y,
                });
            }
        }

        // create states for next steps
        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = s.x + d.0;
            let ny = s.y + d.1;
            if !seen[ny as usize * w + nx as usize] {
                seen[ny as usize * w + nx as usize] = true;

                // save position of first step and don't change it later
                let fx = if s.steps == 0 { nx } else { s.first_step_x };
                let fy = if s.steps == 0 { ny } else { s.first_step_y };

                let ns = ShortestPathState {
                    steps: s.steps + 1,
                    x: nx,
                    y: ny,
                    first_step_x: fx,
                    first_step_y: fy,
                };

                match grid[ny as usize][nx as usize] {
                    '.' => queue.push(Reverse(ns)),
                    '#' | 'G' | 'E' => {}
                    _ => panic!(),
                }
            }
        }
    }

    result
}

fn can_attack(ux: i32, uy: i32, utpe: UnitType, grid: &[Vec<char>]) -> bool {
    let expected = match utpe {
        UnitType::Elf => 'G',
        UnitType::Goblin => 'E',
    };
    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nx = ux + d.0;
        let ny = uy + d.1;
        if ny >= 0
            && (ny as usize) < grid.len()
            && nx >= 0
            && (nx as usize) < grid[ny as usize].len()
            && grid[ny as usize][nx as usize] == expected
        {
            return true;
        }
    }
    false
}

/// Iterate through all given units and check if one of them could be attacked
/// by a unit at the given location `ux,uy` and with type `utpe`.
fn find_enemies_to_attack(ux: i32, uy: i32, utpe: UnitType, units: &[Unit]) -> Vec<&Unit> {
    units
        .iter()
        .filter(|e| e.tpe != utpe && (ux - e.x).abs() + (uy - e.y).abs() == 1)
        .collect::<Vec<_>>()
}

/// Play until the battle is over
fn play(grid: &Vec<Vec<char>>, attack_elf: i32, part1: bool) -> Option<i32> {
    let attack_goblin = 3;

    // create a mutable copy of the grid
    let mut grid = (*grid).clone();

    // find all units
    let mut units = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match grid[y][x] {
                'E' => units.push(Unit {
                    x: x as i32,
                    y: y as i32,
                    tpe: UnitType::Elf,
                    points: 200,
                }),
                'G' => units.push(Unit {
                    x: x as i32,
                    y: y as i32,
                    tpe: UnitType::Goblin,
                    points: 200,
                }),
                _ => {}
            }
        }
    }

    // make sure units take turns in reading order
    units.sort_unstable();

    let mut rounds = 0;
    'outer: loop {
        let mut ui = 0;
        while ui < units.len() {
            if units.iter().all(|u| u.tpe == units[0].tpe) {
                // all enemies have been killed
                break 'outer;
            }

            // have we killed a unit with an index less than ui?
            let mut earlier_unit_killed = false;

            // find enemies next to the current unit
            let mut enemies_to_attack =
                find_enemies_to_attack(units[ui].x, units[ui].y, units[ui].tpe, &units);
            if enemies_to_attack.is_empty() {
                // can't attack: move
                let mut paths = shortest_paths(ui, &units, &grid);
                if !paths.is_empty() {
                    // decide which path to take based on the lowest number of
                    // steps and the destination point in reading order
                    paths.sort_unstable_by_key(|p| (p.steps, p.y, p.x));

                    // update grid (i.e. take step)
                    let sx = paths[0].first_step_x;
                    let sy = paths[0].first_step_y;

                    let u = &mut units[ui];
                    grid[u.y as usize][u.x as usize] = '.';
                    grid[sy as usize][sx as usize] = match u.tpe {
                        UnitType::Elf => 'E',
                        UnitType::Goblin => 'G',
                    };
                    u.x = sx;
                    u.y = sy;

                    // we have moved, try to find enemies again
                    enemies_to_attack = find_enemies_to_attack(u.x, u.y, u.tpe, &units);
                }
            }

            if !enemies_to_attack.is_empty() {
                // We can attack. Select the best enemy based on lowest number
                // of hit points and enemy position in reading order.
                enemies_to_attack.sort_unstable_by(|a, b| {
                    a.points
                        .cmp(&b.points)
                        .then(a.y.cmp(&b.y))
                        .then(a.x.cmp(&b.x))
                });

                // decide how much damage we can do
                let attack = match units[ui].tpe {
                    UnitType::Elf => attack_elf,
                    UnitType::Goblin => attack_goblin,
                };

                // find index of enemy to attack
                let ei = units
                    .iter()
                    .position(|e| e == enemies_to_attack[0])
                    .unwrap();
                let e = &mut units[ei];

                // perform attack
                e.points -= attack;
                if e.points <= 0 {
                    // part 2: abort as soon as an Elf has been killed
                    if !part1 && e.tpe == UnitType::Elf {
                        return None;
                    }

                    // remove the killed enemy
                    grid[e.y as usize][e.x as usize] = '.';
                    units.remove(ei);
                    earlier_unit_killed = ei < ui;
                }
            }

            if !earlier_unit_killed {
                ui += 1;
            }
        }

        // make sure units take turns in reading order
        units.sort_unstable();

        rounds += 1;
    }

    Some(rounds * units.iter().map(|u| u.points).sum::<i32>())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    if let Some(outcome) = play(&grid, 3, true) {
        println!("{}", outcome);
    }

    // part 2
    let mut attack_elf = 3;
    loop {
        attack_elf += 1;
        if let Some(outcome) = play(&grid, attack_elf, false) {
            println!("{}", outcome);
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    trait IntoGrid {
        fn into_grid(self) -> Vec<Vec<char>>;
    }

    impl IntoGrid for &str {
        fn into_grid(self) -> Vec<Vec<char>> {
            self.lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        }
    }

    #[test]
    fn sample_combat1_part1() {
        let grid = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

        let outcome = play(&grid.into_grid(), 3, true);
        assert_eq!(Some(27730), outcome);
    }

    #[test]
    fn sample_combat2_part1() {
        let grid = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

        let outcome = play(&grid.into_grid(), 3, true);
        assert_eq!(Some(36334), outcome);
    }

    #[test]
    fn sample_combat3_part1() {
        let grid = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

        let outcome = play(&grid.into_grid(), 3, true);
        assert_eq!(Some(39514), outcome);
    }

    #[test]
    fn sample_combat4_part1() {
        let grid = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

        let outcome = play(&grid.into_grid(), 3, true);
        assert_eq!(Some(27755), outcome);
    }

    #[test]
    fn sample_combat5_part1() {
        let grid = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

        let outcome = play(&grid.into_grid(), 3, true);
        assert_eq!(Some(28944), outcome);
    }

    #[test]
    fn sample_combat6_part1() {
        let grid = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

        let outcome = play(&grid.into_grid(), 3, true);
        assert_eq!(Some(18740), outcome);
    }

    fn test_part2(grid: Vec<Vec<char>>, expected_attack_elf: i32, expected_outcome: i32) {
        let mut attack_elf = 3;
        let outcome;
        loop {
            attack_elf += 1;
            if let Some(o) = play(&grid, attack_elf, false) {
                outcome = o;
                break;
            }
            assert!(attack_elf < expected_attack_elf);
        }
        assert_eq!(expected_attack_elf, attack_elf);
        assert_eq!(expected_outcome, outcome);
    }

    #[test]
    fn sample_combat1_part2() {
        let grid = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

        test_part2(grid.into_grid(), 15, 4988);
    }

    #[test]
    fn sample_combat2_part2() {
        let grid = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

        test_part2(grid.into_grid(), 4, 31284);
    }

    #[test]
    fn sample_combat3_part2() {
        let grid = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

        test_part2(grid.into_grid(), 15, 3478);
    }

    #[test]
    fn sample_combat4_part2() {
        let grid = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

        test_part2(grid.into_grid(), 12, 6474);
    }

    #[test]
    fn sample_combat5_part2() {
        let grid = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

        test_part2(grid.into_grid(), 34, 1140);
    }
}
