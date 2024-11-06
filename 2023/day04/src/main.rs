use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut lines = input.lines().map(|l| (1, l)).collect::<Vec<_>>();

    let mut total_score = 0;
    let mut total_cards = 0;
    for i in 0..lines.len() {
        let (n, l) = lines[i];
        total_cards += n;

        let (_, numbers) = l.split_once(": ").unwrap();
        let (winning_numbers, my_numbers) = numbers.split_once(" | ").unwrap();
        let winning_numbers = winning_numbers.split_whitespace().collect::<Vec<_>>();
        let matches = my_numbers
            .split_whitespace()
            .filter(|n| winning_numbers.contains(n))
            .count();

        total_score += 1 << (matches - 1);

        for j in i + 1..(lines.len().min(i + 1 + matches)) {
            lines[j].0 += n;
        }
    }

    // part 1
    println!("{}", total_score);

    // part 2
    println!("{}", total_cards);
}
