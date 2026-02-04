use std::{cmp::Ordering, collections::VecDeque, fs};

// Up, Left, Right, Down - sorted in reading order
const DIRS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

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
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Result of [`shortest_path()`]
#[derive(PartialEq, Eq)]
struct ShortestPathResult {
    steps: usize,
    x: i32,
    y: i32,
    start_x: i32,
    start_y: i32,
}

impl Ord for ShortestPathResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps
            .cmp(&other.steps)
            .then(self.y.cmp(&other.y))
            .then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for ShortestPathResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds the shortest paths of the unit with index `ui` to all cells adjacent
/// to all reachable enemies. If there are multiple paths to these neighbors,
/// the function selects the first one in reading order.
fn shortest_path(ui: usize, units: &[Unit], grid: &[Vec<u8>]) -> Option<ShortestPathResult> {
    let u = &units[ui];
    let expected_enemy = match u.tpe {
        UnitType::Elf => b'G',
        UnitType::Goblin => b'E',
    };

    let w = grid[0].len();
    let mut best = vec![(usize::MAX, (i32::MAX, i32::MAX)); w * grid.len()];
    let mut queue = VecDeque::new();
    for (dx, dy) in DIRS {
        let nx = u.x + dx;
        let ny = u.y + dy;
        if nx >= 0
            && ny >= 0
            && (nx as usize) < w
            && (ny as usize) < grid.len()
            && grid[ny as usize][nx as usize] == b'.'
        {
            queue.push_back((1, nx, ny, nx, ny));
            best[ny as usize * w + nx as usize] = (1, (ny, nx));
        }
    }

    let mut lowest_steps = usize::MAX;
    let mut result = None;

    while let Some((steps, x, y, start_x, start_y)) = queue.pop_front() {
        if steps > lowest_steps {
            // we won't find anything better
            break;
        }

        // create states for next steps
        for d in DIRS {
            let nx = x + d.0;
            let ny = y + d.1;
            if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < grid.len() {
                if grid[ny as usize][nx as usize] == expected_enemy {
                    // We are next to an enemy. This is a possible destination cell.
                    let nr = ShortestPathResult {
                        steps,
                        x,
                        y,
                        start_x,
                        start_y,
                    };
                    if result.is_none() || *result.as_ref().unwrap() > nr {
                        result = Some(nr);
                        lowest_steps = steps;
                    }
                    break;
                } else if grid[ny as usize][nx as usize] == b'.'
                    && (best[ny as usize * w + nx as usize].0 > steps + 1
                        || (best[ny as usize * w + nx as usize].0 == steps + 1
                            && best[ny as usize * w + nx as usize].1 > (start_y, start_x)))
                {
                    best[ny as usize * w + nx as usize] = (steps + 1, (start_y, start_x));
                    queue.push_back((steps + 1, nx, ny, start_x, start_y));
                }
            }
        }
    }

    result
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
fn play(grid: &[Vec<u8>], attack_elf: i32, part1: bool) -> Option<i32> {
    let attack_goblin = 3;

    // create a mutable copy of the grid
    let mut grid = grid.to_vec();

    // find all units
    let mut units = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c {
                b'E' => units.push(Unit {
                    x: x as i32,
                    y: y as i32,
                    tpe: UnitType::Elf,
                    points: 200,
                }),
                b'G' => units.push(Unit {
                    x: x as i32,
                    y: y as i32,
                    tpe: UnitType::Goblin,
                    points: 200,
                }),
                _ => {}
            }
        }
    }

    let mut rounds = 0;
    'outer: loop {
        // make sure units take turns in reading order
        units.sort_unstable();

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
                if let Some(path) = shortest_path(ui, &units, &grid) {
                    // update grid (i.e. take step)
                    let sx = path.start_x;
                    let sy = path.start_y;

                    let u = &mut units[ui];
                    grid[u.y as usize][u.x as usize] = b'.';
                    grid[sy as usize][sx as usize] = match u.tpe {
                        UnitType::Elf => b'E',
                        UnitType::Goblin => b'G',
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
                let enemy_to_attack = enemies_to_attack
                    .into_iter()
                    .min_by(|a, b| a.points.cmp(&b.points).then(a.cmp(b)))
                    .unwrap();

                // decide how much damage we can do
                let attack = match units[ui].tpe {
                    UnitType::Elf => attack_elf,
                    UnitType::Goblin => attack_goblin,
                };

                // find index of enemy to attack
                let ei = units.iter().position(|e| e == enemy_to_attack).unwrap();
                let e = &mut units[ei];

                // perform attack
                e.points -= attack;
                if e.points <= 0 {
                    // part 2: abort as soon as an Elf has been killed
                    if !part1 && e.tpe == UnitType::Elf {
                        return None;
                    }

                    // remove the killed enemy
                    grid[e.y as usize][e.x as usize] = b'.';
                    units.remove(ei);
                    earlier_unit_killed = ei < ui;
                }
            }

            if !earlier_unit_killed {
                ui += 1;
            }
        }

        rounds += 1;
    }

    Some(rounds * units.iter().map(|u| u.points).sum::<i32>())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
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
        fn into_grid(self) -> Vec<Vec<u8>>;
    }

    impl IntoGrid for &str {
        fn into_grid(self) -> Vec<Vec<u8>> {
            self.lines()
                .map(|l| l.bytes().collect::<Vec<_>>())
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

    fn test_part2(grid: Vec<Vec<u8>>, expected_attack_elf: i32, expected_outcome: i32) {
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
