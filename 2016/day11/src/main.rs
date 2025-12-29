use std::{collections::VecDeque, fs};

use rustc_hash::FxHashSet;

/// A bit mask consisting of the current elevator position and the number of
/// generators and microchips in each of the 4 floors. Each entry occupies 4
/// bits. The layout is:
///
/// ```text
///
/// 33-36 |  29-32 |  25-28 |  21-24 |  17-20 |  13-16 |  9-12  |  5-8   |  0-4
/// ele   |  gen   |  chip  |  gen   |  chip  |  gen   |  chip  |  gen   |  chip
///       | floor3 | floor3 | floor2 | floor2 | floor1 | floor1 | floor0 | floor0
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Building(u64);

impl Building {
    /// Get the elevator's current position
    fn get_elevator(&self) -> u64 {
        self.0 >> 32
    }

    /// Move the elevator up one floor
    fn inc_elevator(&mut self) {
        self.0 += 1 << 32;
    }

    /// Move the elevator down one floor
    fn dec_elevator(&mut self) {
        self.0 -= 1 << 32;
    }

    /// Get the number of microchips on the given floor
    fn get_microchips(&self, floor: usize) -> u64 {
        (self.0 >> (floor * 8)) & 0b1111
    }

    /// Get the number of generators on the given floor
    fn get_generators(&self, floor: usize) -> u64 {
        (self.0 >> (floor * 8 + 4)) & 0b1111
    }

    /// Increase the number of microchips on the given floor by `n`
    fn inc_microchips(&mut self, floor: usize, n: u64) {
        self.0 += n << (floor * 8);
    }

    /// Increase the number of generators on the given floor by `n`
    fn inc_generators(&mut self, floor: usize, n: u64) {
        self.0 += n << (floor * 8 + 4);
    }

    /// Decrease the number of microchips on the given floor by `n`
    fn dec_microchips(&mut self, floor: usize, n: u64) {
        self.0 -= n << (floor * 8);
    }

    /// Decrease the number of generators on the given floor by `n`
    fn dec_generators(&mut self, floor: usize, n: u64) {
        self.0 -= n << (floor * 8 + 4);
    }
}

/// Try to move one or two items up (or down). Add the new state to the given
/// map of `seen` states and the given `queue`.
fn try_move(
    up: bool,
    steps: usize,
    building: Building,
    seen: &mut FxHashSet<Building>,
    queue: &mut VecDeque<(usize, Building)>,
) {
    // get the elevator's current and next position
    let from = building.get_elevator() as usize;
    let to = if up { from + 1 } else { from - 1 };

    // for all possible combinations of (generators, microchips) ...
    for m in [(2, 0), (1, 0), (1, 1), (0, 1), (0, 2)] {
        // check if we have enough items to move ...
        if building.get_generators(from) >= m.0
            && building.get_microchips(from) >= m.1
            // ... and if the target floor will be valid after the move
            // (a valid state is one where there is either no generator or the
            // number of microchips does not exceed the number of generators)
            && (building.get_generators(to) == 0
                || building.get_generators(to) + m.0
                    >= building.get_microchips(to) + m.1)
            // ... and if the current floor will be valid after the move
            && (building.get_generators(from) - m.0 == 0
                || building.get_generators(from) - m.0
                    >= building.get_microchips(from) - m.1)
        {
            // create new state
            let mut new_building = building;
            if up {
                new_building.inc_elevator();
            } else {
                new_building.dec_elevator();
            }
            new_building.dec_generators(from, m.0);
            new_building.dec_microchips(from, m.1);
            new_building.inc_generators(to, m.0);
            new_building.inc_microchips(to, m.1);

            if seen.insert(new_building) {
                queue.push_back((steps + 1, new_building));
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // Parse current state. IMPORTANT OPTIMIZATION: Instead of differentiating
    // between all chemical elements, we only need to count the number of
    // generators and microchips per floor. This significantly reduces the
    // problem space. For more information, see
    // https://www.reddit.com/r/adventofcode/comments/5hoia9/comment/db1v1ws/
    let mut initial_building = Building(0);
    for (i, l) in input.lines().enumerate() {
        for p in l.split_ascii_whitespace() {
            if p.starts_with("microchip") {
                initial_building.inc_microchips(i, 1);
            } else if p.starts_with("generator") {
                initial_building.inc_generators(i, 1);
            }
        }
    }

    // perform BFS for both parts
    for part1 in [true, false] {
        let mut initial_building = initial_building;
        if !part1 {
            initial_building.inc_generators(0, 2);
            initial_building.inc_microchips(0, 2);
        }

        let mut queue: VecDeque<(usize, Building)> = VecDeque::new();
        queue.push_back((0, initial_building));

        let mut seen: FxHashSet<Building> = FxHashSet::default();
        seen.insert(initial_building);

        while let Some((steps, building)) = queue.pop_front() {
            if building.0 & ((1 << 24) - 1) == 0 {
                // Floors 0-2 are empty. All items must be on floor 3.
                println!("{}", steps);
                break;
            }

            if building.get_elevator() > 0 {
                try_move(false, steps, building, &mut seen, &mut queue);
            }
            if building.get_elevator() < 3 {
                try_move(true, steps, building, &mut seen, &mut queue);
            }
        }
    }
}
