use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    fs,
};

/// An event that happens when a step has finished executing
#[derive(Clone, Copy, PartialEq, Eq)]
struct StepFinishedEvent {
    /// The time the step has finished
    time: usize,

    /// The step ID
    step: usize,

    /// The worker on which the step was executed
    worker: usize,
}

impl Ord for StepFinishedEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for StepFinishedEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Tries to schedule as many steps as possible from the wait queue to the given
/// workers. Adds finished events to the event queue
fn schedule(
    wait_queue: &mut BinaryHeap<Reverse<usize>>,
    event_queue: &mut BinaryHeap<StepFinishedEvent>,
    workers: &mut [Option<usize>; 5],
    current_time: usize,
) {
    for (i, w) in workers.iter_mut().enumerate() {
        if w.is_none() {
            let Some(Reverse(next_step)) = wait_queue.pop() else {
                // nothing more to schedule
                return;
            };
            *w = Some(next_step);
            event_queue.push(StepFinishedEvent {
                time: current_time + next_step + 61, // according to problem statement
                step: next_step,
                worker: i,
            });
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut graph: [Vec<usize>; 26] = [const { Vec::new() }; 26];
    let mut incoming: [usize; 26] = [0; 26];

    for l in input.lines() {
        let b = l.as_bytes();
        let step = (b[5] - b'A') as usize;
        let succ = (b[36] - b'A') as usize;
        graph[step].push(succ);
        incoming[succ] += 1;
    }

    // part 1 - simple topological sort
    let mut part1_incoming: [usize; 26] = incoming;

    // find all steps that don't have incoming edges
    let mut start_steps = Vec::new();
    for (n, succ) in graph.iter().enumerate() {
        if !succ.is_empty() && part1_incoming[n] == 0 {
            start_steps.push(n);
        }
    }

    // add steps without incoming edges into a queue (BinaryHeap makes sure
    // they're sorted alphabetically)
    let mut queue = BinaryHeap::new();
    for &n in &start_steps {
        queue.push(Reverse(n));
    }

    let mut sorted = Vec::new();
    while let Some(Reverse(n)) = queue.pop() {
        // remove ready step and add it to result
        sorted.push(n);

        // Remove edge from n to succ. If succ becomes ready, add it to queue.
        for &succ in &graph[n] {
            part1_incoming[succ] -= 1;
            if part1_incoming[succ] == 0 {
                queue.push(Reverse(succ));
            }
        }
    }

    println!(
        "{}",
        sorted
            .iter()
            .map(|&b| (b as u8 + b'A') as char)
            .collect::<String>()
    );

    // part 2 - simulate a simple event-based scheduler with a planning phase
    // and a scheduling phase. Ready steps are added into a wait queue.
    // Available workers take ready steps from the wait queue (see schedule
    // function) and execute them. When a step has finished, an event appears in
    // the event_queue.
    let mut current_time = 0;
    let mut workers = [None; 5];

    // makes sure finished events are processed in order
    let mut event_queue = BinaryHeap::new();

    // makes sure steps are sorted alphabetically
    let mut wait_queue = BinaryHeap::new();

    // add all steps without incoming edges into the wait queue
    for &n in &start_steps {
        wait_queue.push(Reverse(n));
    }

    // schedule first steps
    schedule(&mut wait_queue, &mut event_queue, &mut workers, 0);

    while let Some(e) = event_queue.pop() {
        // A step has finished. Update current time and worker.
        current_time = e.time;
        workers[e.worker] = None;

        // plan phase - add steps that become ready into the wait queue
        for &succ in &graph[e.step] {
            incoming[succ] -= 1;
            if incoming[succ] == 0 {
                wait_queue.push(Reverse(succ));
            }
        }

        // schedule phase - assign steps to workers
        schedule(
            &mut wait_queue,
            &mut event_queue,
            &mut workers,
            current_time,
        );
    }

    println!("{current_time}");
}
