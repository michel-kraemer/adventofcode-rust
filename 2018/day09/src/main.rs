use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let p = input.split_whitespace().collect::<Vec<_>>();
        let players = p[0].parse::<usize>().unwrap();
        let last_points = p[6].parse::<usize>().unwrap();

        let last_points = if part1 {
            last_points
        } else {
            last_points * 100
        };

        let mut left: VecDeque<usize> = VecDeque::with_capacity(last_points);
        let mut right: VecDeque<usize> = VecDeque::with_capacity(last_points);

        let mut marble = 1;
        let mut current_player = 0;
        let mut scores: HashMap<usize, usize> = HashMap::new();
        right.push_front(0);

        while marble <= last_points {
            if marble % 23 == 0 {
                for _ in 0..7 {
                    if left.is_empty() {
                        std::mem::swap(&mut left, &mut right);
                    }
                    right.push_front(left.pop_back().unwrap());
                }

                let s = scores.entry(current_player).or_default();
                *s += marble;
                *s += right.pop_front().unwrap();
            } else {
                if right.is_empty() {
                    std::mem::swap(&mut left, &mut right);
                }
                left.push_back(right.pop_front().unwrap());

                if right.is_empty() {
                    std::mem::swap(&mut left, &mut right);
                }
                left.push_back(right.pop_front().unwrap());

                right.push_front(marble);
            }

            marble += 1;
            current_player = (current_player + 1) % players;
        }

        let max_score = scores.into_iter().map(|s| s.1).max().unwrap();
        println!("{}", max_score);
    }
}
