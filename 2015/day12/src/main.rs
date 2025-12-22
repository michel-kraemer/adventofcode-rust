use std::fs;

use actson::{JsonEvent, JsonParser, feeder::SliceJsonFeeder};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let feeder = SliceJsonFeeder::new(input.as_bytes());
    let mut parser = JsonParser::new(feeder);

    let mut sum1 = vec![0]; // sums for part 1
    let mut sum2 = vec![0]; // sums for part 2 
    let mut has_red = vec![false]; // whether the current object has a value "red"
    let mut parse_object = vec![]; // whether we parse an object or an array

    while let Some(event) = parser.next_event().unwrap() {
        match event {
            JsonEvent::StartObject => {
                sum1.push(0);
                sum2.push(0);
                has_red.push(false);
                parse_object.push(true);
            }

            JsonEvent::EndObject => {
                // part 1
                let v1 = sum1.pop().unwrap();
                *sum1.last_mut().unwrap() += v1;

                // part 2 - only add sum to parent if the current object has no
                // value "red"
                let v2 = sum2.pop().unwrap();
                let hr = has_red.pop().unwrap();
                if !hr {
                    *sum2.last_mut().unwrap() += v2;
                }

                parse_object.pop();
            }

            JsonEvent::StartArray => {
                parse_object.push(false);
            }

            JsonEvent::EndArray => {
                parse_object.pop();
            }

            JsonEvent::ValueString => {
                if *parse_object.last().unwrap() && parser.current_str().unwrap() == "red" {
                    *has_red.last_mut().unwrap() = true;
                }
            }

            JsonEvent::ValueInt => {
                let v = parser.current_int::<i64>().unwrap();
                *sum1.last_mut().unwrap() += v;
                *sum2.last_mut().unwrap() += v;
            }

            _ => {}
        }
    }

    println!("{}", sum1[0]);
    println!("{}", sum2[0]);
}
