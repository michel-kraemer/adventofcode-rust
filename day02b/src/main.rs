use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let (game, sets) = line.split_once(": ").unwrap();
        let game: usize = game.split_once(" ").unwrap().1.parse().unwrap();
        let sets: Vec<Vec<(usize, &str)>> = sets
            .split("; ")
            .map(|s| {
                s.split(", ")
                    .map(|c| {
                        let p = c.split_once(" ").unwrap();
                        (p.0.parse().unwrap(), p.1)
                    })
                    .collect()
            })
            .collect();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for set in sets {
            for cubes in set {
                let number_of_cubes = cubes.0;
                match cubes.1 {
                    "red" => {
                        if number_of_cubes > min_red {
                            min_red = number_of_cubes;
                        }
                    }
                    "green" => {
                        if number_of_cubes > min_green {
                            min_green = number_of_cubes;
                        }
                    }
                    "blue" => {
                        if number_of_cubes > min_blue {
                            min_blue = number_of_cubes;
                        }
                    }
                    _ => panic!("Unexpected color"),
                }
            }
        }

        let power = min_red * min_green * min_blue;
        sum += power;
    }
    println!("{sum}");
}
