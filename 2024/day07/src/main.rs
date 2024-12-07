use std::fs;

fn check(cur: u64, numbers: &[u64], i: usize, part1: bool) -> bool {
    if i == 0 {
        return cur == numbers[0];
    }

    if !part1 && cur > numbers[i] {
        // get number of digits
        let mask = 10u64.pow(numbers[i].checked_ilog10().unwrap_or(0) + 1);
        if (cur - numbers[i]) % mask == 0 && check(cur / mask, numbers, i - 1, part1) {
            // last n digits can be truncated
            return true;
        }
    }

    if cur % numbers[i] == 0 && check(cur / numbers[i], numbers, i - 1, part1) {
        // number is divisible
        return true;
    }

    if cur >= numbers[i] && check(cur - numbers[i], numbers, i - 1, part1) {
        // we can subtract
        return true;
    }

    false
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();

        let mut total = 0;
        for l in lines {
            let (result, numbers) = l.split_once(": ").unwrap();
            let result = result.parse::<u64>().unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|o| o.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            if check(result, &numbers, numbers.len() - 1, part1) {
                total += result;
            }
        }
        println!("{}", total);
    }
}
