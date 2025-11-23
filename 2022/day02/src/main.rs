use std::fs;

const LOSE: u64 = 0;
const DRAW: u64 = 3;
const WIN: u64 = 6;

const ROCK: u64 = 1;
const PAPER: u64 = 2;
const SCISSORS: u64 = 3;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // part 1
    let mut total = 0;
    for l in &lines {
        let (a, b) = l.split_once(' ').unwrap();
        let score = match (a, b) {
            // rock
            ("A", "X") => ROCK + DRAW,
            ("A", "Y") => PAPER + WIN,
            ("A", "Z") => SCISSORS + LOSE,
            // paper
            ("B", "X") => ROCK + LOSE,
            ("B", "Y") => PAPER + DRAW,
            ("B", "Z") => SCISSORS + WIN,
            // scissors
            ("C", "X") => ROCK + WIN,
            ("C", "Y") => PAPER + LOSE,
            ("C", "Z") => SCISSORS + DRAW,

            (_, _) => unreachable!(),
        };
        total += score;
    }
    println!("{total}");

    // part 2
    let mut total = 0;
    for l in &lines {
        let (a, b) = l.split_once(' ').unwrap();
        let score = match (a, b) {
            // rock
            ("A", "X") => SCISSORS + LOSE,
            ("A", "Y") => ROCK + DRAW,
            ("A", "Z") => PAPER + WIN,
            // paper
            ("B", "X") => ROCK + LOSE,
            ("B", "Y") => PAPER + DRAW,
            ("B", "Z") => SCISSORS + WIN,
            // scissors
            ("C", "X") => PAPER + LOSE,
            ("C", "Y") => SCISSORS + DRAW,
            ("C", "Z") => ROCK + WIN,

            (_, _) => unreachable!(),
        };
        total += score;
    }
    println!("{total}");
}
