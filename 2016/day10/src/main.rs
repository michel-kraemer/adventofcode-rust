use std::{
    cmp::max,
    collections::{BTreeSet, VecDeque},
    fs,
};

#[derive(Default, Clone)]
struct Bot {
    inputs: BTreeSet<u64>,
    low: usize,
    low_bot: bool,
    high: usize,
    high_bot: bool,
}

fn ensure<T: Default>(v: &mut Vec<T>, len: usize) {
    v.resize_with(max(v.len(), len), Default::default);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut bots: Vec<Bot> = Vec::new();

    for l in input.lines() {
        if l.starts_with("value ") {
            let p = l.split(' ').collect::<Vec<_>>();
            let v = p[1].parse::<u64>().unwrap();
            let bot = p.last().unwrap().parse::<usize>().unwrap();
            ensure(&mut bots, bot + 1);
            bots[bot].inputs.insert(v);
        } else if l.starts_with("bot") {
            let p = l.split(' ').collect::<Vec<_>>();
            let bot = p[1].parse::<usize>().unwrap();
            ensure(&mut bots, bot + 1);
            bots[bot].low_bot = p[5] == "bot";
            bots[bot].low = p[6].parse::<usize>().unwrap();
            bots[bot].high_bot = p[10] == "bot";
            bots[bot].high = p[11].parse::<usize>().unwrap();
        }
    }

    let mut queue = VecDeque::new();
    bots.iter()
        .enumerate()
        .filter(|(_, b)| b.inputs.len() == 2)
        .for_each(|(i, _)| queue.push_back(i));

    let mut outputs: Vec<u64> = Vec::new();

    while !queue.is_empty() {
        let bid = queue.pop_front().unwrap();
        let (low_bot, blow, low, high_bot, bhigh, high) = {
            let b = &bots[bid];

            let mut iter = b.inputs.iter();
            let low = iter.next().unwrap();
            let high = iter.next().unwrap();

            if *low == 17 && *high == 61 {
                // part 1
                println!("{}", bid);
            }

            (b.low_bot, b.low, *low, b.high_bot, b.high, *high)
        };

        if low_bot {
            ensure(&mut bots, blow + 1);
            bots[blow].inputs.insert(low);
            if bots[blow].inputs.len() == 2 {
                queue.push_back(blow);
            }
        } else {
            ensure(&mut outputs, blow + 1);
            outputs[blow] = low;
        }

        if high_bot {
            ensure(&mut bots, bhigh + 1);
            bots[bhigh].inputs.insert(high);
            if bots[bhigh].inputs.len() == 2 {
                queue.push_back(bhigh);
            }
        } else {
            ensure(&mut outputs, bhigh + 1);
            outputs[bhigh] = high;
        }
    }

    // part 2
    println!("{}", outputs[0] * outputs[1] * outputs[2]);
}
