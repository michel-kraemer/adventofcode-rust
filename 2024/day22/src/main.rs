use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let mut total1 = 0;
    let mut bananas: HashMap<u32, i64> = HashMap::new();
    for l in lines {
        let mut n = l.parse::<i64>().unwrap();

        let mut seen = HashSet::new();
        let mut current_sequence = 0u32;
        let mut old_price = n % 10;
        for i in 0..2000 {
            n ^= n << 6;
            n %= 16777216;

            n ^= n >> 5;
            n %= 16777216;

            n ^= n << 11;
            n %= 16777216;

            let price = n % 10;
            let diff = price - old_price;

            current_sequence = (current_sequence << 8) + (diff + 10) as u32;

            if i >= 3 && !seen.contains(&current_sequence) {
                seen.insert(current_sequence);
                *bananas.entry(current_sequence).or_default() += price;
            }

            old_price = price;
        }

        total1 += n;
    }

    let total2 = bananas.values().max().unwrap();

    println!("{}", total1);
    println!("{}", total2)
}
