use std::fs;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum Input {
    #[default]
    Unknown,
    Value(u64),
    Bot(usize, bool),
}

#[derive(Default)]
struct Bot {
    inputs: [Input; 2],
}

fn ensure<T: Default>(v: &mut Vec<T>, len: usize) {
    if v.len() < len {
        v.resize_with(len, Default::default);
    }
}

fn add_input(bot: &mut Bot, input: Input) {
    if bot.inputs[0] == Input::Unknown {
        bot.inputs[0] = input;
    } else {
        bot.inputs[1] = input;
    }
}

fn eval(input: Input, bots: &mut [Bot]) -> u64 {
    match input {
        Input::Unknown => panic!("Unknown input"),
        Input::Value(v) => v,
        Input::Bot(bot, high) => {
            let v1 = eval(bots[bot].inputs[0], bots);
            bots[bot].inputs[0] = Input::Value(v1);
            let v2 = eval(bots[bot].inputs[1], bots);
            bots[bot].inputs[1] = Input::Value(v2);
            if high {
                v1.max(v2)
            } else {
                v1.min(v2)
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut bots: Vec<Bot> = Vec::new();
    let mut outputs: Vec<Input> = Vec::new();
    for l in input.lines() {
        if l.starts_with("value") {
            let mut parts = l.split_ascii_whitespace();
            let value = parts.nth(1).unwrap().parse::<u64>().unwrap();
            let bot = parts.nth(3).unwrap().parse::<usize>().unwrap();
            ensure(&mut bots, bot + 1);
            add_input(&mut bots[bot], Input::Value(value));
        } else {
            let mut parts = l.split_ascii_whitespace();
            let from = parts.nth(1).unwrap().parse::<usize>().unwrap();
            let low_type = parts.nth(3).unwrap();
            let low = parts.next().unwrap().parse::<usize>().unwrap();
            let high_type = parts.nth(3).unwrap();
            let high = parts.next().unwrap().parse::<usize>().unwrap();
            if low_type == "bot" {
                ensure(&mut bots, low + 1);
                add_input(&mut bots[low], Input::Bot(from, false));
            } else {
                ensure(&mut outputs, low + 1);
                outputs[low] = Input::Bot(from, false);
            }
            if high_type == "bot" {
                ensure(&mut bots, high + 1);
                add_input(&mut bots[high], Input::Bot(from, true));
            } else {
                ensure(&mut outputs, high + 1);
                outputs[high] = Input::Bot(from, true);
            }
        }
    }

    // part 2: compute product of output[0], output[1], and output[2]
    let total2 =
        eval(outputs[0], &mut bots) * eval(outputs[1], &mut bots) * eval(outputs[2], &mut bots);

    // part 1: look for bot that compares 61 and 17
    for i in 0..bots.len() {
        let v1 = eval(bots[i].inputs[0], &mut bots);
        let v2 = eval(bots[i].inputs[1], &mut bots);
        if (v1 == 61 && v2 == 17) || (v1 == 17 && v2 == 61) {
            println!("{i}");
            break;
        }
    }

    println!("{total2}");
}
