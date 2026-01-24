use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let max_recipes = input.trim().parse::<usize>().unwrap();
    let mask = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    // part 1
    let mut recipes = Vec::with_capacity(100_000);
    recipes.push(3u8);
    recipes.push(7u8);
    let mut cur1 = 0;
    let mut cur2 = 1;

    loop {
        let r1 = recipes[cur1];
        let r2 = recipes[cur2];
        let sum = r1 + r2;
        if sum >= 10 {
            recipes.push(1);
            recipes.push(sum % 10);
        } else {
            recipes.push(sum);
        }

        cur1 = (cur1 + (r1 as usize) + 1) % recipes.len();
        cur2 = (cur2 + (r2 as usize) + 1) % recipes.len();

        // part 1
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
    }

    // part 2
    recipes.clear();
    recipes.push(3u8);
    recipes.push(7u8);
    cur1 = 0;
    cur2 = 1;

    loop {
        let r1 = recipes[cur1];
        let r2 = recipes[cur2];
        let sum = r1 + r2;
        if sum >= 10 {
            recipes.push(1);
            recipes.push(sum % 10);
        } else {
            recipes.push(sum);
        }

        cur1 = (cur1 + (r1 as usize) + 1) % recipes.len();
        cur2 = (cur2 + (r2 as usize) + 1) % recipes.len();

        if recipes.len() >= mask.len() && recipes[recipes.len() - mask.len()..] == mask {
            println!("{}", recipes.len() - mask.len());
            break;
        }
        if sum >= 10
            && recipes.len() > mask.len()
            && recipes[recipes.len() - mask.len() - 1..recipes.len() - 1] == mask
        {
            println!("{}", recipes.len() - mask.len() - 1);
            break;
        }
    }
}
