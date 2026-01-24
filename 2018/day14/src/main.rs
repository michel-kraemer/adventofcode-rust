use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input.trim();

    let max_recipes = input.parse::<usize>().unwrap();
    assert!(
        input.len() <= 15,
        "This solution only works for inputs with less than 16 digits"
    );

    let mut needle = 0;
    for i in input.trim().bytes() {
        needle = (needle << 4) | (i - b'0') as u64;
    }
    let mask = (1 << (input.len() * 4)) - 1;
    let mask_plus_one = mask << 4 | 0b1111;

    let mut recipes = Vec::with_capacity(100_000);
    for part1 in [true, false] {
        recipes.clear();
        recipes.push(3u8);
        recipes.push(7u8);
        let mut cur1 = 0;
        let mut cur2 = 1;
        let mut last = 0;

        loop {
            let r1 = recipes[cur1];
            let r2 = recipes[cur2];
            let sum = r1 + r2;
            if sum >= 10 {
                let r = sum % 10;
                recipes.push(1);
                recipes.push(r);
                last = ((last << 8) | 0b10000 | r as u64) & mask_plus_one;
            } else {
                recipes.push(sum);
                last = ((last << 4) | sum as u64) & mask_plus_one;
            }

            cur1 = (cur1 + (r1 as usize) + 1) % recipes.len();
            cur2 = (cur2 + (r2 as usize) + 1) % recipes.len();

            // part 1
            if part1 {
                if recipes.len() > max_recipes + 10 {
                    println!(
                        "{}",
                        recipes[max_recipes..max_recipes + 10]
                            .iter()
                            .map(|&i| (b'0' + i) as char)
                            .collect::<String>(),
                    );
                    break;
                }
            } else {
                if recipes.len() >= input.len() && last & mask == needle {
                    println!("{}", recipes.len() - input.len());
                    break;
                }
                if sum >= 10 && recipes.len() > input.len() && last >> 4 == needle {
                    println!("{}", recipes.len() - input.len() - 1);
                    break;
                }
            }
        }
    }
}
