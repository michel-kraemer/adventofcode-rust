use std::{fs::File, io::{BufReader, BufRead}};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "game.pest"]
struct GameParser;

fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let result = GameParser::parse(Rule::game, &line).unwrap().next().unwrap();
        let mut is_possible = true;
        let mut inner_result = result.into_inner();
        let game_id = inner_result.next().unwrap().as_str().parse::<u32>().unwrap();
        for set in inner_result {
            for cubes in set.into_inner() {
                let mut inner_cubes = cubes.into_inner();
                let number_of_cubes = inner_cubes.next().unwrap().as_str().parse::<u32>().unwrap();
                match inner_cubes.next().unwrap().as_rule() {
                    Rule::red => {
                        if number_of_cubes > max_red {
                            is_possible = false;
                        }
                    },
                    Rule::green => {
                        if number_of_cubes > max_green {
                            is_possible = false;
                        }
                    },
                    Rule::blue => {
                        if number_of_cubes > max_blue {
                            is_possible = false;
                        }
                    },
                    _ => panic!("Unexpected color"),
                }

                if !is_possible {
                    break;
                }
            }
            if !is_possible {
                break;
            }
        }
        if is_possible {
            sum += game_id;
        }
    }
    println!("Result: {}", sum);
}
