use std::{cmp::Ordering, collections::BinaryHeap, fs};

use rustc_hash::FxHashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct State {
    steps: usize,
    elevator: usize,
    floors: [(usize, usize); 4],
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

fn push_state(
    old_state: State,
    to: usize,
    new_floors: [(usize, usize); 4],
    seen: &mut FxHashMap<(usize, [(usize, usize); 4]), usize>,
    queue: &mut BinaryHeap<State>,
) {
    let new_state = State {
        steps: old_state.steps + 1,
        elevator: to,
        floors: new_floors,
    };
    let old = *seen
        .get(&(new_state.elevator, new_state.floors))
        .unwrap_or(&usize::MAX);
    if new_state.steps < old {
        seen.insert((new_state.elevator, new_state.floors), new_state.steps);
        queue.push(new_state);
    }
}

fn try_move(
    to: usize,
    state: State,
    seen: &mut FxHashMap<(usize, [(usize, usize); 4]), usize>,
    queue: &mut BinaryHeap<State>,
) {
    for m in [(2, 0), (1, 0), (1, 1), (0, 1), (0, 2)] {
        if state.floors[state.elevator].0 >= m.0
            && state.floors[state.elevator].1 >= m.1
            && (state.floors[to].0 == 0 || state.floors[to].0 + m.0 >= state.floors[to].1 + m.1)
            && (state.floors[state.elevator].0 - m.0 == 0
                || state.floors[state.elevator].0 - m.0 >= state.floors[state.elevator].1 - m.1)
        {
            let mut new_floors = state.floors;
            new_floors[state.elevator].0 -= m.0;
            new_floors[to].0 += m.0;
            new_floors[state.elevator].1 -= m.1;
            new_floors[to].1 += m.1;
            push_state(state, to, new_floors, seen, queue);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut initial_floors = [(0, 0); 4];
    for (i, l) in input.lines().enumerate() {
        for p in l.split_ascii_whitespace() {
            if p.starts_with("microchip") {
                initial_floors[i].1 += 1;
            } else if p.starts_with("generator") {
                initial_floors[i].0 += 1;
            }
        }
    }

    for part1 in [true, false] {
        if !part1 {
            initial_floors[0].0 += 2;
            initial_floors[0].1 += 2;
        }

        let initial_state = State {
            steps: 0,
            elevator: 0,
            floors: initial_floors,
        };

        let mut queue = BinaryHeap::new();
        queue.push(initial_state);

        let mut seen: FxHashMap<(usize, [(usize, usize); 4]), usize> = FxHashMap::default();
        seen.insert(
            (initial_state.elevator, initial_state.floors),
            initial_state.steps,
        );

        while let Some(state) = queue.pop() {
            if state.floors[0] == (0, 0) && state.floors[1] == (0, 0) && state.floors[2] == (0, 0) {
                println!("{}", state.steps);
                break;
            }

            if state.elevator > 0 {
                try_move(state.elevator - 1, state, &mut seen, &mut queue);
            }
            if state.elevator < 3 {
                try_move(state.elevator + 1, state, &mut seen, &mut queue);
            }
        }
    }
}
