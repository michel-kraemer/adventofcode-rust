use std::{cmp::Ordering, collections::BinaryHeap, fs};

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct State {
    steps: usize,
    building: Building,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Try to move one or two items up (or down). Add the new state to the given
/// map of `seen` states and the given `queue`.
fn try_move(up: bool, state: State, seen: &mut FxHashSet<Building>, queue: &mut BinaryHeap<State>) {
    // get the elevator's current and next position
    let from = state.building.get_elevator() as usize;
    let to = if up { from + 1 } else { from - 1 };

    // for all possible combinations of (generators, microchips) ...
    for m in [(2, 0), (1, 0), (1, 1), (0, 1), (0, 2)] {
        // check if we have enough items to move ...
        if state.building.get_generators(from) >= m.0
            && state.building.get_microchips(from) >= m.1
            // ... and if the target floor will be valid after the move
            // (a valid state is one where there is either no generator or the
            // number of microchips does not exceed the number of generators)
            && (state.building.get_generators(to) == 0
                || state.building.get_generators(to) + m.0
                    >= state.building.get_microchips(to) + m.1)
            // ... and if the current floor will be valid after the move
            && (state.building.get_generators(from) - m.0 == 0
                || state.building.get_generators(from) - m.0
                    >= state.building.get_microchips(from) - m.1)
        {
            // create new state
            let mut new_building = state.building;
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
                queue.push(State {
                    steps: state.steps + 1,
                    building: new_building,
                });
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

    // perform Dijkstra's for both parts
    for part1 in [true, false] {
        let mut initial_state = State {
            steps: 0,
            building: initial_building,
        };

        if !part1 {
            initial_state.building.inc_generators(0, 2);
            initial_state.building.inc_microchips(0, 2);
        }

        let mut queue = BinaryHeap::new();
        queue.push(initial_state);

        let mut seen: FxHashSet<Building> = FxHashSet::default();
        seen.insert(initial_building);

        while let Some(state) = queue.pop() {
            if state.building.0 & ((1 << 24) - 1) == 0 {
                // Floors 0-2 are empty. All items must be in floor 3.
                println!("{}", state.steps);
                break;
            }

            if state.building.get_elevator() > 0 {
                try_move(false, state, &mut seen, &mut queue);
            }
            if state.building.get_elevator() < 3 {
                try_move(true, state, &mut seen, &mut queue);
            }
        }
    }
}
