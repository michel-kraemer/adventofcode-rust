use std::{fs::File, io::{BufReader, BufRead}};

use regex::RegexBuilder;

fn convert_digit(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => digit.parse::<u32>().unwrap(),
    }
}

fn main() {
    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    let mut sum = 0;
    let r1 = RegexBuilder::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)")
        .case_insensitive(false).build().unwrap();
    let r2 = RegexBuilder::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)")
        .case_insensitive(false).build().unwrap();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let mut i = r1.find_iter(&line);
        let first_digit = i.next().unwrap().as_str();
        let reverse_line = line.chars().rev().collect::<String>();
        let mut i = r2.find_iter(&reverse_line);
        let last_digit = i.next().unwrap().as_str();
        let first_digit = convert_digit(first_digit);
        let last_digit = convert_digit(last_digit.chars().rev().collect::<String>().as_str());
        let number = first_digit * 10 + last_digit;
        sum += number;
    }
    println!("Result: {}", sum);
}
