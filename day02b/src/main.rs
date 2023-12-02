use std::{fs::File, io::{BufReader, BufRead}};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "game.pest"]
struct GameParser;

fn main() {
    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let result = GameParser::parse(Rule::game, &line).unwrap().next().unwrap();
        let inner_result = result.into_inner();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for set in inner_result {
            for cubes in set.into_inner() {
                let mut inner_cubes = cubes.into_inner();
                let number_of_cubes = inner_cubes.next().unwrap().as_str().parse::<u32>().unwrap();
                match inner_cubes.next().unwrap().as_rule() {
                    Rule::red => {
                        if number_of_cubes > min_red {
                            min_red = number_of_cubes;
                        }
                    },
                    Rule::green => {
                        if number_of_cubes > min_green {
                            min_green = number_of_cubes;
                        }
                    },
                    Rule::blue => {
                        if number_of_cubes > min_blue {
                            min_blue = number_of_cubes;
                        }
                    },
                    _ => panic!("Unexpected color"),
                }
            }
        }

        let power = min_red * min_green * min_blue;
        sum += power;
    }
    println!("Result: {}", sum);
}
