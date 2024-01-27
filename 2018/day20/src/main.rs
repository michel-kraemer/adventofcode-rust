use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    i: usize,
    x: i32,
    y: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.i.cmp(&other.i)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let regex = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .trim()
        .chars()
        .collect::<Vec<_>>();

    // create a map of rooms and doors between them
    let mut rooms: HashSet<(i32, i32)> = HashSet::new();
    let mut doors: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    let initial = State { i: 0, x: 0, y: 0 };
    seen.insert(initial.clone());
    queue.push_back(initial);
    rooms.insert((0, 0));

    while !queue.is_empty() {
        let mut s = queue.pop_front().unwrap();
        while s.i < regex.len() {
            match regex[s.i] {
                // skip characters that don't give us any information
                '^' | '$' | ')' => {
                    s.i += 1;
                }

                // visit next room north, south, west, or east and add
                // door between current room and next one
                'N' => {
                    rooms.insert((s.x, s.y - 1));
                    doors.entry((s.x, s.y)).or_default().push((s.x, s.y - 1));
                    s.i += 1;
                    s.y -= 1;
                }
                'S' => {
                    rooms.insert((s.x, s.y + 1));
                    doors.entry((s.x, s.y)).or_default().push((s.x, s.y + 1));
                    s.i += 1;
                    s.y += 1;
                }
                'W' => {
                    rooms.insert((s.x - 1, s.y));
                    doors.entry((s.x, s.y)).or_default().push((s.x - 1, s.y));
                    s.i += 1;
                    s.x -= 1;
                }
                'E' => {
                    rooms.insert((s.x + 1, s.y));
                    doors.entry((s.x, s.y)).or_default().push((s.x + 1, s.y));
                    s.i += 1;
                    s.x += 1;
                }

                // this is a junction: look for '|' characters at the same
                // level and insert new starting points into the queue
                '(' => {
                    let mut level = 0;
                    let mut j = s.i;
                    loop {
                        match regex[j] {
                            '(' => level += 1,
                            '|' => {
                                if level == 1 {
                                    let ns = State {
                                        i: j + 1,
                                        x: s.x,
                                        y: s.y,
                                    };
                                    if !seen.contains(&ns) {
                                        seen.insert(ns.clone());
                                        queue.push_back(ns);
                                    }
                                }
                            }
                            ')' => {
                                level -= 1;
                                if level == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                        j += 1;
                    }
                    s.i += 1;
                }

                // skip everything after the '|' character until the end of
                // the junction at the current level
                '|' => {
                    let mut level = 1;
                    let mut j = s.i;
                    loop {
                        match regex[j] {
                            '(' => level += 1,
                            ')' => {
                                level -= 1;
                                if level == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                        j += 1;
                    }
                    s.i = j + 1;
                }

                _ => panic!(),
            }
        }
    }

    // perform BFS and record number of steps for each room seen
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(State { i: 1, x: 0, y: 0 }));

    let mut seen = HashMap::new();
    seen.insert((0, 0), 1);

    while !queue.is_empty() {
        let s = queue.pop().unwrap().0;
        if let Some(next) = doors.get(&(s.x, s.y)) {
            for c in next {
                seen.entry((c.0, c.1)).or_insert_with(|| {
                    queue.push(Reverse(State {
                        i: s.i + 1,
                        x: c.0,
                        y: c.1,
                    }));
                    s.i
                });
            }
        }
    }

    // part 1
    println!("{}", seen.iter().map(|s| s.1).max().unwrap());

    // part 2
    println!("{}", seen.iter().filter(|s| *s.1 >= 1000).count());
}
