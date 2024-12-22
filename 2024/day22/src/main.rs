use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let mut total1 = 0;
    let mut total2 = 0;
    let mut bananas = vec![0; 1 << 20];
    let mut seen = vec![usize::MAX; 1 << 20];
    let mask = (1 << 20) - 1;

    for (j, l) in lines.iter().enumerate() {
        let mut n = l.parse::<i64>().unwrap();

        let mut current_sequence = 0;
        let mut old_price = n % 10;
        for i in 0..2000 {
            n ^= n << 6;
            n &= 16777215; // same as `% 16777216` but faster

            n ^= n >> 5;
            // n &= 16777215; // unnecessary

            n ^= n << 11;
            n &= 16777215;

            let price = n % 10;
            let diff = price - old_price;

            current_sequence = ((current_sequence << 5) & mask) + (diff + 10) as usize;

            if i >= 3 && seen[current_sequence] != j {
                seen[current_sequence] = j;
                bananas[current_sequence] += price;
                total2 = total2.max(bananas[current_sequence]);
            }

            old_price = price;
        }

        total1 += n;
    }

    println!("{}", total1);
    println!("{}", total2)
}
