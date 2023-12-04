use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split(&[':', '|']).collect();

        let numbers_on_card = parts[1].split(" ").filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let numbers_i_have = parts[2].split(" ").filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        
        let mut points = 0;
        for c in numbers_on_card {
            if numbers_i_have.contains(&c) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        sum += points;
    }

    println!("Sum: {}", sum);
}
