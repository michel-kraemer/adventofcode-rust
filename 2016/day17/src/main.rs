use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    fs,
};

#[derive(PartialEq, Eq, Debug)]
struct State {
    x: i32,
    y: i32,
    path: String,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.path.len().cmp(&other.path.len())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_open(b: u8) -> bool {
    !b.is_ascii_digit() && b != b'a'
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let passcode = input.trim();

        let mut queue = BinaryHeap::new();
        queue.push(Reverse(State {
            x: 0,
            y: 0,
            path: passcode.to_string(),
        }));

        let mut max = 0;
        while !queue.is_empty() {
            let s = queue.pop().unwrap().0;

            if s.x == 3 && s.y == 3 {
                if part1 {
                    println!("{}", &s.path[passcode.len()..]);
                    break;
                } else {
                    let steps = s.path.len() - passcode.len();
                    if steps > max {
                        max = steps;
                    }
                    continue;
                }
            }

            let hash = format!("{:x}", md5::compute(&s.path));
            let up = is_open(hash.as_bytes()[0]);
            let down = is_open(hash.as_bytes()[1]);
            let left = is_open(hash.as_bytes()[2]);
            let right = is_open(hash.as_bytes()[3]);

            if up && s.y > 0 {
                queue.push(Reverse(State {
                    x: s.x,
                    y: s.y - 1,
                    path: format!("{}U", s.path),
                }));
            }
            if down && s.y < 3 {
                queue.push(Reverse(State {
                    x: s.x,
                    y: s.y + 1,
                    path: format!("{}D", s.path),
                }));
            }
            if left && s.x > 0 {
                queue.push(Reverse(State {
                    x: s.x - 1,
                    y: s.y,
                    path: format!("{}L", s.path),
                }));
            }
            if right && s.x < 3 {
                queue.push(Reverse(State {
                    x: s.x + 1,
                    y: s.y,
                    path: format!("{}R", s.path),
                }));
            }
        }

        if !part1 {
            println!("{}", max);
        }
    }
}
