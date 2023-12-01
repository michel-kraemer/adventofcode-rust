use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let first_digit = line.chars().find(|&c| c.is_digit(10))
            .expect("No digit found").to_digit(10).unwrap();
        let last_digit = line.chars().rev().find(|&c| c.is_digit(10))
            .expect("No digit found").to_digit(10).unwrap();
        let number = first_digit * 10 + last_digit;
        sum += number;
    }
    println!("Result: {}", sum);
}
