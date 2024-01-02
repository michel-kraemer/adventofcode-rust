use std::fs;

use skiplist::SkipList;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let num_elves = input.trim().parse::<usize>().unwrap();

        let mut elves = SkipList::with_capacity(num_elves);
        for i in 0..num_elves {
            elves.push_back(i + 1);
        }

        let mut i = 0;
        while elves.len() > 1 {
            let mut n = if part1 {
                i + 1
            } else {
                (i + elves.len() / 2) % elves.len()
            };
            if n == elves.len() {
                n = 0;
            }

            elves.remove(n);

            if n > i {
                i += 1;
            }
            if i == elves.len() {
                i = 0;
            }
        }

        println!("{}", elves[0]);
    }
}
